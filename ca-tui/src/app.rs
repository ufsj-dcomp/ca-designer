use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};
use strum::{VariantArray, VariantNames};

use crate::{
    tab::{GraphvizTab, ModelTab, Tab},
    widgets::Navbar,
};

pub struct App {
    model: libca::Model,
    tab: Box<dyn Tab>,
    current_tab: TabType,
}

impl App {
    pub fn new() -> Self {
        Self {
            model: libca::Model::game_of_life(),
            tab: Box::new(ModelTab::new()),
            current_tab: TabType::Model,
        }
    }

    pub fn draw(&mut self, ctx: &mut Frame) {
        const VERTICAL_CONSTRAINTS: [Constraint; 3] = [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ];

        let [header_area, main_area, navbar_area] =
            Layout::vertical(VERTICAL_CONSTRAINTS).areas(ctx.area());

        self.tab.draw(&self.model, main_area, ctx);

        self.draw_header(header_area, ctx);
        Self::draw_navbar(navbar_area, ctx);
    }

    pub fn handle_key_press(&mut self, key_ev: KeyEvent) -> bool {
        match key_ev.code {
            key_code @ KeyCode::Esc => {
                return match self.current_tab {
                    TabType::Model => {
                        if self.tab.is_modal_open() {
                            self.tab.handle_key_press(key_code, &mut self.model);
                            false
                        } else {
                            true
                        }
                    }
                    TabType::Graph => true,
                    TabType::Simulation => true,
                }
            }
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
                self.tab = match self.current_tab {
                    TabType::Model => Box::new(ModelTab::new()),
                    TabType::Graph => Box::new(GraphvizTab::new(&self.model).unwrap()),
                    TabType::Simulation => todo!(),
                };
            }
            key_code => match self.current_tab {
                TabType::Model => self.tab.handle_key_press(key_code, &mut self.model),
                TabType::Graph => todo!(),
                TabType::Simulation => todo!(),
            },
        };

        false
    }

    fn draw_header(&self, area: Rect, ctx: &mut Frame) {
        const NAMES: &[&str] = <TabType as VariantNames>::VARIANTS;
        const TABS: &[TabType] = <TabType as VariantArray>::VARIANTS;

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
            (" Tab ", " Next Tab "),
            (" ← ", " Prev. Pane "),
            (" → ", " Next Pane "),
            (" Esc ", " Quit "),
        ];

        Navbar::draw(KEYS, area, ctx);
    }
}

#[repr(usize)]
#[derive(VariantNames, VariantArray, Clone, Copy, PartialEq, Eq)]
enum TabType {
    Model,
    Graph,
    Simulation,
}

impl TabType {
    fn next(self) -> TabType {
        match self {
            TabType::Model => TabType::Graph,
            TabType::Graph => TabType::Model,
            TabType::Simulation => todo!(),
        }
    }
}
