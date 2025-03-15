#![feature(iter_collect_into)]
#![feature(vec_into_raw_parts)]

pub mod grid;
pub mod model;
pub mod simulation;
pub mod state_map;

use std::{num::NonZero, sync::LazyLock};

pub use model::{Condition, Edge, Model, Node, NodeId, Operand, Value};

pub static AVAILABLE_PARALLELISM: LazyLock<usize> = LazyLock::new(|| {
    std::thread::available_parallelism()
        .map(NonZero::get)
        .unwrap_or(1)
});
