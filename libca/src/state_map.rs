use crate::model::NodeId;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

const DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED: usize = 64;

#[derive(Debug)]
pub struct StateMap(Vec<u8>);

impl StateMap {
    pub fn new() -> Self {
        Self(vec![0; DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED])
    }

    pub fn count_states(&mut self, states: impl Iterator<Item = NodeId>) {
        // Clear vec; all states have 0 count
        self.0.par_iter_mut().for_each(|entry| *entry = 0);

        states
            .into_iter()
            .map(NodeId::as_index)
            .for_each(|idx| self.0[idx] += 1);
    }

    #[inline]
    pub fn get_count(&self, state: NodeId) -> u8 {
        self.0[state.as_index()]
    }

    pub const fn default_size() -> usize {
        size_of::<Self>() * DEFAULT_NUMBER_OF_DIFFERENT_STATES_EXPECTED
    }
}

impl Default for StateMap {
    fn default() -> Self {
        Self::new()
    }
}
