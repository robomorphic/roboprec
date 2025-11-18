use once_cell::sync::Lazy;
use std::{
    hash::{Hash, Hasher},
    sync::Mutex,
};

use super::helper::add_name;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum VarType {
    Scalar,
    Vector,
    Matrix,
}

static UNIQUE_ID_COUNTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

fn gen_unique_id() -> usize {
    let mut counter = UNIQUE_ID_COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdSize {
    Scalar,
    Vector { len: usize },
    Matrix { row_size: usize, col_size: usize },
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: usize,
    pub name: String,
    pub var_type: VarType,
    pub size: IdSize,
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        //self.name.hash(state);
        //self.var_type.hash(state);
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Identifier {}

impl Identifier {
    pub fn new_scalar(name: &str) -> Self {
        Self {
            id: gen_unique_id(),
            name: add_name(name),
            var_type: VarType::Scalar,
            size: IdSize::Scalar,
        }
    }

    pub fn new_vector(name: &str, size: usize) -> Self {
        Self {
            id: gen_unique_id(),
            name: add_name(name),
            var_type: VarType::Vector,
            size: IdSize::Vector { len: size },
        }
    }

    pub fn new_matrix(name: &str, row_size: usize, col_size: usize) -> Self {
        Self {
            id: gen_unique_id(),
            name: add_name(name),
            var_type: VarType::Matrix,
            size: IdSize::Matrix { row_size, col_size },
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
