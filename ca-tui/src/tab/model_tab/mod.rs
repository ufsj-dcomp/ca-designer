mod add_node;

use add_node::AddNodeModal;
use crossterm::event::KeyCode;
use libca::{NodeId, Operand};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders},
    Frame,
};
use strum::IntoStaticStr;

use crate::widgets::{Navbar, StatefulModal};

use super::Tab;

const MAIN_AREA_AND_NAVBAR_CONSTRAINTS: [Constraint; 2] =
    [Constraint::Min(0), Constraint::Length(1)];

pub enum ModalMessage {
    AddNode(libca::Node),
    AddEdge(libca::Edge),
    Close,
}

pub struct ModelTab {
    curr_panel: Panel,
    curr_node_row: usize,
    curr_edge_row: usize,
    curr_condition_row: usize,
    add_modal: Option<Box<dyn StatefulModal<Message = ModalMessage>>>,
}

impl Tab for ModelTab {
    fn is_modal_open(&self) -> bool {
        self.add_modal.is_some()
    }

    fn handle_key_press(&mut self, key_code: KeyCode, model: &mut libca::Model) {
        let (row, upper_bound, dependencies): (&mut _, _, &mut [_]) = match self.curr_panel {
            Panel::Nodes => (
                &mut self.curr_node_row,
                model.nodes().len(),
                &mut [&mut self.curr_edge_row, &mut self.curr_condition_row],
            ),
            Panel::Edges => (
                &mut self.curr_edge_row,
                model
                    .nodes()
                    .nth(self.curr_node_row)
                    .map(|(node_id, _)| model.edges_from_node(node_id).count())
                    .unwrap_or_default(),
                &mut [&mut self.curr_condition_row],
            ),
            Panel::Conditions => (&mut self.curr_condition_row, 0, &mut []),
        };

        match key_code {
            KeyCode::Esc if self.add_modal.is_some() => self.add_modal = None,
            key if let Some(add_modal) = &mut self.add_modal => {
                if let Some(message) = add_modal.key_press(key) {
                    match message {
                        ModalMessage::AddNode(node) => model.add_node(node),
                        ModalMessage::AddEdge(edge) => {
                            model.add_edge(edge);
                        }
                        ModalMessage::Close => self.add_modal = None,
                    };

                    self.add_modal = None;
                }
            }
            KeyCode::Right => self.curr_panel.cycle_right(),
            KeyCode::Left => self.curr_panel.cycle_left(),
            KeyCode::Up => {
                *row = row.checked_sub(1).unwrap_or_default();
                dependencies.iter_mut().for_each(|dep| **dep = 0);
            }
            KeyCode::Down => {
                *row = row.checked_add(1).unwrap_or_default().min(upper_bound - 1);
                dependencies.iter_mut().for_each(|dep| **dep = 0);
            }
            KeyCode::Char('a') => self.add_modal = Some(self.curr_panel.add_modal(model)),
            KeyCode::Char('D') => match self.curr_panel {
                Panel::Nodes => {
                    if let Some(node_id) = self.current_node(model) {
                        model.delete_node(*node_id);
                        let current_node_count = model.nodes().len();
                        if self.curr_node_row >= current_node_count {
                            self.curr_node_row = current_node_count - 1;
                        }
                    }
                }
                Panel::Edges => todo!(),
                Panel::Conditions => todo!(),
            },
            _ => {}
        };
    }

    fn draw(&mut self, model: &libca::Model, area: Rect, ctx: &mut Frame) {
        let mut horizontal_layout = [
            Constraint::Fill(2),
            Constraint::Fill(2),
            Constraint::Fill(2),
        ];

        // Current pane should look bigger
        horizontal_layout[self.curr_panel as usize] = Constraint::Fill(3);

        let [node_area, edge_area, condition_area] =
            Layout::horizontal(horizontal_layout).areas(area);

        self.draw_nodes(model, node_area, ctx);
        self.draw_edges(model, edge_area, ctx);
        self.draw_conditions(model, condition_area, ctx);

        if let Some(add_modal) = &self.add_modal {
            add_modal.draw(ctx);
        }
    }
}

impl ModelTab {
    pub fn new() -> Self {
        Self {
            curr_panel: Panel::Nodes,
            curr_node_row: 0,
            curr_edge_row: 0,
            curr_condition_row: 0,
            add_modal: None,
        }
    }

    pub fn current_node<'m>(&self, model: &'m libca::Model) -> Option<&'m NodeId> {
        model
            .nodes()
            .nth(self.curr_node_row)
            .map(|(node_id, _)| node_id)
    }

    pub fn current_edge<'m>(&self, model: &'m libca::Model) -> Option<&'m libca::Edge> {
        self.current_node(model)
            .and_then(|node_id| model.edges_from_node(node_id).nth(self.curr_edge_row))
    }

    fn draw_nodes(&self, model: &libca::Model, area: Rect, ctx: &mut Frame) {
        let block = Panel::Nodes.block(&self.curr_panel);
        let content_area = block.inner(area);

        let [main_area, navbar_area] =
            Layout::vertical(MAIN_AREA_AND_NAVBAR_CONSTRAINTS).areas(content_area);
        let layout = Layout::vertical(model.nodes().map(|_| Constraint::Length(3)));

        ctx.render_widget(block, area);
        model
            .nodes()
            .map(|(_, node)| NodeWrapper(node))
            .zip(layout.split(main_area).iter())
            .enumerate()
            .for_each(|(idx, (node, area))| node.draw(idx == self.curr_node_row, *area, ctx));

        if self.curr_panel == Panel::Nodes {
            Self::draw_navbar(navbar_area, ctx);
        }
    }

    fn draw_edges(&self, model: &libca::Model, area: Rect, ctx: &mut Frame) {
        let block = Panel::Edges.block(&self.curr_panel);
        let content_area = block.inner(area);

        let [main_area, navbar_area] =
            Layout::vertical(MAIN_AREA_AND_NAVBAR_CONSTRAINTS).areas(content_area);

        let Some(current_node_id) = self.current_node(model) else {
            return;
        };

        let layout = Layout::vertical(
            model
                .edges_from_node(current_node_id)
                .map(|_| Constraint::Fill(1)),
        );

        ctx.render_widget(block, area);

        model
            .edges_from_node(current_node_id)
            .map(EdgeWrapper)
            .zip(layout.split(main_area).iter())
            .enumerate()
            .for_each(|(idx, (edge, area))| {
                edge.draw(idx == self.curr_edge_row, model, *area, ctx)
            });

        if self.curr_panel == Panel::Edges {
            Self::draw_navbar(navbar_area, ctx);
        }
    }

    fn draw_conditions(&self, model: &libca::Model, area: Rect, ctx: &mut Frame) {
        let block = Panel::Conditions.block(&self.curr_panel);
        let content_area = block.inner(area);

        ctx.render_widget(block, area);

        let [main_area, navbar_area] =
            Layout::vertical(MAIN_AREA_AND_NAVBAR_CONSTRAINTS).areas(content_area);

        let Some(edge) = self.current_edge(model) else {
            return;
        };

        let layout = Layout::vertical(edge.conditions().iter().map(|_| Constraint::Fill(1)));
        edge.conditions()
            .iter()
            .map(ConditionWrapper)
            .zip(layout.split(main_area).iter())
            .enumerate()
            .for_each(|(idx, (condition, area))| {
                condition.draw(idx, self.curr_condition_row, model, *area, ctx)
            });

        if self.curr_panel == Panel::Conditions {
            Self::draw_navbar(navbar_area, ctx);
        }
    }

    fn draw_navbar(area: Rect, ctx: &mut Frame) {
        const KEYS: &[(&str, &str)] =
            &[(" a ", " Add "), (" D ", " Remove "), (" ↑↓ ", " Select ")];

        Navbar::draw(KEYS, area, ctx);
    }
}

#[repr(usize)]
#[derive(IntoStaticStr, PartialEq, Eq, Clone, Copy)]
enum Panel {
    #[strum(serialize = " States ")]
    Nodes,
    #[strum(serialize = " Transitions ")]
    Edges,
    #[strum(serialize = " Conditions ")]
    Conditions,
}

impl Panel {
    fn block(&self, curr_panel: &Self) -> Block {
        let style = Style::new();
        let style = if self == curr_panel {
            style.white()
        } else {
            style.dark_gray()
        };

        let title: &'static str = self.into();
        Block::bordered().style(style).title(Line::raw(title))
    }

    fn cycle_right(&mut self) {
        *self = match self {
            Panel::Nodes => Panel::Edges,
            Panel::Edges => Panel::Conditions,
            Panel::Conditions => Panel::Nodes,
        };
    }

    fn cycle_left(&mut self) {
        *self = match self {
            Panel::Nodes => Panel::Conditions,
            Panel::Edges => Panel::Nodes,
            Panel::Conditions => Panel::Edges,
        };
    }

    fn add_modal(&self, model: &libca::Model) -> Box<dyn StatefulModal<Message = ModalMessage>> {
        match self {
            Panel::Nodes => Box::new(AddNodeModal::new()),
            Panel::Edges => todo!(),
            Panel::Conditions => todo!(),
        }
    }
}

struct NodeWrapper<'n>(&'n libca::Node);

impl NodeWrapper<'_> {
    fn draw(&self, is_current: bool, area: Rect, ctx: &mut Frame) {
        let style = if is_current {
            Style::new().blue()
        } else {
            Style::new().white()
        };

        let block = Block::bordered()
            .borders(Borders::TOP | Borders::BOTTOM)
            .style(style);

        let content_area = block.inner(area);
        ctx.render_widget(block, area);
        ctx.render_widget(Line::raw(self.0.name()).style(style), content_area);
    }
}

struct EdgeWrapper<'e>(&'e libca::Edge);

impl EdgeWrapper<'_> {
    fn draw(&self, is_current: bool, model: &libca::Model, area: Rect, ctx: &mut Frame) {
        let style = if is_current {
            Style::new().blue()
        } else {
            Style::new().white()
        };

        let Some(to_node) = model.get_node(self.0.to_node_id()) else {
            return;
        };
        let block = Block::bordered()
            .borders(Borders::TOP | Borders::BOTTOM)
            .style(style);

        let content_area = block.inner(area);
        ctx.render_widget(block, area);

        let mut text = Text::raw(self.0.name()).style(style);
        text.push_line(format!("Transitions to: {}", to_node.name()));

        ctx.render_widget(text, content_area);
    }
}

struct ConditionWrapper<'c>(&'c libca::Condition);

impl ConditionWrapper<'_> {
    fn draw(
        &self,
        idx: usize,
        selected_idx: usize,
        model: &libca::Model,
        area: Rect,
        ctx: &mut Frame,
    ) {
        let prefix = if idx == 0 { "When" } else { "And" };

        let style = if idx == selected_idx {
            Style::new().blue()
        } else {
            Style::new().white()
        };

        use libca::Value::*;
        let op = self.0.operand;

        let text = match (self.0.left(), self.0.right()) {
            (Absolute(l), Absolute(r)) => {
                let evaluated = if op.evaluate(l, r) { "always" } else { "never" };
                let rendered_op: &'static str = op.into();
                format!("{prefix} {l} {rendered_op} {r} ({evaluated})")
            }
            (PopulationCount(l_node_id), PopulationCount(r_node_id)) => {
                let (before, after) = operand_to_binary_comparison_texts(op);
                let l_node_name = node_id_to_name(l_node_id, model);
                let r_node_name = node_id_to_name(r_node_id, model);
                format!("{prefix} there are {before} {l_node_name} neighbors {after} there are {r_node_name} neighbors")
            }
            (PopulationCount(node_id), Absolute(abs))
            | (Absolute(abs), PopulationCount(node_id)) => {
                let node_name = node_id_to_name(node_id, model);
                let op_text = operand_to_simple_comparison_text(op);
                format!("{prefix} there are {op_text} {abs} {node_name} neighbors")
            }
        };

        let text = Text::raw(text).style(style);
        ctx.render_widget(text, area);
    }
}

/// Returns prefix and suffix form
const fn operand_to_binary_comparison_texts(op: Operand) -> (&'static str, &'static str) {
    match op {
        Operand::Equal => ("as many", "as"),
        Operand::Greater => ("more", "than"),
        Operand::GreaterOrEqual => ("as many or more", "than"),
        Operand::Less => ("fewer", "than"),
        Operand::LessOrEqual => ("as many or fewer", "than"),
        Operand::Different => ("not as many", "as"),
    }
}

const fn operand_to_simple_comparison_text(op: Operand) -> &'static str {
    match op {
        Operand::Equal => "exactly",
        Operand::Greater => "over",
        Operand::GreaterOrEqual => "exactly or over",
        Operand::Less => "fewer than",
        Operand::LessOrEqual => "exactly or fewer than",
        Operand::Different => "not",
    }
}

fn node_id_to_name<'m>(node_id: &libca::NodeId, model: &'m libca::Model) -> &'m str {
    model
        .get_node(node_id)
        .map(|node| node.name())
        .unwrap_or("?")
}
