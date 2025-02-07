use crate::model::NodeId;

use super::{neighbor_strategy::NeighboringStrategy, Grid};

const IGNORED_CHARS: &[char] = &['\n', ' '];

pub fn grid_from_repr(repr: &'static str, map: impl Fn(char) -> NodeId) -> Grid {
    let height = repr.trim_matches(IGNORED_CHARS).lines().count();

    let clean_repr = repr.chars().filter(|c| !IGNORED_CHARS.contains(c));

    // Clone of iter, not str
    let n_cells = clean_repr.clone().count();
    let cells_per_row = n_cells.div_ceil(height);

    let mut grid = Grid::empty(
        n_cells,
        cells_per_row,
        NeighboringStrategy::SquareAndCorners,
    );

    grid.cells
        .iter_mut()
        .zip(clean_repr)
        .for_each(|(cell, c)| *cell = map(c));

    grid
}

pub fn grid_to_repr<F>(grid: &Grid, n_rows: usize, map: F) -> String
where
    F: FnMut(NodeId) -> char + Clone + Copy,
{
    grid.cells
        .chunks(n_rows)
        .map(|chunk| String::from_iter(chunk.iter().copied().map(map)))
        .collect::<Vec<_>>()
        .join("\n")
}

// ░ 9617
// ▒ 9618
// ▓ 9619
// █ 9608

pub fn game_of_life_grid(repr: &'static str) -> Grid {
    grid_from_repr(repr, |c| match c {
        '░' => NodeId(0),
        '█' => NodeId(1),
        _ => panic!("Unrecognized cell representation '{c}' for Game of Life"),
    })
}

pub fn to_game_of_life_output(grid: &Grid, n_rows: usize) -> String {
    grid_to_repr(grid, n_rows, |state| match state {
        NodeId(0) => '░',
        NodeId(1) => '█',
        _ => panic!("Invalid state NodeId({})", state.0),
    })
}
