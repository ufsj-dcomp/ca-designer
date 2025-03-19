use crate::{grid::Grid, model::Model, state_map::StatePool};

pub struct SimulationContext {
    pub model: Model,
    pub grid: Grid,
    state_pool: StatePool,
}

impl SimulationContext {
    pub fn new(model: Model, grid: Grid) -> Self {
        Self {
            state_pool: StatePool::new(),
            model,
            grid,
        }
    }

    pub fn step(&mut self) {
        self.grid
            .map_cells(&self.state_pool, |curr_state, state_map| {
                self.model.next_state(curr_state, state_map)
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::test_utils::{game_of_life_grid, to_game_of_life_output};

    #[test]
    fn gol_block_should_remain() {
        let model = Model::game_of_life();
        let grid = game_of_life_grid(
            "
            ░░░░
            ░██░
            ░██░
            ░░░░
        ",
        );

        let mut ctx = SimulationContext::new(model, grid);
        ctx.step();

        insta::assert_snapshot!(to_game_of_life_output(&ctx.grid));
    }

    #[test]
    fn gol_blinker_should_oscillate() {
        let model = Model::game_of_life();
        let grid = game_of_life_grid(
            "
            ░░░
            ░█░
            ░█░
            ░█░
            ░░░
        ",
        );

        let mut ctx = SimulationContext::new(model, grid);
        ctx.step();

        insta::assert_snapshot!("horizontal", to_game_of_life_output(&ctx.grid));
        ctx.step();

        insta::assert_snapshot!("vertical", to_game_of_life_output(&ctx.grid));
    }

    #[test]
    fn gol_glider_should_move_and_cycle() {
        let model = Model::game_of_life();
        let grid = game_of_life_grid(include_str!("../fixtures/gol/glider.txt"));

        let mut ctx = SimulationContext::new(model, grid);

        for _ in 0..32 {
            ctx.step();
        }

        insta::assert_snapshot!(to_game_of_life_output(&ctx.grid));
    }
}
