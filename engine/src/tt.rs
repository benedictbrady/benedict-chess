use crate::moves::Move;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

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

/// Pack a TTEntry into two u64 values.
/// Word 0: key(32) | best_move(32)
/// Word 1: score(16) | depth(8) | bound(8) | age(8) | padding(24)
fn pack(entry: &TTEntry) -> [u64; 2] {
    let w0 = ((entry.key as u64) << 32) | (entry.best_move.raw() as u64);
    let w1 = ((entry.score as u16 as u64) << 48)
        | ((entry.depth as u8 as u64) << 40)
        | ((entry.bound as u8 as u64) << 32)
        | ((entry.age as u64) << 24);
    [w0, w1]
}

/// Unpack two u64 values into a TTEntry.
fn unpack(words: [u64; 2]) -> TTEntry {
    let w0 = words[0];
    let w1 = words[1];
    TTEntry {
        key: (w0 >> 32) as u32,
        best_move: Move::from_raw(w0 as u32),
        score: (w1 >> 48) as u16 as i16,
        depth: (w1 >> 40) as u8 as i8,
        bound: match (w1 >> 32) as u8 {
            1 => Bound::Lower,
            2 => Bound::Upper,
            _ => Bound::Exact,
        },
        age: (w1 >> 24) as u8,
    }
}

/// Thread-safe transposition table using lock-free atomic storage.
/// Each entry is stored as two AtomicU64 values. Torn reads are
/// detected by key verification and harmlessly ignored.
pub struct TranspositionTable {
    /// Flat storage: entries[i*2] and entries[i*2+1] form one TT entry.
    entries: Vec<AtomicU64>,
    mask: usize,
    generation: AtomicU8,
}

// SAFETY: The AtomicU64 and AtomicU8 types are inherently Send + Sync.
// The Vec is never resized after construction.
unsafe impl Send for TranspositionTable {}
unsafe impl Sync for TranspositionTable {}

impl TranspositionTable {
    /// Create a new transposition table with the given size in MB.
    pub fn new(size_mb: usize) -> Self {
        // Each logical entry = 2 AtomicU64 = 16 bytes
        let num_entries = (size_mb * 1024 * 1024) / 16;
        // Round down to power of 2
        let num_entries = num_entries.next_power_of_two() >> 1;
        let num_entries = num_entries.max(1024);

        let mut entries = Vec::with_capacity(num_entries * 2);
        for _ in 0..num_entries * 2 {
            entries.push(AtomicU64::new(0));
        }

        TranspositionTable {
            entries,
            mask: num_entries - 1,
            generation: AtomicU8::new(0),
        }
    }

    #[inline]
    fn index(&self, hash: u64) -> usize {
        ((hash as usize) & self.mask) * 2
    }

    #[inline]
    fn verify_key(hash: u64) -> u32 {
        (hash >> 32) as u32
    }

    /// Probe the transposition table. Returns entry by value (lock-free).
    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let idx = self.index(hash);
        let w0 = self.entries[idx].load(Ordering::Relaxed);
        let w1 = self.entries[idx + 1].load(Ordering::Relaxed);
        let entry = unpack([w0, w1]);
        if entry.key == Self::verify_key(hash) && entry.depth >= 0 {
            Some(entry)
        } else {
            None
        }
    }

    /// Store a position in the transposition table (lock-free).
    pub fn store(
        &self,
        hash: u64,
        best_move: Move,
        score: i32,
        depth: i32,
        bound: Bound,
    ) {
        let idx = self.index(hash);
        let key = Self::verify_key(hash);
        let gen = self.generation.load(Ordering::Relaxed);

        // Read existing entry to check replacement policy
        let old_w0 = self.entries[idx].load(Ordering::Relaxed);
        let old_w1 = self.entries[idx + 1].load(Ordering::Relaxed);
        let old = unpack([old_w0, old_w1]);

        // Replace if: empty, older generation, or deeper/equal depth
        if old.depth < 0 || old.age != gen || depth as i8 >= old.depth {
            let entry = TTEntry {
                key,
                best_move,
                score: score.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
                depth: depth as i8,
                bound,
                age: gen,
            };
            let words = pack(&entry);
            self.entries[idx].store(words[0], Ordering::Relaxed);
            self.entries[idx + 1].store(words[1], Ordering::Relaxed);
        }
    }

    /// Increment generation counter (call at start of each search).
    pub fn new_generation(&self) {
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Clear the table.
    pub fn clear(&self) {
        for entry in self.entries.iter() {
            entry.store(0, Ordering::Relaxed);
        }
        self.generation.store(0, Ordering::Relaxed);
    }
}
