mod into_wit;
mod model;

pub use into_wit::*;
pub use model::*;

pub use linkme::distributed_slice;

#[doc(hidden)]
#[distributed_slice]
pub static ALL_WIT_TYPES_FOR_GOLEM: [fn() -> WitExport];

#[doc(hidden)]
pub fn get_all_wit_types() -> Vec<WitExport> {
    ALL_WIT_TYPES_FOR_GOLEM.iter().map(|f| f()).collect()
}
