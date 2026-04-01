use crate::moves::Move;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Bound {
    Exact = 0,
    Lower = 1,
    Upper = 2,
}

#[derive(Copy, Clone)]
pub struct TTEntry {
    pub key: u32,
    pub best_move: Move,
    pub score: i16,
    pub depth: i8,
    pub bound: Bound,
    pub age: u8,
}

impl Default for TTEntry {
    fn default() -> Self {
        TTEntry {
            key: 0,
            best_move: Move::NULL,
            score: 0,
            depth: -1,
            bound: Bound::Exact,
            age: 0,
        }
    }
}

pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    mask: usize,
    generation: u8,
}

impl TranspositionTable {
    /// Create a new transposition table with the given size in MB.
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<TTEntry>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;
        // Round down to power of 2
        let num_entries = num_entries.next_power_of_two() >> 1;
        let num_entries = num_entries.max(1024);

        TranspositionTable {
            entries: vec![TTEntry::default(); num_entries],
            mask: num_entries - 1,
            generation: 0,
        }
    }

    #[inline]
    fn index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }

    #[inline]
    fn verify_key(hash: u64) -> u32 {
        (hash >> 32) as u32
    }

    /// Probe the transposition table.
    pub fn probe(&self, hash: u64) -> Option<&TTEntry> {
        let idx = self.index(hash);
        let entry = &self.entries[idx];
        if entry.key == Self::verify_key(hash) && entry.depth >= 0 {
            Some(entry)
        } else {
            None
        }
    }

    /// Store a position in the transposition table.
    pub fn store(
        &mut self,
        hash: u64,
        best_move: Move,
        score: i32,
        depth: i32,
        bound: Bound,
    ) {
        let idx = self.index(hash);
        let key = Self::verify_key(hash);
        let entry = &mut self.entries[idx];

        // Replace if: new entry has higher depth, or existing entry is from older generation
        if entry.depth < 0 || entry.age != self.generation || depth as i8 >= entry.depth {
            *entry = TTEntry {
                key,
                best_move,
                score: score.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
                depth: depth as i8,
                bound,
                age: self.generation,
            };
        }
    }

    /// Increment generation counter (call at start of each search).
    pub fn new_generation(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// Clear the table.
    pub fn clear(&mut self) {
        self.entries.fill(TTEntry::default());
        self.generation = 0;
    }
}
