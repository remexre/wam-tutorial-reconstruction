use std::ops::{Index, IndexMut};

/// The register store. Access the values in the registers by indexing.
#[derive(Debug)]
pub struct Env {
    regs: Vec<usize>,
}

impl Env {
    /// Creates a new register store.
    pub fn new() -> Env {
        Env { regs: Vec::new() }
    }

    /// Clears all the values from the registers.
    pub fn clear(&mut self) {
        self.regs.clear()
    }
}

impl Index<usize> for Env {
    type Output = usize;
    fn index(&self, n: usize) -> &usize {
        &self.regs[n]
    }
}

impl IndexMut<usize> for Env {
    fn index_mut(&mut self, n: usize) -> &mut usize {
        while self.regs.len() <= n {
            self.regs.push(0);
        }
        &mut self.regs[n]
    }
}
