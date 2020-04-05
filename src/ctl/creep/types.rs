//!
//! 
//!

use screeps::creep::Part;


/// High level creep types
pub enum CreepType {
    Harvester(HarvesterType),
    Builder(BuilderType),
}

/// Types of harvester creeps
pub enum HarvesterType {
    BasicHarvester(BasicHarvester),
}

/// Types of builder creeps
pub enum BuilderType {
    BasicBuilder(BasicBuilder),
}

/// Get info for a creep type
pub trait CreepInfo {
    /// Returns a JSON friendly role name
    fn role() -> &'static str;

    /// Gets the parts required by a creep
    fn parts() -> &'static [Part];

    /// get the cost associated with the creep parts
    fn cost() -> u32;
}


/// Information for creating and using a basic harvester
pub struct BasicHarvester {}

static BASIC_HARVESTER_PARTS: [Part; 4] = [Part::Move, Part::Move, Part::Carry, Part::Work];
static mut BASIC_HARVESTER_COST: Option<u32> = None;

impl CreepInfo for BasicHarvester {
    fn role() -> &'static str {
        "basic_harvester"
    }

    fn parts() -> &'static [Part] {
        &BASIC_HARVESTER_PARTS
    }

    fn cost() -> u32 {
        unsafe {
            if BASIC_HARVESTER_COST.is_none() {
                BASIC_HARVESTER_COST = Some(BASIC_HARVESTER_PARTS.iter().fold(0, |cost, part| cost + part.cost()));
            }
            BASIC_HARVESTER_COST.unwrap()
        }
    }
}


/// Information for creating and using a basic builder
pub struct BasicBuilder {}

static BASIC_BUILDER_PARTS: [Part; 4] = [Part::Move, Part::Move, Part::Carry, Part::Work];
static mut BASIC_BUILDER_COST: Option<u32> = None;

impl CreepInfo for BasicBuilder {
    fn role() -> &'static str {
        "basic_builder"
    }
    
    fn parts() -> &'static [Part] {
        &BASIC_BUILDER_PARTS
    }

    fn cost() -> u32 {
        unsafe {
            if BASIC_BUILDER_COST.is_none() {
                BASIC_BUILDER_COST = Some(BASIC_BUILDER_PARTS.iter().fold(0, |cost, part| cost + part.cost()));
            }
            BASIC_BUILDER_COST.unwrap()
        }
    }
}