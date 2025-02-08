use std::collections::HashMap;

use edge::Edge;
use node::Node;
use serde::{Deserialize, Serialize};

pub use node::NodeId;

use crate::state_map::StateMap;

mod edge;
mod node;

#[cfg(test)]
pub mod test_utils;

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) edges: Vec<Edge>,
}

impl Model {
    pub fn next_state(&self, curr_state: NodeId, neighbors: &StateMap) -> NodeId {
        self.edges
            .iter()
            .find_map(|edge| edge.transition(curr_state, neighbors))
            .unwrap_or(curr_state)
    }
}

#[cfg(test)]
mod tests {
    use test_utils::game_of_life_rules;

    use super::*;

    #[test]
    fn model_serialization() {
        insta::with_settings!({sort_maps =>true}, {
            insta::assert_ron_snapshot!(&game_of_life_rules());
        });
    }
}
