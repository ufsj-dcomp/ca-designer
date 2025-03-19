use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use libca::simulation::SimulationContext;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    DefaultTerminal, Frame,
};
use strum::{VariantArray, VariantNames};
use tokio::sync::Mutex;

use crate::{
    tab::{GraphvizTab, ModelTab, SimulationTab, Tab},
    widgets::Navbar,
};

pub struct App {
    simulation_ctx: Arc<Mutex<SimulationContext>>,
    node_colors: Vec<Color>,
    tab: Box<dyn Tab>,
    current_tab: TabType,
}

impl App {
    pub fn new(simulation_ctx: Arc<Mutex<SimulationContext>>) -> Self {
        Self {
            simulation_ctx,
            node_colors: [Color::Black, Color::White].to_vec(),
            tab: Box::new(ModelTab::new()),
            current_tab: TabType::Model,
        }
    }

    pub async fn draw_to(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        const VERTICAL_CONSTRAINTS: [Constraint; 3] = [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ];

        let sim = self.simulation_ctx.lock().await;
        terminal.draw(|ctx| {
            let [header_area, main_area, navbar_area] =
                Layout::vertical(VERTICAL_CONSTRAINTS).areas(ctx.area());

            self.tab.draw(&sim, &self.node_colors, main_area, ctx);
            self.draw_header(header_area, ctx);
            Self::draw_navbar(navbar_area, ctx);
        })?;

        Ok(())
    }

    pub async fn handle_key_press(&mut self, key_ev: KeyEvent) -> Message {
        let mut sim = self.simulation_ctx.lock().await;
        let model = &mut sim.model;

        match key_ev.code {
            key_code @ KeyCode::Esc => {
                return match self.current_tab {
                    TabType::Model => {
                        if self.tab.is_modal_open() {
                            self.tab.handle_key_press(key_code, model);
                            Message::None
                        } else {
                            Message::CloseApplication
                        }
                    }
                    TabType::Graph | TabType::Simulation => Message::CloseApplication,
                }
            }
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
                self.tab = match self.current_tab {
                    TabType::Model => Box::new(ModelTab::new()),
                    TabType::Graph => Box::new(GraphvizTab::new(model).unwrap()),
                    TabType::Simulation => Box::new(SimulationTab::new(&sim.grid).unwrap()),
                };
            }
            key_code => match self.current_tab {
                TabType::Model => self.tab.handle_key_press(key_code, model),
                TabType::Graph => todo!(),
                TabType::Simulation => todo!(),
            },
        };

        Message::None
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

pub enum Message {
    ResumeSimulation,
    PauseSimulation,
    StepSimulation,
    UpdateModel(libca::Model),
    CloseApplication,
    None,
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
            TabType::Graph => TabType::Simulation,
            TabType::Simulation => TabType::Model,
        }
    }
}
