#![feature(iter_collect_into)]
#![feature(vec_into_raw_parts)]

pub mod grid;
pub mod model;
pub mod simulation;
pub mod state_map;

pub use model::{Condition, Edge, Model, Node, NodeId, Operand, Value};
