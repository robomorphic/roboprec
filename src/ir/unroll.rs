use indexmap::IndexMap;

use crate::{
    analysis::real::Real,
    ir::{
        expr::{Expr, Opr, OprBinary, OprUnary},
        identifier::{IdSize, Identifier, VarType},
        program::{Output, Program, ProgramInput, ProgramOutput},
    },
};

/// This function unrolls the intermediate representation
/// By unrolling, we mean translating the vectors and matrices into scalars
/// We will create new scalars for vectors and matrices, and keep track of them as we unroll the program
/// To keep track, we may need a map or sth
pub fn unroll_ir(program: &Program) -> Program {
    let mut new_inputs: IndexMap<Identifier, ProgramInput> = IndexMap::new();
    let mut new_outputs: IndexMap<Identifier, ProgramOutput> = IndexMap::new();
    let mut new_body: Vec<Expr> = vec![];

    // the map to convert variables
    // TODO: Make this a normal map
    let mut unroll_vector_map: IndexMap<Identifier, Vec<Identifier>> = IndexMap::new();
    let mut unroll_matrix_map: IndexMap<Identifier, Vec<Vec<Identifier>>> = IndexMap::new();

    // handle inputs
    for (id, input) in program.get_inputs() {
        match input {
            ProgramInput::Scalar { .. } => {
                new_inputs.insert(id.clone(), input.clone());
            }
            ProgramInput::Vector { info } => {
                // now, we need to create new identifiers
                let curr_name = id.name();
                // For now, we can just add _{i} to the name and use it
                let size = match id.size {
                    IdSize::Vector { len } => len,
                    _ => panic!("Size should be a vector here!"),
                };
                let mut all_ids = vec![];
                for (i, element) in info.iter().enumerate() {
                    let new_name = format!("{}_{}", curr_name, i);
                    let new_id = Identifier::new_scalar(&new_name);

                    new_inputs.insert(
                        new_id.clone(),
                        ProgramInput::Scalar {
                            info: element.clone(),
                        },
                    );
                    all_ids.push(new_id);
                }
                assert_eq!(info.len(), size, "Vector input size mismatch");
                unroll_vector_map.insert(id.clone(), all_ids);
            }
            ProgramInput::Matrix { info } => {
                let curr_name = id.name();

                let (row_size, col_size) = match id.size {
                    IdSize::Matrix { row_size, col_size } => (row_size, col_size),
                    _ => panic!("Size should be a matrix here!"),
                };
                let mut all_ids: Vec<Vec<Identifier>> = vec![];
                for (i, row) in info.iter().enumerate() {
                    let mut row_ids: Vec<Identifier> = vec![];
                    for (j, element) in row.iter().enumerate() {
                        let new_name = format!("{:?}_{}_{}", curr_name, i, j);
                        let new_id = Identifier::new_scalar(&new_name);

                        new_inputs.insert(
                            new_id.clone(),
                            ProgramInput::Scalar {
                                info: element.clone(),
                            },
                        );
                        row_ids.push(new_id);
                    }
                    all_ids.push(row_ids);
                }
                assert_eq!(info.len(), row_size, "Matrix row size mismatch");
                assert!(
                    info.iter().all(|row| row.len() == col_size),
                    "Matrix column size mismatch"
                );
                unroll_matrix_map.insert(id.clone(), all_ids);
            }
        }
    }

    // now we need to start handling the body
    for expr in program.get_body() {
        match expr {
            Expr::Let { id: result_id, opr } => {
                match opr {
                    Opr::ConstantScalar { .. } => new_body.push(expr.clone()),
                    Opr::ConstantVector { value: values } => {
                        // this can be a bunch of constantscalar's,
                        // and we just need to record the new ids
                        let result_name = &result_id.name;
                        let mut all_ids = vec![];
                        for (i, value) in values.iter().enumerate() {
                            let new_name = format!("{}_{}", result_name, i);
                            let new_id = Identifier::new_scalar(&new_name);
                            all_ids.push(new_id.clone());

                            let new_expr = Expr::Let {
                                id: new_id.clone(),
                                opr: Opr::ConstantScalar {
                                    value: value.clone(),
                                },
                            };
                            new_body.push(new_expr);
                        }

                        unroll_vector_map.insert(result_id.clone(), all_ids);
                    }
                    Opr::ConstantMatrix { value: values } => {
                        let result_name = &result_id.name;
                        let mut all_ids = vec![];
                        for (i, row) in values.iter().enumerate() {
                            let mut row_ids = vec![];
                            for (j, value) in row.iter().enumerate() {
                                let new_name = format!("{}_{}_{}", result_name, i, j);
                                let new_id = Identifier::new_scalar(&new_name);
                                row_ids.push(new_id.clone());

                                let new_expr = Expr::Let {
                                    id: new_id.clone(),
                                    opr: Opr::ConstantScalar {
                                        value: value.clone(),
                                    },
                                };
                                new_body.push(new_expr);
                            }
                            all_ids.push(row_ids);
                        }
                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                    }
                    Opr::ConstructScalar { id } => {
                        // other than registering the new name, should we do anything else?
                        // I wouldn't even add this to the expr, but, new names may help in debugging
                        // in the code generation phase
                        // OprUnary::Assign already does this, so I'll just use that
                        // No need to change names
                        let new_expr = Expr::Let {
                            id: result_id.clone(),
                            opr: Opr::Unary {
                                opr1: id.clone(),
                                opr_type: OprUnary::Assign,
                            },
                        };
                        new_body.push(new_expr);
                    }
                    Opr::ConstructVector { ids } => {
                        // For this we need to create size number of assigns
                        let result_name = &result_id.name;
                        let mut all_ids = vec![];
                        for (i, id) in ids.iter().enumerate() {
                            let new_name = format!("{}_{}", result_name, i);
                            let new_id = Identifier::new_scalar(&new_name);
                            all_ids.push(new_id.clone());

                            let new_expr = Expr::Let {
                                id: new_id.clone(),
                                opr: Opr::Unary {
                                    opr1: id.clone(),
                                    opr_type: OprUnary::Assign,
                                },
                            };
                            new_body.push(new_expr);
                        }
                        unroll_vector_map.insert(result_id.clone(), all_ids);
                    }
                    Opr::ConstructMatrix { ids } => {
                        let result_name = &result_id.name;
                        let mut all_ids = vec![];
                        for (i, row) in ids.iter().enumerate() {
                            let mut row_ids = vec![];
                            for (j, id) in row.iter().enumerate() {
                                let new_name = format!("{}_{}_{}", result_name, i, j);
                                let new_id = Identifier::new_scalar(&new_name);
                                row_ids.push(new_id.clone());

                                let new_expr = Expr::Let {
                                    id: new_id.clone(),
                                    opr: Opr::Unary {
                                        opr1: id.clone(),
                                        opr_type: OprUnary::Assign,
                                    },
                                };
                                new_body.push(new_expr);
                            }
                            all_ids.push(row_ids);
                        }
                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                    }
                    Opr::Unary { opr1, opr_type } => {
                        match opr1.var_type {
                            VarType::Scalar => new_body.push(expr.clone()),
                            VarType::Vector => {
                                let corresponding_ids = unroll_vector_map.get(opr1).unwrap();
                                let result_name = &result_id.name;
                                match opr_type {
                                    OprUnary::Transpose => {
                                        panic!("Transpose opr is not supported for vectors!")
                                    }
                                    OprUnary::Index { index } => {
                                        assert!(index.len() == 1);
                                        // we only need to return one variable,
                                        // meaning that this could be just an assign operation
                                        let index = index[0];

                                        let new_expr = Expr::Let {
                                            id: result_id.clone(),
                                            opr: Opr::Unary {
                                                opr1: corresponding_ids[index].clone(),
                                                opr_type: OprUnary::Assign,
                                            },
                                        };
                                        new_body.push(new_expr);
                                    }
                                    OprUnary::Assign
                                    | OprUnary::Neg
                                    | OprUnary::AssignNoOpt => {
                                        // In this case the result id is also a vector,
                                        // so we need to create new ids for that, too
                                        let mut all_ids = vec![];
                                        for (i, id) in corresponding_ids.iter().enumerate() {
                                            let new_name = format!("{}_{}", result_name, i);
                                            let new_id = Identifier::new_scalar(&new_name);
                                            all_ids.push(new_id.clone());

                                            let new_expr = Expr::Let {
                                                id: new_id,
                                                opr: Opr::Unary {
                                                    opr1: id.clone(),
                                                    opr_type: opr_type.clone(), // assign or neg
                                                },
                                            };
                                            new_body.push(new_expr);
                                        }
                                        unroll_vector_map.insert(result_id.clone(), all_ids);
                                    }
                                }
                            }
                            VarType::Matrix => {
                                let corresponding_ids = unroll_matrix_map.get(opr1).unwrap();
                                let row_size = corresponding_ids.len();
                                let col_size = corresponding_ids[0].len();
                                let result_name = &result_id.name;
                                match opr_type {
                                    OprUnary::Transpose => {
                                        let mut all_ids = vec![];
                                        // create new ids for the result, rest is easy
                                        for (i, row) in corresponding_ids.iter().enumerate() {
                                            let mut row_ids = vec![];
                                            for (j, id) in row.iter().enumerate() {
                                                let new_name =
                                                    format!("{}_{}_{}", result_name, i, j);
                                                let new_id = Identifier::new_scalar(&new_name);
                                                row_ids.push(new_id.clone());

                                                let new_expr = match opr_type {
                                                    OprUnary::Transpose => Expr::Let {
                                                        id: new_id.clone(),
                                                        opr: Opr::Unary {
                                                            opr1: id.clone(),
                                                            opr_type: OprUnary::Assign,
                                                        },
                                                    },
                                                    OprUnary::Assign
                                                    | OprUnary::Neg
                                                    | OprUnary::AssignNoOpt => {
                                                        panic!("You shouldn't be here!")
                                                    }
                                                    OprUnary::Index { .. } => {
                                                        panic!("You shouldn't be here!")
                                                    }
                                                };
                                                new_body.push(new_expr);
                                            }
                                            all_ids.push(row_ids);
                                        }
                                        // need to transpose the ids
                                        let transposed_ids = (0..col_size)
                                            .map(|j| {
                                                (0..row_size)
                                                    .map(|i| all_ids[i][j].clone())
                                                    .collect()
                                            })
                                            .collect();
                                        unroll_matrix_map.insert(result_id.clone(), transposed_ids);
                                    }
                                    OprUnary::Assign
                                    | OprUnary::Neg
                                    | OprUnary::AssignNoOpt => {
                                        let mut all_ids = vec![];
                                        // create new ids for the result, rest is easy
                                        for (i, row) in corresponding_ids.iter().enumerate() {
                                            let mut row_ids = vec![];
                                            for (j, id) in row.iter().enumerate() {
                                                let new_name =
                                                    format!("{}_{}_{}", result_name, i, j);
                                                let new_id = Identifier::new_scalar(&new_name);
                                                row_ids.push(new_id.clone());

                                                let new_expr = match opr_type {
                                                    OprUnary::Assign
                                                    | OprUnary::Neg
                                                    | OprUnary::AssignNoOpt => {
                                                        Expr::Let {
                                                            id: new_id,
                                                            opr: Opr::Unary {
                                                                opr1: id.clone(),
                                                                opr_type: opr_type.clone(), // assign or neg
                                                            },
                                                        }
                                                    }
                                                    OprUnary::Transpose => {
                                                        panic!("You shouldn't be here!")
                                                    }
                                                    OprUnary::Index { .. } => {
                                                        panic!("You shouldn't be here!")
                                                    }
                                                };
                                                new_body.push(new_expr);
                                            }
                                            all_ids.push(row_ids);
                                        }
                                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                                    }
                                    OprUnary::Index { index } => {
                                        assert!(index.len() == 2);
                                        let row_index = index[0];
                                        let col_index = index[1];
                                        // we don't need to create a new id
                                        let new_expr = Expr::Let {
                                            id: result_id.clone(),
                                            opr: Opr::Unary {
                                                opr1: corresponding_ids[row_index][col_index]
                                                    .clone(),
                                                opr_type: OprUnary::Assign,
                                            },
                                        };
                                        new_body.push(new_expr);
                                    }
                                }
                            }
                        }
                    }
                    Opr::Binary {
                        opr1,
                        opr2,
                        opr_type,
                    } => {
                        let result_name = &result_id.name;
                        match opr_type {
                            OprBinary::Cross => {
                                let ids_1 = &unroll_vector_map.get(opr1).unwrap();
                                let ids_2 = &unroll_vector_map.get(opr2).unwrap();
                                // we need 3 new ids for the scalar results
                                let mut ids = vec![];
                                for i in 0..3 {
                                    let new_name = format!("{}_{}", result_name, i);
                                    let new_id = Identifier::new_scalar(&new_name);
                                    ids.push(new_id);
                                }

                                // new_0 = opr1[1]*opr2[2] - opr1[2]*opr2[1]
                                // new_1 = opr1[2]*opr2[0] - opr1[0]*opr2[2]
                                // new_2 = opr1[0]*opr2[1] - opr1[1]*opr2[0]

                                // I guess I need to do it by hand
                                //// First element:
                                let mul1 = Identifier::new_scalar("cross_temp");
                                let new_expr = Expr::Let {
                                    id: mul1.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[1].clone(),
                                        opr2: ids_2[2].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let mul2 = Identifier::new_scalar("cross_temp2");
                                let new_expr = Expr::Let {
                                    id: mul2.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[2].clone(),
                                        opr2: ids_2[1].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let new_expr = Expr::Let {
                                    id: ids[0].clone(),
                                    opr: Opr::Binary {
                                        opr1: mul1,
                                        opr2: mul2,
                                        opr_type: OprBinary::Sub,
                                    },
                                };
                                new_body.push(new_expr);

                                //// Second element:
                                let mul1 = Identifier::new_scalar("cross_temp3");
                                let new_expr = Expr::Let {
                                    id: mul1.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[2].clone(),
                                        opr2: ids_2[0].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let mul2 = Identifier::new_scalar("cross_temp4");
                                let new_expr = Expr::Let {
                                    id: mul2.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[0].clone(),
                                        opr2: ids_2[2].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let new_expr = Expr::Let {
                                    id: ids[1].clone(),
                                    opr: Opr::Binary {
                                        opr1: mul1,
                                        opr2: mul2,
                                        opr_type: OprBinary::Sub,
                                    },
                                };
                                new_body.push(new_expr);

                                //// Third element:
                                let mul1 = Identifier::new_scalar("cross_temp5");
                                let new_expr = Expr::Let {
                                    id: mul1.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[0].clone(),
                                        opr2: ids_2[1].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let mul2 = Identifier::new_scalar("cross_temp6");
                                let new_expr = Expr::Let {
                                    id: mul2.clone(),
                                    opr: Opr::Binary {
                                        opr1: ids_1[1].clone(),
                                        opr2: ids_2[0].clone(),
                                        opr_type: OprBinary::Mul,
                                    },
                                };
                                new_body.push(new_expr);
                                let new_expr = Expr::Let {
                                    id: ids[2].clone(),
                                    opr: Opr::Binary {
                                        opr1: mul1,
                                        opr2: mul2,
                                        opr_type: OprBinary::Sub,
                                    },
                                };
                                new_body.push(new_expr);

                                unroll_vector_map.insert(result_id.clone(), ids);
                            }
                            OprBinary::Dot => {
                                let ids_1 = &unroll_vector_map.get(opr1).unwrap();
                                let ids_2 = &unroll_vector_map.get(opr2).unwrap();
                                assert!(opr1.size == opr2.size);
                                let size = match opr1.size {
                                    IdSize::Vector { len } => len,
                                    _ => panic!("There should be a vector here!"),
                                };
                                // we will sum all multiplications for this
                                let mut mult_ids = vec![];
                                for i in 0..size {
                                    // mult ids
                                    let new_name = format!("dot_{}", i);
                                    let new_id = Identifier::new_scalar(&new_name);

                                    let new_expr = Expr::Let {
                                        id: new_id.clone(),
                                        opr: Opr::Binary {
                                            opr1: ids_1[i].clone(),
                                            opr2: ids_2[i].clone(),
                                            opr_type: OprBinary::Mul,
                                        },
                                    };

                                    mult_ids.push(new_id);
                                    new_body.push(new_expr);
                                }

                                // now we need to sum them up
                                // result will be a scalar anyways
                                // I can do one final assign operation for that
                                let mut curr_sum_id = mult_ids[0].clone();
                                for (i, item) in mult_ids.iter().enumerate().skip(1) {
                                    let new_name = format!("dot_sum_{}", i);
                                    let new_id = Identifier::new_scalar(&new_name);

                                    let new_expr = Expr::Let {
                                        id: new_id.clone(),
                                        opr: Opr::Binary {
                                            opr1: curr_sum_id.clone(),
                                            opr2: item.clone(),
                                            opr_type: OprBinary::Add,
                                        },
                                    };
                                    new_body.push(new_expr);
                                    curr_sum_id = new_id;
                                }

                                // final sum id will be just assigned to the actual result id
                                let new_expr = Expr::Let {
                                    id: result_id.clone(),
                                    opr: Opr::Unary {
                                        opr1: curr_sum_id,
                                        opr_type: OprUnary::Assign,
                                    },
                                };
                                new_body.push(new_expr);
                            }
                            OprBinary::Div | OprBinary::Sub | OprBinary::Add | OprBinary::Mul => {
                                // This one is important because of associativity
                                // we shouldn't do scalar / matrix, or similarly scalar - matrix
                                match (opr1.var_type.clone(), opr2.var_type.clone()) {
                                    (VarType::Scalar, VarType::Scalar) => {
                                        // this is a scalar operation
                                        new_body.push(expr.clone());
                                    }
                                    (VarType::Vector, VarType::Vector) => {
                                        // need to create new ids for the result
                                        assert!(opr1.size == opr2.size);
                                        let size = match opr1.size {
                                            IdSize::Vector { len } => len,
                                            _ => panic!("There should be a vector here!"),
                                        };
                                        let ids_0 = unroll_vector_map.get(opr1).unwrap();
                                        let ids_1 = unroll_vector_map.get(opr2).unwrap();

                                        let mut all_ids = vec![];
                                        for i in 0..size {
                                            let new_name = format!("{}_{}", result_id.name, i);
                                            let new_id = Identifier::new_scalar(&new_name);
                                            let new_expr = Expr::Let {
                                                id: new_id.clone(),
                                                opr: Opr::Binary {
                                                    opr1: ids_0[i].clone(),
                                                    opr2: ids_1[i].clone(),
                                                    opr_type: opr_type.clone(),
                                                },
                                            };
                                            new_body.push(new_expr);
                                            all_ids.push(new_id);
                                        }
                                        unroll_vector_map.insert(result_id.clone(), all_ids);
                                    }
                                    (VarType::Matrix, VarType::Matrix) => {
                                        // this is a matrix operation
                                        assert!(opr1.size == opr2.size);
                                        let (row_size, col_size) = match opr1.size {
                                            IdSize::Matrix { row_size, col_size } => {
                                                (row_size, col_size)
                                            }
                                            _ => panic!("There should be a matrix here!"),
                                        };

                                        let ids_0 = unroll_matrix_map.get(opr1).unwrap();
                                        let ids_1 = unroll_matrix_map.get(opr2).unwrap();

                                        let mut all_ids: Vec<Vec<Identifier>> = vec![];
                                        for i in 0..row_size {
                                            let mut row_ids = vec![];
                                            for j in 0..col_size {
                                                let new_name =
                                                    format!("{}_{}_{}", result_id.name, i, j);
                                                let new_id = Identifier::new_scalar(&new_name);
                                                let new_expr = Expr::Let {
                                                    id: new_id.clone(),
                                                    opr: Opr::Binary {
                                                        opr1: ids_0[i][j].clone(),
                                                        opr2: ids_1[i][j].clone(),
                                                        opr_type: opr_type.clone(),
                                                    },
                                                };
                                                new_body.push(new_expr);
                                                row_ids.push(new_id);
                                            }
                                            all_ids.push(row_ids);
                                        }
                                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                                    }
                                    (VarType::Vector, VarType::Scalar) => {
                                        // all elements of vector is divided by scalar
                                        let ids = unroll_vector_map.get(opr1).unwrap();

                                        let mut all_ids = vec![];
                                        for (i, id) in ids.iter().enumerate() {
                                            let new_name = format!("{}_{}", result_id.name, i);
                                            let new_id = Identifier::new_scalar(&new_name);
                                            let new_expr = Expr::Let {
                                                id: new_id.clone(),
                                                opr: Opr::Binary {
                                                    opr1: id.clone(),
                                                    opr2: opr2.clone(),
                                                    opr_type: opr_type.clone(),
                                                },
                                            };
                                            new_body.push(new_expr);
                                            all_ids.push(new_id);
                                        }
                                        unroll_vector_map.insert(result_id.clone(), all_ids);
                                    }
                                    (VarType::Matrix, VarType::Scalar) => {
                                        // this is a matrix operation
                                        let ids = unroll_matrix_map.get(opr1).unwrap();

                                        let mut all_ids: Vec<Vec<Identifier>> = vec![];
                                        for (i, row) in ids.iter().enumerate() {
                                            let mut row_ids = vec![];
                                            for (j, id) in row.iter().enumerate() {
                                                let new_name =
                                                    format!("{}_{}_{}", result_id.name, i, j);
                                                let new_id = Identifier::new_scalar(&new_name);
                                                let new_expr = Expr::Let {
                                                    id: new_id.clone(),
                                                    opr: Opr::Binary {
                                                        opr1: id.clone(),
                                                        opr2: opr2.clone(),
                                                        opr_type: opr_type.clone(),
                                                    },
                                                };
                                                new_body.push(new_expr);
                                                row_ids.push(new_id);
                                            }
                                            all_ids.push(row_ids);
                                        }

                                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                                    }
                                    (VarType::Scalar, VarType::Vector) => {
                                        // if sub or div, this does not make sense
                                        match opr_type {
                                            OprBinary::Sub | OprBinary::Div => {
                                                panic!(
                                                    "This kind of operation {:?} between {:?} and {:?} does not make sense",
                                                    opr_type, opr1.size, opr2.size
                                                );
                                            }
                                            _ => {}
                                        }

                                        let ids = unroll_vector_map.get(opr2).unwrap();

                                        let mut all_ids = vec![];
                                        for (i, id) in ids.iter().enumerate() {
                                            let new_name = format!("{}_{}", result_id.name, i);
                                            let new_id = Identifier::new_scalar(&new_name);
                                            let new_expr = Expr::Let {
                                                id: new_id.clone(),
                                                opr: Opr::Binary {
                                                    opr1: opr1.clone(),
                                                    opr2: id.clone(),
                                                    opr_type: opr_type.clone(),
                                                },
                                            };
                                            new_body.push(new_expr);
                                            all_ids.push(new_id);
                                        }
                                        unroll_vector_map.insert(result_id.clone(), all_ids);
                                    }
                                    (VarType::Scalar, VarType::Matrix) => {
                                        // if sub or div, this does not make sense
                                        match opr_type {
                                            OprBinary::Sub | OprBinary::Div => {
                                                panic!(
                                                    "This kind of operation {:?} between {:?} and {:?} does not make sense",
                                                    opr_type, opr1.size, opr2.size
                                                );
                                            }
                                            _ => {}
                                        }

                                        let ids = unroll_matrix_map.get(opr2).unwrap();

                                        let mut all_ids: Vec<Vec<Identifier>> = vec![];
                                        for (i, row) in ids.iter().enumerate() {
                                            let mut row_ids = vec![];
                                            for (j, id) in row.iter().enumerate() {
                                                let new_name =
                                                    format!("{}_{}_{}", result_id.name, i, j);
                                                let new_id = Identifier::new_scalar(&new_name);
                                                let new_expr = Expr::Let {
                                                    id: new_id.clone(),
                                                    opr: Opr::Binary {
                                                        opr1: opr1.clone(),
                                                        opr2: id.clone(),
                                                        opr_type: opr_type.clone(),
                                                    },
                                                };
                                                new_body.push(new_expr);
                                                row_ids.push(new_id);
                                            }
                                            all_ids.push(row_ids);
                                        }

                                        unroll_matrix_map.insert(result_id.clone(), all_ids);
                                    }
                                    _ => panic!(
                                        "This kind of operation {:?} between {:?} and {:?} does not make sense",
                                        opr_type, opr1.size, opr2.size
                                    ),
                                }
                                // does not make sense,
                            }
                        }
                    }
                }
            }
        }
    }
    // if the id is a vector or a matrix, reconstruct it so that its errors are accessible to the user
    for (id, _) in program.get_outputs() {
        let new_output = match id.var_type {
            VarType::Scalar => {
                ProgramOutput::Scalar {
                    info: Output {
                        id: id.clone(), // in this case the id is the same
                        range: (Real::zero(), Real::zero()),
                        error: (Real::zero(), Real::zero()),
                    },
                }
            }
            VarType::Vector => {
                let all_ids = unroll_vector_map.get(id).unwrap();
                ProgramOutput::Vector {
                    info: all_ids
                        .iter()
                        .map(|id| Output {
                            id: id.clone(), // we use new ids for all
                            range: (Real::zero(), Real::zero()),
                            error: (Real::zero(), Real::zero()),
                        })
                        .collect(),
                }
            }
            VarType::Matrix => {
                let all_ids = unroll_matrix_map.get(id).unwrap();
                ProgramOutput::Matrix {
                    info: all_ids
                        .iter()
                        .map(|row| {
                            row.iter()
                                .map(|id| Output {
                                    id: id.clone(), // new ids for all
                                    range: (Real::zero(), Real::zero()),
                                    error: (Real::zero(), Real::zero()),
                                })
                                .collect()
                        })
                        .collect(),
                }
            }
        };
        new_outputs.insert(id.clone(), new_output);
    }

    // return the new program
    let mut new_program = Program::new();
    new_program.set_inputs(&new_inputs);
    new_program.set_outputs(&new_outputs);
    new_program.set_body(&new_body);
    new_program
}
