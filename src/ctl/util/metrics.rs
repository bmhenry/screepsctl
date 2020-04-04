//!
//! Global metrics
//!

use std::collections::HashMap;

use log::*;
// use serde::{Serialize, Deserialize};

use screeps::memory;


static mut METRICS: Option<Metrics> = None;


/// Contains global data about shard state, both for a single tick
/// and over a set amount of time
#[derive(Debug)]
struct Metrics {
    counts: HashMap<String, u32>,
}

impl Metrics {
    pub fn init() -> Metrics {
        Metrics {
            counts: HashMap::<String, u32>::new()
        }
    }

    // handle moving to the next tick (resetting counts, popping oldest, etc)
    pub fn tick(&mut self) {
        self.counts.clear()
    }
}

/// Returns a reference to the global metrics structure
fn get_metrics() -> &'static mut Metrics {
    unsafe {
        METRICS.get_or_insert(Metrics::init())
    }
}

/// Manually initialize metrics
pub fn init_metrics() {
    let _ = get_metrics();
}

/// Step metrics to next tick
pub fn tick_metrics() {
    get_metrics().tick();
}

/// Increments one of the 'count' metrics by name
fn inc_count(key: &str, mut val: u32) {
    let metrics = get_metrics();
    if let Some((_k, v)) = metrics.counts.get_key_value(key) {
        val += v;
    }
    metrics.counts.insert(String::from(key), val);
}

/// Increment the number of harvester creeps spawned this tick
pub fn inc_harvesters(count: u32) {
    inc_count("harvester_creeps", count);
}

/// Increment the amount of energy stored this tick
pub fn inc_energy(count: u32) {
    inc_count("energy", count);
}

/// Saves the metrics to memory, so they can be accessed again next tick
pub fn save() {
    let _ = memory::root().dict_or_create("metrics");
    let _ = memory::root().dict_or_create("metrics.count");

    memory::root().path_set("metrics", &get_metrics().counts)
}

/// Logs the current metrics to the console
pub fn log() {
    debug!("METRICS:\nCounts: {:#?}\n", get_metrics().counts)
}