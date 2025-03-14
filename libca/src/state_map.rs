use crate::{grid::Grid, model::NodeId};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

const DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED: usize = 64;

pub struct StatePool(Vec<StateMap>);

impl StatePool {
    pub fn new(grid: &Grid) -> Self {
        let n_cells = grid.n_cells();
        let mut state_pool = Vec::with_capacity(n_cells);

        for _ in 0..state_pool.capacity() {
            state_pool.push(StateMap::new());
        }

        Self(state_pool)
    }

    pub fn get(&self, idx: usize) -> &StateMap {
        let idx = idx % self.0.len();
        &self.0[idx]
    }
}

#[derive(Debug)]
pub struct StateMap {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl StateMap {
    pub fn new() -> Self {
        let (ptr, len, cap) = vec![0; DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED].into_raw_parts();
        Self { ptr, len, cap }
    }

    pub fn count_states(&self, states: impl Iterator<Item = NodeId>) {
        // # Safety
        // This is safe so long as:
        // 1. `self.ptr` is not null
        // 2. `self.ptr + self.len - 1` is indexable
        // This should hold true so long as `self.ptr` and `self.len` are only
        // attributed to once
        let slice = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };

        // Clear vec; all states have 0 count
        slice.par_iter_mut().for_each(|entry| *entry = 0);

        states
            .into_iter()
            .map(NodeId::as_index)
            .for_each(|idx| slice[idx] += 1);
    }

    #[inline]
    pub fn get_count(&self, state: NodeId) -> u8 {
        let idx = state.as_index();
        assert!(
            idx < self.len,
            "Tried indexing a larger state than we had space for"
        );

        // # Safety
        // This is safe so long as:
        // 1. `self.ptr` is not null
        //  - This is the case considering we only ever write to `self.ptr` once,
        //    from a `Vec::into_raw_parts` call
        // 2. `state` is less than `self.len`
        //  - This is the assertion above
        unsafe { *self.ptr.add(state.as_index()) }
    }

    pub const fn default_size() -> usize {
        size_of::<Self>() * DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED
    }
}

// # Safety
// StateMap should only be used to collect temporary data. For such purposes,
// it features interior mutability
unsafe impl Send for StateMap {}

unsafe impl Sync for StateMap {}

impl Default for StateMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StateMap {
    fn drop(&mut self) {
        // # Safety
        // This assumes `self.ptr`, `self.len` and `self.cap`
        // were handed off from a `Vec::into_raw_parts`
        unsafe {
            let _ = Vec::from_raw_parts(self.ptr, self.len, self.cap);
        }
    }
}
