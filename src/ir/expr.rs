use log::warn;

use crate::{analysis::real::Real, ir::identifier::IdSize};

use super::{identifier::Identifier, program::get_program};

#[derive(Debug, Clone)]
pub enum OprUnary {
    Neg,    // Negation
    Assign, // Reassignment to a different variable (in our context this only means a name change)
    AssignNoOpt,
    Index { index: Vec<usize> }, // Indexing operation
    Transpose,                   // Transpose operation (for matrices)
}

#[derive(Debug, Clone)]
pub enum OprBinary {
    Add,   // Addition
    Sub,   // Subtraction
    Mul,   // Multiplication
    Div,   // Division
    Cross, // Cross product (only for 3D vectors)
    Dot,   // Dot product (only for vectors)
}

#[derive(Debug, Clone)]
/// I need to refactor this, I don't really like ConstantScalar, ConstantVector and ConstantMatrix.
pub enum Opr {
    ConstantScalar {
        value: Real,
    },
    ConstantVector {
        value: Vec<Real>,
    },
    ConstantMatrix {
        value: Vec<Vec<Real>>,
    },
    Unary {
        opr1: Identifier,
        opr_type: OprUnary,
    },
    Binary {
        opr1: Identifier,
        opr2: Identifier,
        opr_type: OprBinary,
    },
    // This one is used to construct a new vector or matrix from a list of identifiers.
    ConstructScalar {
        id: Identifier,
    },
    ConstructVector {
        ids: Vec<Identifier>,
    },
    ConstructMatrix {
        ids: Vec<Vec<Identifier>>,
    },
}

#[derive(Debug, Clone)]
/// This expression is only used as an input for the analysis stages.
/// It should be later replaced by a more complex AST, specifically for analysis stages.
pub enum Expr {
    Let { id: Identifier, opr: Opr },
}

pub fn create_scalar_constant(name: &str, value: Real) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstantScalar { value },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_vector_constant(name: &str, value: Vec<Real>) -> Identifier {
    let size = value.len();
    let new_id = Identifier::new_vector(name, size);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstantVector { value },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_matrix_constant(name: &str, value: Vec<Vec<Real>>) -> Identifier {
    let size0 = value.len();
    let size1 = if size0 == 0 { 0 } else { value[0].len() };
    let new_id = Identifier::new_matrix(name, size0, size1);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstantMatrix { value },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_unary_scalar_expr(name: &str, opr1: Identifier, opr_type: OprUnary) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Unary { opr1, opr_type },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_unary_vector_expr(name: &str, opr1: Identifier, opr_type: OprUnary) -> Identifier {
    let size = match opr1.size {
        IdSize::Vector { len } => len,
        _ => panic!("There should be a vector here!"),
    };
    let new_id = Identifier::new_vector(name, size);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Unary { opr1, opr_type },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_unary_matrix_expr(name: &str, opr1: Identifier, opr_type: OprUnary) -> Identifier {
    let (size0, size1) = match opr1.size {
        IdSize::Matrix { row_size, col_size } => (row_size, col_size),
        _ => panic!("There should be a matrix here!"),
    };
    let new_id = Identifier::new_matrix(name, size0, size1);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Unary { opr1, opr_type },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_binary_scalar_expr(
    name: &str,
    opr1: Identifier,
    opr2: Identifier,
    opr_type: OprBinary,
) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Binary {
            opr1,
            opr2,
            opr_type,
        },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_binary_vector_expr(
    name: &str,
    opr1: Identifier,
    opr2: Identifier,
    opr_type: OprBinary,
    len: usize,
) -> Identifier {
    let new_id = Identifier::new_vector(name, len);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Binary {
            opr1,
            opr2,
            opr_type,
        },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_binary_matrix_expr(
    name: &str,
    opr1: Identifier,
    opr2: Identifier,
    opr_type: OprBinary,
    row_size: usize,
    col_size: usize,
) -> Identifier {
    let new_id = Identifier::new_matrix(name, row_size, col_size);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Binary {
            opr1,
            opr2,
            opr_type,
        },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_construct_scalar_expr(name: &str, id: Identifier) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstructScalar { id },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_construct_vector_expr(name: &str, ids: Vec<Identifier>) -> Identifier {
    let size = ids.len();
    let new_id = Identifier::new_vector(name, size);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstructVector { ids },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_construct_matrix_expr(name: &str, ids: Vec<Vec<Identifier>>) -> Identifier {
    let size0 = ids.len();
    let size1 = ids[0].len();
    let new_id = Identifier::new_matrix(name, size0, size1);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::ConstructMatrix { ids },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_index_vector_expr(name: &str, opr1: Identifier, index: usize) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Unary {
            opr1,
            opr_type: OprUnary::Index { index: vec![index] },
        },
    };
    get_program().add_expr(expr);
    new_id
}

pub fn create_index_matrix_row_expr(
    _name: &str,
    _opr1: Identifier,
    _row_index: usize,
) -> Identifier {
    // we first need to create a bunch of scalar identifiers for each element in the row
    // then we need to create a vector identifier for the row
    // finally we need to create the expression for the vector identifier
    warn!("This function \"create_index_matrix_row_expr\" is not tested very well!");
    let size1 = match _opr1.size {
        IdSize::Matrix {
            row_size: _,
            col_size,
        } => col_size,
        _ => panic!("There should be a matrix here!"),
    };
    let mut element_ids: Vec<Identifier> = vec![];
    for col_index in 0..size1 {
        let element_name = format!("{}_row{}_col{}", _name, _row_index, col_index);
        let element_id =
            create_index_matrix_element_expr(&element_name, _opr1.clone(), _row_index, col_index);
        element_ids.push(element_id);
    }
    create_construct_vector_expr(_name, element_ids)
}

pub fn create_index_matrix_element_expr(
    name: &str,
    opr1: Identifier,
    row_index: usize,
    col_index: usize,
) -> Identifier {
    let new_id = Identifier::new_scalar(name);
    let expr = Expr::Let {
        id: new_id.clone(),
        opr: Opr::Unary {
            opr1,
            opr_type: OprUnary::Index {
                index: vec![row_index, col_index],
            },
        },
    };
    get_program().add_expr(expr);
    new_id
}

/*
pub enum Expr<T>
where
    T: Range
{
    LetScalar{
        name: String,
        opr: Opr,
        /// if the expr is constant, it will always be populated, otherwise it will be set as the computation result with default values
        value: Option<Real>,
        range: Option<T>,
        accuracy: Option<T>
    },
    LetVector{
        name: String,
        opr: Opr,
        /// if the expr is constant, it will always be populated, otherwise it will be set as the computation result with default values
        value: Option<Vec<Real>>,
        range: Option<Vec<T>>,
        accuracy: Option<Vec<T>>
    },
    ExprMatrix{
        name: String,
        opr: Opr,
        /// if the expr is constant, it will always be populated, otherwise it will be set as the computation result with default values
        value: Option<Vec<Vec<Real>>>,
        range: Option<Vec<Vec<T>>>,
        accuracy: Option<Vec<Vec<T>>>
    }
}
*/
