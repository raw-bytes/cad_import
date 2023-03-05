use std::sync::atomic::{AtomicU64, Ordering};

pub type ID = u64;

pub struct IDCounter {
    counter: AtomicU64,
}

impl IDCounter {
    /// Returns a new initialized ID counter
    pub const fn new() -> Self {
        Self {
            counter: AtomicU64::new(0u64),
        }
    }

    /// Generates and returns a new ID.
    pub fn gen(&self) -> ID {
        let id = self.counter.fetch_add(1u64, Ordering::SeqCst);
        id
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_counter() {
        let id_counter = IDCounter::new();

        assert_eq!(id_counter.gen(), 0);
        assert_eq!(id_counter.gen(), 1);
        assert_eq!(id_counter.gen(), 2);
        assert_eq!(id_counter.gen(), 3);
    }
}