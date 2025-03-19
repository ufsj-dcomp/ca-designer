mod graphviz_tab;
mod model_tab;
mod simulation_tab;

use crossterm::event::KeyCode;
pub use graphviz_tab::GraphvizTab;
use libca::simulation::SimulationContext;
pub use model_tab::ModelTab;
use ratatui::{layout::Rect, style::Color, Frame};
pub use simulation_tab::SimulationTab;

pub trait Tab {
    fn draw(
        &self,
        simulation_ctx: &SimulationContext,
        colors: &[Color],
        area: Rect,
        ctx: &mut Frame,
    );

    fn handle_key_press(&mut self, key_code: KeyCode, model: &mut libca::Model) {}

    fn is_modal_open(&self) -> bool {
        false
    }
}
