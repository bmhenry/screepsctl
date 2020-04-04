//!
//! 
//!

use screeps::creep::Part;


/// High level creep types
pub enum CreepType {
    Harvester(HarvesterType),
}

/// Types of harvester creeps
pub enum HarvesterType {
    BasicHarvester(BasicHarvester),
}

/// Get info for a creep type
pub trait CreepInfo {
    /// Gets the parts required by a creep
    fn parts() -> Vec<Part>;

    /// get the cost associated with the creep parts
    fn cost() -> u32;
}


/// Information for creating and using a basic harvester
pub struct BasicHarvester {}

impl CreepInfo for BasicHarvester {
    fn parts() -> Vec<Part> {
        vec![Part::Move, Part::Move, Part::Carry, Part::Work]
    }

    fn cost() -> u32 {
        Self::parts().iter().fold(0, |cost, part| cost + part.cost())
    }
}
