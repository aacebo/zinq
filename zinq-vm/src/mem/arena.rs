use crate::value::{Value, ValueId};

#[derive(Debug, Clone)]
pub struct Arena {
    items: Vec<Value>,
}

impl Arena {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn items(&self) -> &[Value] {
        &self.items
    }

    pub fn get(&self, id: ValueId) -> &Value {
        &self.items[id.to_usize()]
    }

    pub fn get_mut(&mut self, id: ValueId) -> &mut Value {
        &mut self.items[id.to_usize()]
    }

    pub fn push(&mut self, value: Value) -> ValueId {
        let id = ValueId::from(self.items.len() as u32);
        self.items.push(value);
        id
    }
}
