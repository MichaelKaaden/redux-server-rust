use std::collections::HashMap;
use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    index: u32,
    value: i32,
}

pub fn get_counters(counters: MutexGuard<HashMap<u32, i32>>) -> Vec<Counter> {
    let mut indices = counters.keys().into_iter().collect::<Vec<&u32>>();
    indices.sort(); // return counters sorted ascending by index

    let mut result: Vec<Counter> = Vec::new();
    for index in indices {
        result.push(Counter {
            index: *index,
            value: *counters.get(index).unwrap(),
        });
    }

    result
}
