use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Styled, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Tabs, Widget},
    Frame,
};
use strum::{VariantArray, VariantNames};

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
        const VERTICAL_CONSTRAINTS: [Constraint; 3] = [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ];

        let [header_area, main_area, navbar_area] =
            Layout::vertical(VERTICAL_CONSTRAINTS).areas(ctx.area());

        match self.current_tab {
            Tab::Model => self.model_tab.draw(&self.model, main_area, ctx),
            Tab::Graph => todo!(),
            Tab::Simulation => todo!(),
        }

        self.draw_header(header_area, ctx);
        Self::draw_navbar(navbar_area, ctx);
    }

    pub fn handle_key_press(&mut self, key_ev: KeyEvent) -> bool {
        match key_ev.code {
            key_code @ KeyCode::Esc => {
                return match self.current_tab {
                    Tab::Model => {
                        if self.model_tab.is_modal_open() {
                            self.model_tab.handle_key_press(key_code, &mut self.model);
                            false
                        } else {
                            true
                        }
                    }
                    Tab::Graph => true,
                    Tab::Simulation => true,
                }
            }
            key_code => match self.current_tab {
                Tab::Model => self.model_tab.handle_key_press(key_code, &mut self.model),
                Tab::Graph => todo!(),
                Tab::Simulation => todo!(),
            },
        };

        false
    }

    fn draw_header(&self, area: Rect, ctx: &mut Frame) {
        const NAMES: &'static [&str] = <Tab as VariantNames>::VARIANTS;
        const TABS: &'static [Tab] = <Tab as VariantArray>::VARIANTS;

        const HORIZONTAL_CONSTRAINTS: [Constraint; 2] = [Constraint::Fill(1); 2];
        let [title_area, tabs_area] = Layout::horizontal(HORIZONTAL_CONSTRAINTS).areas(area);

        ctx.render_widget("CA TUI".bold(), title_area);

        let tabs: Line = TABS
            .iter()
            .zip(NAMES)
            .map(|(tab, name)| {
                let mut span = Span::raw(*name);
                if *tab == self.current_tab {
                    span = span.style(Style::new().yellow());
                }

                span
            })
            .intersperse(Span::raw(" • "))
            .collect::<Line>()
            .right_aligned();

        ctx.render_widget(tabs, tabs_area);
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
#[derive(VariantNames, VariantArray, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Model,
    Graph,
    Simulation,
}
