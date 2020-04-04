//!
//! Utility functions
//!

use stdweb::js;
use stdweb::unstable::TryInto;

#[allow(unused)]
pub fn random() -> u32 {
    let rnd: f64 = js! { return Math.random() }.try_into().unwrap();
    (rnd * std::u32::MAX as f64).floor() as u32
}