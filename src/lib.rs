use std::collections::HashMap;
use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    index: u32,
    value: i32,
}

pub fn get_counters(counters: MutexGuard<HashMap<u32, i32>>) -> Vec<Counter> {
    let initial_index: u32 = 0;
    let initial_value = *counters.get(&initial_index).unwrap();

    vec![Counter {
        index: initial_index,
        value: initial_value,
    }]
}
