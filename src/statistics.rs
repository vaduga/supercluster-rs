use std::collections::HashMap;
use std::fmt::Debug;
use gloo_console::__macro::JsValue;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

pub trait Accumulator: Debug {
    fn init(&self, i: usize) -> Statistic;
    fn accumulate(&mut self, i: usize, value: usize);
}

#[derive(Debug, Clone)]
pub struct ThresholdIdsCounter {
    values: Vec<HashMap<usize, usize>>,
}

impl ThresholdIdsCounter {
    pub fn new() -> Self {
        let mut initial_map = HashMap::new();
        for i in 0..=10 {
            initial_map.insert(i, 0);
        }
         Self {
             values: vec![initial_map; 100], // total number of points
         }
    }
}

impl Accumulator for ThresholdIdsCounter {
    fn init(&self, i: usize) -> Statistic {
        // Get the value at index i, which is guaranteed to exist
        let value = self.values[i].clone();

        Statistic::new(value)
    }

    fn accumulate(&mut self, i:usize, value: usize) {
        if let Some(count) = self.values[i].get_mut(&value) {
           *count += 1;
        }
    }

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistic {
    pub(crate) value: HashMap<usize, usize>,
}

impl Statistic {
    pub fn new(value: HashMap<usize, usize>) -> Self {
        Self { value }
        //todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics(pub(crate) HashMap<String, Statistic>);

impl Statistics {
    pub fn new(stats: HashMap<String, Statistic>) -> Self {
        Self(stats)
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self(HashMap::new())
    }
}


