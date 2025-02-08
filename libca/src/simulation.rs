use rayon::iter::IntoParallelRefMutIterator;

use crate::{grid::Grid, model::Model, state_map::StateMap};

pub struct SimulationContext {
    model: Model,
    grid: Grid,
    state_pool: Vec<StateMap>,
}

impl SimulationContext {
    pub fn new(model: Model, grid: Grid) -> Self {
        let n_cells = grid.n_cells();
        let mut state_pool = Vec::with_capacity(n_cells);

        for _ in 0..state_pool.capacity() {
            state_pool.push(StateMap::new());
        }

        Self {
            model,
            grid,
            state_pool,
        }
    }

    pub fn step(&mut self) {
        self.grid
            .map_cells(self.state_pool.par_iter_mut(), |curr_state, state_map| {
                self.model.next_state(curr_state, state_map)
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grid::test_utils::{game_of_life_grid, to_game_of_life_output},
        model::test_utils::game_of_life_rules,
    };

    #[test]
    fn gol_block_should_remain() {
        let model = game_of_life_rules();
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
        let model = game_of_life_rules();
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
        let model = game_of_life_rules();
        let grid = game_of_life_grid(include_str!("../fixtures/gol/glider.txt"));

        let mut ctx = SimulationContext::new(model, grid);

        for _ in 0..32 {
            ctx.step();
        }

        insta::assert_snapshot!(to_game_of_life_output(&ctx.grid));
    }
}
