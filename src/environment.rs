use std::collections::HashMap;
use crate::model::{CellAddress, Evaluatable};

pub struct Environment {
    map: HashMap<CellAddress, Box<dyn Evaluatable>>,
}

impl Environment {
    pub fn init() -> Environment {
        Environment { map: HashMap::new(), }
    }
    
    pub fn set_cell(&mut self, adr: &CellAddress, val: Box<dyn Evaluatable>) {
       self.map.insert(*adr, val);
    }

    pub fn get_cell(&self, adr: &CellAddress) -> Option<&Box<dyn Evaluatable>> {
        self.map.get(&adr)
    }
}
