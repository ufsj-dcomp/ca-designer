use super::{
    edge::{Condition, Edge, EdgeId, Operand, Value},
    node::Node,
    Model, NodeId,
};

pub fn game_of_life_rules() -> Model {
    let dead = NodeId(0);
    let alive = NodeId(1);

    Model {
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
