mod graphviz_tab;
mod model_tab;

use crossterm::event::KeyCode;
pub use graphviz_tab::GraphvizTab;
pub use model_tab::ModelTab;
use ratatui::{layout::Rect, Frame};

pub trait Tab {
    fn draw(&mut self, model: &libca::Model, area: Rect, ctx: &mut Frame);

    fn handle_key_press(&mut self, key_code: KeyCode, model: &mut libca::Model) {}

    fn is_modal_open(&self) -> bool {
        false
    }
}
