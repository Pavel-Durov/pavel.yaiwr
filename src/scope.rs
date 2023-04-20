use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::instruction::{Instruction, StackValue};

#[derive(Debug, Clone)]
pub struct Scope {
    var_store: HashMap<u64, StackValue>,
    fun_store: HashMap<u64, Instruction>,
    outter_scope: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            var_store: HashMap::new(),
            outter_scope: None,
            fun_store: HashMap::new(),
        }
    }

    pub fn get_var_store_len(&self) -> usize {
        self.var_store.len()
    }

    pub fn from_scope(outer_scope: Rc<RefCell<Scope>>) -> Self {
        let scope = Scope {
            var_store: HashMap::new(),
            outter_scope: Some(outer_scope),
            fun_store: HashMap::new(),
        };
        return scope;
    }

    fn __get_func(&self, func_id: u64) -> Option<&Instruction> {
        match (self.fun_store.get(&func_id), self.get_var(func_id)) {
            (Some(f), _) => Some(f),
            (_, Some(StackValue::Function(f_id))) => self.fun_store.get(&f_id),
            _ => None,
        }
    }
    pub fn get_func(&self, id: u64) -> Option<Instruction> {
        let var = self.__get_func(id);
        match var {
            Some(x) => {
                return Some(x.clone());
            }
            None => {
                if let Some(out) = self.outter_scope.clone() {
                    out.borrow().get_func(id.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn set_func(&mut self, id: u64, func: Instruction) -> Option<Instruction> {
        self.fun_store.insert(id, func)
    }

    pub fn dec_var(&mut self, id: u64, val: StackValue) -> Option<StackValue> {
        self.var_store.insert(id, val)
    }

    pub fn set_var(&mut self, id: u64, val: StackValue) -> Option<StackValue> {
        let var = self.var_store.get(&id.clone());
        
        match var {
            Some(..) => {
                self.var_store.insert(id, val);
                Some(val)
            }
            None => {
                if let Some(out) = &mut self.outter_scope {
                    return out.clone().borrow_mut().set_var(id, val);
                } else {
                    return None;
                }
            }
        }
    }

    pub fn get_var(&self, id: u64) -> Option<StackValue> {
        let var = self.var_store.get(&id.clone());
        match var {
            Some(x) => {
                return Some(*x);
            }
            None => {
                if let Some(out) = self.outter_scope.clone() {
                    out.borrow().get_var(id)
                } else {
                    None
                }
            }
        }
    }
}
