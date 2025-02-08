use std::ops::Rem;

use crate::model::NodeId;

const MAX_NEIGHBORS_PER_CELL: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct NeighboringContext {
    pub(super) cells_per_row: usize,
    pub(super) strategy: NeighboringStrategy,
}

impl NeighboringContext {
    fn get_neighbors(&self, index: usize) -> IndexIter {
        match self.strategy {
            #[allow(unreachable_code)]
            NeighboringStrategy::Square => IndexIter::new(&[
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_sub(self.cells_per_row),
                index.checked_add(self.cells_per_row),
                todo!("Evaluate edge cases"),
            ]),
            NeighboringStrategy::SquareAndCorners => {
                let col = dbg!(dbg!(index).rem(self.cells_per_row));
                let allow_left_side = col > 0;
                let allow_right_side = col < self.cells_per_row - 1;

                IndexIter::new(&[
                    allow_left_side.then(|| index.checked_sub(1)).flatten(),
                    allow_right_side.then(|| index.checked_add(1)).flatten(),
                    index.checked_sub(self.cells_per_row),
                    allow_right_side
                        .then(|| index.checked_sub(self.cells_per_row - 1))
                        .flatten(),
                    allow_left_side
                        .then(|| index.checked_sub(self.cells_per_row + 1))
                        .flatten(),
                    index.checked_add(self.cells_per_row),
                    allow_left_side
                        .then(|| index.checked_add(self.cells_per_row - 1))
                        .flatten(),
                    allow_right_side
                        .then(|| index.checked_add(self.cells_per_row + 1))
                        .flatten(),
                ])
            }
            #[allow(unreachable_code)]
            NeighboringStrategy::Hexagon => IndexIter::new(&[
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_sub(self.cells_per_row),
                index.checked_sub(self.cells_per_row - 1),
                index.checked_add(self.cells_per_row),
                index.checked_add(self.cells_per_row - 1),
                todo!("Evaluate edge cases"),
            ]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NeighboringStrategy {
    Square,
    SquareAndCorners,
    Hexagon,
}

struct IndexIter {
    curr: usize,
    indexes: [Option<usize>; MAX_NEIGHBORS_PER_CELL],
}

impl IndexIter {
    pub fn new(list: &[Option<usize>]) -> Self {
        let mut indexes = [None; MAX_NEIGHBORS_PER_CELL];
        list.iter()
            .filter(|l| l.is_some())
            .zip(indexes.iter_mut())
            .for_each(|(l, n)| *n = *l);

        Self { curr: 0, indexes }
    }
}

impl Iterator for IndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.indexes.get(self.curr)?;
        self.curr += 1;
        *element
    }
}

pub trait IterNeighbors {
    fn iter_neighbors(
        &self,
        idx: usize,
        strategy: NeighboringContext,
    ) -> impl Iterator<Item = NodeId>;
}

impl IterNeighbors for Vec<NodeId> {
    fn iter_neighbors(
        &self,
        idx: usize,
        n_ctx: NeighboringContext,
    ) -> impl Iterator<Item = NodeId> {
        let neighbor_idxs = n_ctx.get_neighbors(idx);
        neighbor_idxs
            .into_iter()
            .flat_map(|idx| self.get(idx))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        NeighboringContext{
            cells_per_row: 4,
            strategy: NeighboringStrategy::SquareAndCorners,
        },
        5,
        &[0, 1, 2, 4, 6, 8, 9, 10]
    )]
    #[case(
        NeighboringContext{
            cells_per_row: 3,
            strategy: NeighboringStrategy::SquareAndCorners,
        },
        6,
        &[3, 4, 7, 9, 10]
    )]
    #[case(
        NeighboringContext{
            cells_per_row: 3,
            strategy: NeighboringStrategy::SquareAndCorners,
        },
        8,
        &[4, 5, 7, 10, 11]
    )]
    fn iter_neighbors_returns_expected_results(
        #[case] neighbor_ctx: NeighboringContext,
        #[case] idx: usize,
        #[case] expected_indexes: &[usize],
    ) {
        let mut actual_indexes: Vec<_> = neighbor_ctx.get_neighbors(idx).collect();
        actual_indexes.sort();

        assert_eq!(actual_indexes, expected_indexes);
    }
}
