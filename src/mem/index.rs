//! Implementations of IndexMove and IndexSet somewhat in the spirit of
//! <https://github.com/rust-lang/rfcs/issues/997>
//!
//! In the hope one day this gets added to the language with some syntactic sugar.

/// This trait works like [`std::ops::Index`], but instead of returning a reference, it moves
/// a value (probably a copy or smart pointer; in this crate only copies) out of the indexed value.
pub trait IndexMove<Idx: ?Sized> {
    /// The returned type after indexing.
    type Output: ?Sized;

    fn index_move(&self, index: Idx) -> Self::Output;
}

/// This trait works like [`std::ops::IndexMut`], but instead of returning a mutable reference,
/// the value is directly set in the method call and nothing is returned.
/// This is pretty limited in how it is implemented, but good enough for this use case.
pub trait IndexSet<Idx: ?Sized>: IndexMove<Idx> {
    fn index_set(&mut self, index: Idx, value: &Self::Output);
}
