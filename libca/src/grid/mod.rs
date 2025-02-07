pub mod neighbor_strategy;

#[cfg(test)]
pub mod test_utils;

use neighbor_strategy::{IterNeighbors, NeighboringContext, NeighboringStrategy};
use rand::seq::IndexedRandom;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::{model::NodeId, state_map::StateMap};

#[derive(Debug)]
pub struct Grid {
    neighbor_ctx: NeighboringContext,
    n_cells: usize,
    cells: Vec<NodeId>,
    next_cells: Vec<NodeId>,
}

impl Grid {
    pub fn empty(n_cells: usize, cells_per_row: usize, strategy: NeighboringStrategy) -> Self {
        Self {
            neighbor_ctx: NeighboringContext {
                cells_per_row,
                strategy,
            },
            n_cells,
            cells: vec![Default::default(); n_cells],
            next_cells: vec![Default::default(); n_cells],
        }
    }

    pub fn randomize(&mut self, state_probabilities: &[StateProbabilty]) -> anyhow::Result<()> {
        self.cells.clear();
        let mut rng = rand::rng();

        state_probabilities
            .choose_multiple_weighted(&mut rng, self.n_cells, |sp| sp.weight)?
            .map(|sp| sp.state)
            .collect_into(&mut self.cells);

        Ok(())
    }

    pub fn map_cells<'is, IS, F>(&mut self, state_maps: IS, f: F)
    where
        IS: IndexedParallelIterator<Item = &'is mut StateMap>,
        F: Fn(NodeId, &'is StateMap) -> NodeId + Send + Sync,
    {
        self.cells
            .par_iter()
            .enumerate()
            .zip(self.next_cells.par_iter_mut())
            .zip(state_maps)
            .for_each(|(((idx, cell), next_cell), state_map)| {
                let neighbors = self.cells.iter_neighbors(idx, self.neighbor_ctx);
                state_map.count_states(neighbors);
                *next_cell = f(*cell, state_map);
            });

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn n_cells(&self) -> usize {
        self.n_cells
    }
}

pub struct StateProbabilty {
    state: NodeId,
    weight: f32,
}
