use serde::{Deserialize, Serialize};

use crate::state_map::StateMap;

use super::node::NodeId;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EdgeId(pub(crate) u32);

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub(crate) id: EdgeId,
    pub(crate) name: String,
    pub(crate) from_node: NodeId,
    pub(crate) to_node: NodeId,
    /// Conditions must ALL match, so there's an implicit `&&`
    /// operator between any pair of conditions.
    pub(crate) conditions: Vec<Condition>,
}

impl Edge {
    pub fn transition(&self, node_id: NodeId, neighbors: &StateMap) -> Option<NodeId> {
        (self.from_node == node_id
            && self
                .conditions
                .iter()
                .all(|cond| cond.is_satisfied(neighbors)))
        .then_some(self.to_node)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub(crate) left: Value,
    pub(crate) operand: Operand,
    pub(crate) right: Value,
}

impl Condition {
    fn is_satisfied(&self, neighbors: &StateMap) -> bool {
        let left = self.left.to_absolute(neighbors);
        let right = self.right.to_absolute(neighbors);
        self.operand.compare(left, right)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
// #[serde(tag = "type", content = "val")]
pub enum Value {
    Absolute(u32),
    PopulationCount(NodeId),
}

impl Value {
    #[inline]
    fn to_absolute(self, neighbors: &StateMap) -> u32 {
        match self {
            Value::Absolute(abs) => abs,
            Value::PopulationCount(node_id) => neighbors.get_count(node_id) as u32,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Operand {
    /// `==`
    Equal,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `!=`
    Different,
}

impl Operand {
    #[inline]
    pub fn compare(self, left: u32, right: u32) -> bool {
        match self {
            Operand::Equal => left == right,
            Operand::Greater => left > right,
            Operand::GreaterOrEqual => left >= right,
            Operand::Less => left < right,
            Operand::LessOrEqual => left <= right,
            Operand::Different => left != right,
        }
    }
}
