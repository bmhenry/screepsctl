use std::str::FromStr;

use screeps::{HasId, ObjectId, SizedRoomObject};
use screeps::memory::MemoryReference;


mod js;
pub mod metrics;

pub use js::*;




pub fn obj_from_mem_id<T: HasId + SizedRoomObject>(mem: MemoryReference, name: &str) -> Option<T> {
    mem.string(name)
        .ok()
        .flatten()
        .map(|raw_id| ObjectId::<T>::from_str(raw_id.as_str()).ok())
        .flatten()
        .map(|id| id.try_resolve().ok())
        .flatten()
        .flatten()
}