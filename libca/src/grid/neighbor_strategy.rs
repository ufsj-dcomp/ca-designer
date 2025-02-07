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
            NeighboringStrategy::Square => IndexIter::new(&[
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_sub(self.cells_per_row),
                index.checked_add(self.cells_per_row),
            ]),
            NeighboringStrategy::SquareAndCorners => IndexIter::new(&[
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_sub(self.cells_per_row),
                index.checked_sub(self.cells_per_row - 1),
                index.checked_sub(self.cells_per_row + 1),
                index.checked_add(self.cells_per_row),
                index.checked_add(self.cells_per_row - 1),
                index.checked_add(self.cells_per_row + 1),
            ]),
            NeighboringStrategy::Hexagon => IndexIter::new(&[
                index.checked_sub(1),
                index.checked_add(1),
                index.checked_sub(self.cells_per_row),
                index.checked_sub(self.cells_per_row - 1),
                index.checked_add(self.cells_per_row),
                index.checked_add(self.cells_per_row - 1),
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

// #[cfg(test)]
// mod tests {
//     use crate::grid::test_utils::game_of_life_grid;

//     #[rstest]
//     #[case(
//         NeighboringStrategy::SquareAndCorners,
//         "
//             ░░░░
//             ░██░
//             ░██░
//             ░░░░
//         ",
//         5,
//         [0, 1, 2, 4, 6, 7, 8, 9]
//     )]
//     fn iter_neighbors_returns_expected_results(
//         #[case] strategy: NeighboringStrategy,
//         #[case] states: &'static str,
//         #[case] idx: usize,
//         #[case] expected: [usize],
//     ) {
//         let grid = game_of_life_grid(states);
//     }
// }
