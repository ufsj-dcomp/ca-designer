use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    Frame,
};

use crate::widgets::{Navbar, StatefulModal};

use super::ModalMessage;

const CURSOR: char = '█';

pub struct AddNodeModal {
    name: String,
}

impl AddNodeModal {
    pub fn new() -> Self {
        Self {
            name: CURSOR.to_string(),
        }
    }

    fn draw_navbar(area: Rect, ctx: &mut Frame) {
        const KEYS: &[(&str, &str)] = &[
            (" ↑ ", " Up "),
            (" ↓ ", " Down "),
            (" ⤶ ", " Submit "),
            (" Esc ", " Cancel "),
        ];

        Navbar::draw(KEYS, area, ctx);
    }
}

impl StatefulModal for AddNodeModal {
    type Message = super::ModalMessage;

    fn title(&self) -> &str {
        "Add Node"
    }

    fn key_press(&mut self, key: KeyCode) -> Option<Self::Message> {
        match key {
            KeyCode::Char(c) => {
                let text = &mut self.name;
                let idx = text
                    .char_indices()
                    .last()
                    .map(|(idx, _)| idx)
                    .unwrap_or_default();
                text.insert(idx, c); // Inserts before the cursor
            }
            KeyCode::Backspace => {
                let text = &mut self.name;

                if text.len() > 1 {
                    text.pop();
                    text.pop();
                    text.push(CURSOR);
                }
            }
            KeyCode::Enter => {
                self.name.pop(); // Removes trailing cursor
                let mut node_name = String::new();
                std::mem::swap(&mut node_name, &mut self.name);
                return Some(ModalMessage::AddNode(libca::Node::new(node_name)));
            }
            _ => {}
        };

        None
    }

    fn draw_inner(&self, area: Rect, ctx: &mut Frame) {
        const VERTICAL_CONSTRAINTS: [Constraint; 2] =
            [Constraint::Length(1), Constraint::Length(1)];

        const HORIZONTAL_CONSTRINATS: [Constraint; 2] = [Constraint::Fill(3), Constraint::Fill(10)];

        let [input_area, navbar_area] = Layout::vertical(VERTICAL_CONSTRAINTS).areas(area);

        let title_style = Style::new().gray().bold();
        let input_style = Style::new().white().reversed();

        let [title_area, input_area] = Layout::horizontal(HORIZONTAL_CONSTRINATS).areas(input_area);

        let title = Text::raw("Name: ").style(title_style);
        let input = Text::raw(&self.name).style(input_style);

        ctx.render_widget(title, title_area);
        ctx.render_widget(input, input_area);

        Self::draw_navbar(navbar_area, ctx);
    }
}
