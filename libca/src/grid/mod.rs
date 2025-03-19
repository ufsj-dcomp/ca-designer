pub mod neighbor_strategy;

#[cfg(test)]
pub mod test_utils;

use neighbor_strategy::{IterNeighbors, NeighboringContext, NeighboringStrategy};
use rand::seq::IndexedRandom;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::{
    model::NodeId,
    state_map::{StateMap, StatePool},
    AVAILABLE_PARALLELISM,
};

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

        (0..self.cells.capacity())
            .map(|_| state_probabilities.choose_weighted(&mut rng, |sp| sp.weight))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|sp| sp.state)
            .collect_into(&mut self.cells);

        Ok(())
    }

    pub fn map_cells<'s, F>(&mut self, state_pool: &'s StatePool, f: F)
    where
        F: Fn(NodeId, &'s StateMap) -> NodeId + Send + Sync,
    {
        let chunk_size = *AVAILABLE_PARALLELISM;
        self.cells
            .chunks(chunk_size)
            .zip(self.next_cells.chunks_mut(chunk_size))
            .enumerate()
            .for_each(|(outer_idx, (cells, next_cells))| {
                cells
                    .par_iter()
                    .enumerate()
                    .zip(next_cells.par_iter_mut())
                    .for_each(|((inner_idx, cell), next_cell)| {
                        let idx = outer_idx * chunk_size + inner_idx;
                        let state_map = state_pool.get(idx);
                        let neighbors = self.cells.iter_neighbors(idx, self.neighbor_ctx);
                        state_map.count_states(neighbors);
                        *next_cell = f(*cell, state_map);
                    });
            });

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    #[inline]
    pub fn n_cells(&self) -> usize {
        self.n_cells
    }

    #[inline]
    pub fn n_rows(&self) -> usize {
        self.n_cells.div_ceil(self.neighbor_ctx.cells_per_row)
    }

    #[inline]
    pub fn cells_per_row(&self) -> usize {
        self.neighbor_ctx.cells_per_row
    }

    #[inline]
    pub fn cells(&self) -> &[NodeId] {
        &self.cells
    }
}

pub struct StateProbabilty {
    pub state: NodeId,
    pub weight: f32,
}
