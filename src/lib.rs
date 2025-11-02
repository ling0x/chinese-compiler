use std::collections::HashMap;

use pest_derive::Parser;

pub mod chinese_number;
pub mod codegen;
pub mod evaluator;
pub mod executor;
pub mod parser;

#[derive(Parser)]
#[grammar = "chinese_lang.pest"]
pub struct ChineseLangParser;

// Runtime environment to store variables
pub struct Environment {
    variables: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    String(String),
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
