use crate::mem::{IndexMove, IndexSet};
use desmume_sys::*;
use std::marker::PhantomData;
use std::ops::{
    Deref, DerefMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// Numeric data types that can be read from / written into NDS memory.
pub trait MemType: Sized + Copy {}
impl MemType for u8 {}
impl MemType for u16 {}
impl MemType for u32 {}
impl MemType for i8 {}
impl MemType for i16 {}
impl MemType for i32 {}

const START_OF_MEMORY: u32 = 0;
const END_OF_MEMORY: u32 = 0xFFFFFFFF; // todo: is this true?

/// Trait for accessing memory. You probably don't want to use this, use the `IndexMove` trait
/// instead, if available. See [`TypedMemoryAccessor`].
///
/// This trait is an implementation detail and not meant to be implemented from other crates.
trait MemoryReadAccess<T: MemType> {
    /// Read a part of memory. `end - size + 1` must be a multiple of the size of `T`.
    fn read_range(&self, start: u32, end: u32) -> Vec<T>;
    /// Read a single value from memory.
    fn read(&self, addr: u32) -> T;
}

/// Trait for accessing memory. You probably don't want to use this, use the `IndexMove`, `IndexSet`
/// traits instead, if available. See [`TypedMemoryAccessor`].
///
/// This trait is an implementation detail and not meant to be implemented from other crates.
trait MemoryWriteAccess<T: MemType> {
    /// Write a part of memory. `end - size + 1` must be a multiple of the size of `T`.
    /// `source` must have the size `(end - size + 1) / std::mem::size_of<T>()`.
    fn write_range(&mut self, start: u32, end: u32, source: &[T]);
    /// Write a single value to memory. This should be equivalent to `self.write_range(starttart + std::mem::size_of<T>() - 1, &[value])`.
    fn write(&mut self, addr: u32, value: T);
}

/// A reader/writer over the NDS memory (that can also read). It can be indexed since it implements
/// [`IndexMove`] and [`IndexSet`] (via [`MemIndexWrapper`] for compiler-compatibility reasons). It is indexed by
/// using `u32`'s or ranges over `u32`s that address a specific space in the NDS memory. The value
/// returned is the data at those memory locations in the format specified by `T` (eg. `u8`, `i16`, `u32`, etc.).
pub struct TypedMemoryAccessor<M, T: MemType>(pub(crate) M, pub(crate) PhantomData<T>);

impl_read_write_access!(
    TypedMemoryAccessor,
    u8,
    u8,
    desmume_memory_read_byte,
    desmume_memory_write_byte
);
impl_read_write_access!(
    TypedMemoryAccessor,
    u16,
    u16,
    desmume_memory_read_short,
    desmume_memory_write_short
);
impl_read_write_access!(
    TypedMemoryAccessor,
    u32,
    c_ulong,
    desmume_memory_read_long,
    desmume_memory_write_long
);
impl_read_write_access!(
    TypedMemoryAccessor,
    i8,
    u8,
    desmume_memory_read_byte_signed,
    desmume_memory_write_byte
);
impl_read_write_access!(
    TypedMemoryAccessor,
    i16,
    u16,
    desmume_memory_read_short_signed,
    desmume_memory_write_short
);
impl_read_write_access!(
    TypedMemoryAccessor,
    i32,
    c_ulong,
    desmume_memory_read_long_signed,
    desmume_memory_write_long
);

/// A tiny wrapper to work around Rust's orphan rules limitations for the Index/IndexMut implementations of the readers and writers.
/// Not pretty, but you can pretty much just full-transparently ignore this type. See [`TypedMemoryAccessor`] instead.
pub struct MemIndexWrapper<T, U>(pub(crate) T, pub(crate) PhantomData<U>);
impl<T, U> Deref for MemIndexWrapper<T, U> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, U> DerefMut for MemIndexWrapper<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: MemType, U> IndexMove<u32> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = T;

    fn index_move(&self, index: u32) -> Self::Output {
        self.read(index)
    }
}

impl<T: MemType, U> IndexMove<Range<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, index: Range<u32>) -> Self::Output {
        self.read_range(index.start, index.end - 1)
    }
}

impl<T: MemType, U> IndexMove<RangeFrom<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, index: RangeFrom<u32>) -> Self::Output {
        self.read_range(index.start, END_OF_MEMORY)
    }
}

impl<T: MemType, U> IndexMove<RangeFull> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, _index: RangeFull) -> Self::Output {
        self.read_range(START_OF_MEMORY, END_OF_MEMORY)
    }
}

impl<T: MemType, U> IndexMove<RangeInclusive<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, index: RangeInclusive<u32>) -> Self::Output {
        self.read_range(*index.start(), *index.end())
    }
}

impl<T: MemType, U> IndexMove<RangeTo<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, index: RangeTo<u32>) -> Self::Output {
        self.read_range(START_OF_MEMORY, index.end - 1)
    }
}

impl<T: MemType, U> IndexMove<RangeToInclusive<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T>,
{
    type Output = Vec<T>;

    fn index_move(&self, index: RangeToInclusive<u32>) -> Self::Output {
        self.read_range(START_OF_MEMORY, index.end)
    }
}

impl<T: MemType, U> IndexSet<u32> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: u32, value: &Self::Output) {
        self.write(index, *value)
    }
}

impl<T: MemType, U> IndexSet<Range<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: Range<u32>, value: &Self::Output) {
        self.write_range(index.start, index.end - 1, value)
    }
}

impl<T: MemType, U> IndexSet<RangeFrom<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: RangeFrom<u32>, value: &Self::Output) {
        self.write_range(index.start, END_OF_MEMORY, value)
    }
}

impl<T: MemType, U> IndexSet<RangeFull> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, _index: RangeFull, value: &Self::Output) {
        self.write_range(START_OF_MEMORY, END_OF_MEMORY, value)
    }
}

impl<T: MemType, U> IndexSet<RangeInclusive<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: RangeInclusive<u32>, value: &Self::Output) {
        self.write_range(*index.start(), *index.end(), value)
    }
}

impl<T: MemType, U> IndexSet<RangeTo<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: RangeTo<u32>, value: &Self::Output) {
        self.write_range(START_OF_MEMORY, index.end - 1, value)
    }
}

impl<T: MemType, U> IndexSet<RangeToInclusive<u32>> for MemIndexWrapper<U, T>
where
    U: MemoryReadAccess<T> + MemoryWriteAccess<T>,
{
    fn index_set(&mut self, index: RangeToInclusive<u32>, value: &Self::Output) {
        self.write_range(START_OF_MEMORY, index.end, value)
    }
}
