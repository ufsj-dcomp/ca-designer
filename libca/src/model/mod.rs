use std::collections::HashMap;

use edge::EdgeId;
pub use edge::{Condition, Edge, Operand, Value};
pub use node::Node;
use serde::{Deserialize, Serialize};

pub use node::NodeId;

use crate::state_map::StateMap;

mod edge;
mod node;

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) edges: Vec<Edge>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_state(&self, curr_state: NodeId, neighbors: &StateMap) -> NodeId {
        self.edges
            .iter()
            .find_map(|edge| edge.transition(curr_state, neighbors))
            .unwrap_or(curr_state)
    }

    pub fn nodes(&self) -> impl ExactSizeIterator<Item = (&NodeId, &Node)> {
        self.nodes.iter()
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn all_edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn edges_from_node<'n, 's: 'n>(
        &'s self,
        from_node_id: &'n NodeId,
    ) -> impl Iterator<Item = &'s Edge> + use<'s, 'n> {
        self.edges
            .iter()
            .filter(move |edge| &edge.from_node == from_node_id)
    }
}

// Known Models
impl Model {
    pub fn game_of_life() -> Self {
        let dead = NodeId(0);
        let alive = NodeId(1);

        Self {
            nodes: [
                (dead, Node("Dead".to_string())),
                (alive, Node("Alive".to_string())),
            ]
            .into_iter()
            .collect(),
            edges: vec![
                // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                Edge {
                    id: EdgeId(0),
                    name: "Underpopulation".to_string(),
                    from_node: alive,
                    to_node: dead,
                    conditions: vec![Condition {
                        left: Value::PopulationCount(alive),
                        operand: Operand::Less,
                        right: Value::Absolute(2),
                    }],
                },
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                Edge {
                    id: EdgeId(1),
                    name: "Overpopulation".to_string(),
                    from_node: alive,
                    to_node: dead,
                    conditions: vec![Condition {
                        left: Value::PopulationCount(alive),
                        operand: Operand::Greater,
                        right: Value::Absolute(3),
                    }],
                },
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                Edge {
                    id: EdgeId(2),
                    name: "Reproduction".to_string(),
                    from_node: dead,
                    to_node: alive,
                    conditions: vec![Condition {
                        left: Value::PopulationCount(alive),
                        operand: Operand::Equal,
                        right: Value::Absolute(3),
                    }],
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_serialization() {
        insta::with_settings!({sort_maps =>true}, {
            insta::assert_ron_snapshot!(Model::game_of_life());
        });
    }
}
