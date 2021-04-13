pub mod quick_find;
mod quick_union;

pub trait UF {
    fn new(n: usize) -> Self;

    // whose id equals id[p] to id[q].
    fn union(&mut self, p: usize, q: usize);

    // Check if p and q have the same id.
    fn connected(&self, p: usize, q: usize) -> bool;
}



