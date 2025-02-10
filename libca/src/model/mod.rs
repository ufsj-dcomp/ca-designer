use std::collections::BTreeMap;

use edge::EdgeId;
pub use edge::{Condition, Edge, Operand, Value};
pub use node::Node;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

pub use node::NodeId;

use crate::state_map::StateMap;

mod edge;
mod node;

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    pub(crate) nodes: BTreeMap<NodeId, Node>,
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

    pub fn add_node(&mut self, node: Node) {
        let next_node_id = self
            .nodes
            .last_key_value()
            .map(|(node_id, _)| node_id.as_index() + 1)
            .unwrap_or_default();

        let node_id = NodeId(next_node_id);
        self.nodes.insert(node_id, node);
    }

    pub fn add_edge(&mut self, mut edge: Edge) -> &EdgeId {
        let next_edge_id = self
            .edges
            .iter()
            .map(|edge| edge.id.0)
            .max()
            .map(|id| id + 1)
            .unwrap_or_default();

        let insert_at_idx = self.edges.len();

        edge.id = EdgeId(next_edge_id);
        self.edges.push(edge);

        &self.edges[insert_at_idx].id
    }

    pub fn delete_node(&mut self, node_id: NodeId) {
        let Some(_) = self.nodes.remove(&node_id) else {
            return;
        };

        self.edges
            .retain(|edge| edge.from_node != node_id && edge.to_node != node_id);

        let Some((highest_node_id, _)) = self.nodes.last_key_value() else {
            return;
        };

        let highest_node_id = *highest_node_id;

        if highest_node_id < node_id {
            return;
        }

        // Minimize NodeId's by getting the highest node and assigning it
        // the removed node's ID
        self.edges.par_iter_mut().for_each(|edge| {
            if edge.from_node == highest_node_id {
                edge.from_node = node_id;
            }

            if edge.to_node == highest_node_id {
                edge.to_node = node_id;
            }
        });

        if let Some(highest_node) = self.nodes.remove(&highest_node_id) {
            self.nodes.insert(node_id, highest_node);
        }
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
