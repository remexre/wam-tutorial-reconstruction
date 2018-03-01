use std::borrow::Borrow;

/// An environment for an arbitrary type.
#[derive(Clone, Debug)]
pub struct Env<K: Eq, V> {
    vals: Vec<(K, V)>,
}

impl<K: Eq, V> Env<K, V> {
    /// Creates a new, empty Env.
    pub fn new() -> Env<K, V> {
        Env { vals: Vec::new() }
    }

    /// Checks if the given key is contained within the environment.
    pub fn contains<T: Borrow<K>>(&self, k: T) -> bool {
        self.get(k).is_some()
    }

    /// Checks if the environment is empty.
    pub fn empty(&self) -> bool {
        self.vals.is_empty()
    }

    /// Adds a value to the environment.
    pub fn push(&mut self, k: K, v: V) {
        self.vals.push((k, v));
    }

    /// Removes `n` values from the environment.
    pub fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.vals.pop().unwrap();
        }
    }

    /// Retrieves a value with a specific key from the environment.
    pub fn get<T: Borrow<K>>(&self, k: T) -> Option<&V> {
        let k = k.borrow();
        for &(ref k2, ref v) in self.vals.iter().rev() {
            if k == k2 {
                return Some(v);
            }
        }
        None
    }

    /// Runs the given closure in a `let`-`in`-like construct, such that the
    /// binding exists within the closure but not outside of it.
    pub fn with<F, U>(&mut self, k: K, v: V, f: F) -> U
    where
        F: FnOnce(&mut Env<K, V>) -> U,
    {
        self.push(k, v);
        let u = f(self);
        self.pop(1);
        u
    }
}
