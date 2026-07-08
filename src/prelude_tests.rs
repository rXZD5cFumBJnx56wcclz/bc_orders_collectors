#![allow(unused_imports)]

#[cfg(test)]
pub mod prelude {
    pub use std::sync::LazyLock;

    pub use bc_utils_lg::structs::trade::*;
    pub use pretty_assertions::assert_eq as assert_eq_pr;

    pub use crate::main_trait::OrderCollector;
}
