use std::collections::HashMap;
use std::fmt::Debug;
use gloo_console::__macro::JsValue;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

pub trait Accumulator: Debug {
    fn init(&self, i: usize) -> Statistic;
    fn accumulate(&mut self, value: usize);
}

#[derive(Debug, Clone)]
pub struct ThresholdCounter {
    counts: HashMap<usize, usize>,
}

impl ThresholdCounter {
    pub fn new() -> Self {
        let mut counts = HashMap::new();
        for i in 0..=10 {
            counts.insert(i, 0);
        }

        Self { counts }
    }
}

impl Accumulator for ThresholdCounter {
    fn init(&self, i: usize) -> Statistic {
        // Convert counts to HashMap<String, Statistic>
        let mut stats_map = HashMap::new();
        for (key, &value) in &self.counts {
            stats_map.insert(
                key.clone(),
                value,
            );
        }
        Statistic {
            value: stats_map
        }
    }

    fn accumulate(&mut self, value: usize) {
        if let Some(count) = self.counts.get_mut(&value) {
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


