mod into_wit;
mod model;

pub use into_wit::*;
pub use model::*;

#[macro_export]
macro_rules! golem_gen {
    () => {
        #[distributed_slice]
        pub static ALL_WIT_TYPES_FOR_GOLEM: [fn() -> WitExport];
    };
}
