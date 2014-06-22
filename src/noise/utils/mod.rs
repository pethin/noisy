//! Miscelaneous, helper functions.

pub use utils::fastfloor::fastfloor;
pub use utils::if_else::if_else;
pub use utils::grad::{grad1, grad2, grad3};
pub use utils::lerp::lerp;
pub use utils::fade::fade;

mod fastfloor;
mod if_else;
mod grad;
mod lerp;
mod fade;
