//!
//! Resource details & calculations
//!

use log::*;

use screeps::{HasPosition, LookResult, RoomObjectProperties, Source, Terrain};


// number of currently available spots around the source
pub fn free_source_spots(source: &Source) -> u32 {
    let mut count: i32 = 0;

    let x = source.pos().x();
    let y = source.pos().y();
    let places = source.room().look_at_area(y-1, x-1, y+1, x+1);

    for place in places {
        match place.look_result {
            // for possible open spots, increment by one
            LookResult::Terrain(Terrain::Plain) |
            LookResult::Terrain(Terrain::Swamp) => {
                debug!("Found plain or swamp @ {},{}", place.x, place.y);
                count += 1;
            },
            // for occupied spots, decrement by one
            LookResult::Creep(_) => {
                count -= 1;
            }
            _ => ()
        }
    }

    count as u32
}

/// number of generally occupiable spots around the source
pub fn total_source_spots(source: &Source) -> u32 {
    let mut count: u32 = 0;

    let x = source.pos().x();
    let y = source.pos().y();
    let places = source.room().look_at_area(y-1, x-1, y+1, x+1);

    for place in places {
        match place.look_result {
            LookResult::Terrain(Terrain::Plain) | 
            LookResult::Terrain(Terrain::Swamp) => {
                debug!("Found plain or swamp @ {},{}", place.x, place.y);
                count += 1;
            },
            _ => ()
        }
    }

    count
}