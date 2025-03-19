use libca::{grid::Grid, simulation::SimulationContext};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    Frame,
};

use super::Tab;

const HALF_BLOCK: &str = "▄";

pub struct SimulationTab {
    constraints: Vec<Constraint>,
}

impl SimulationTab {
    pub fn new(grid: &Grid) -> anyhow::Result<Self> {
        let height = grid.n_rows();
        let constraints = vec![Constraint::Length(1); height];

        Ok(Self { constraints })
    }
}

impl Tab for SimulationTab {
    fn draw(
        &self,
        simulation_ctx: &SimulationContext,
        colors: &[Color],
        area: Rect,
        ctx: &mut Frame,
    ) {
        let layout = Layout::vertical(&self.constraints);
        let sub_areas = layout.split(area);

        simulation_ctx
            .grid
            .cells()
            .chunks(simulation_ctx.grid.cells_per_row())
            .map_windows(|[upper_line, lower_line]| {
                upper_line
                    .iter()
                    .zip(lower_line.iter())
                    .map(|(upper_state, lower_state)| {
                        HALF_BLOCK
                            .bg(colors[upper_state.as_index()])
                            .fg(colors[lower_state.as_index()])
                    })
                    .collect::<Line>()
            })
            .zip(sub_areas.iter())
            .for_each(|(line, area)| ctx.render_widget(line, *area));
    }
}
