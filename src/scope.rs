use std::collections::HashMap;

use crate::{err::InterpError, instruction::StackValue};

#[derive(Debug)]
pub struct Scope {
    pub var_store: HashMap<String, StackValue>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
        }
    }

    pub fn get_var(&self, id: &String) -> Result<&StackValue, InterpError> {
        self.var_store
            .get(id)
            .ok_or(InterpError::VariableNotFound(id.to_string()))
    }

    pub fn set_var(&mut self, id: String, val: StackValue) -> Option<StackValue> {
        self.var_store.insert(id, val)
    }

    pub fn from_scope(other: &Scope) -> Self {
        let mut scope = Scope {
            var_store: HashMap::new(),
        };
        for (k, v) in other.var_store.iter() {
            scope.set_var(k.to_string(), *v);
        }
        return scope;
    }

    pub fn assign(&mut self, kv: HashMap<&String, &StackValue>) {
        for kv in kv.iter() {
            self.set_var(kv.0.to_string(), **kv.1);
        }
    }
}
