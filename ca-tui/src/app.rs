use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use strum::VariantNames;

use crate::{tab::ModelTab, widgets::Navbar};

pub struct App {
    model: libca::Model,
    model_tab: ModelTab,
    current_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        Self {
            model: libca::Model::game_of_life(),
            model_tab: ModelTab::new(),
            current_tab: Tab::Model,
        }
    }

    pub fn draw(&self, ctx: &mut Frame) {
        const VERTICAL_CONSTRAINTS: [Constraint; 2] = [Constraint::Min(0), Constraint::Length(1)];

        let [main_area, navbar_area] = Layout::vertical(VERTICAL_CONSTRAINTS).areas(ctx.area());

        match self.current_tab {
            Tab::Model => self.model_tab.draw(&self.model, main_area, ctx),
            Tab::Graph => todo!(),
            Tab::Simulation => todo!(),
        }

        Self::draw_navbar(navbar_area, ctx);
    }

    pub fn handle_key_press(&mut self, key_ev: KeyEvent) -> bool {
        match key_ev.code {
            KeyCode::Esc => return true,
            key_code => match self.current_tab {
                Tab::Model => self.model_tab.handle_key_press(key_code, &self.model),
                Tab::Graph => todo!(),
                Tab::Simulation => todo!(),
            },
        };

        false
    }

    fn draw_navbar(area: Rect, ctx: &mut Frame) {
        const KEYS: &[(&str, &str)] = &[
            (" ← ", " Prev. Pane "),
            (" → ", " Next Pane "),
            (" Esc ", " Quit "),
        ];

        Navbar::draw(KEYS, area, ctx);
    }
}

#[repr(usize)]
#[derive(VariantNames, Clone, Copy)]
enum Tab {
    Model,
    Graph,
    Simulation,
}
