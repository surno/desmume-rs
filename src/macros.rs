macro_rules! impl_read_access (
    ($for_type:ident, $integer_type:ident, $read_fn:ident) => (
        impl<'a> MemoryAccess<$integer_type, false> for $for_type<'a, $integer_type> {
            unsafe fn read_range(&self, start: u32, end: u32) -> Vec<$integer_type> {
                let size_of = std::mem::size_of::<$integer_type>();
                assert_eq!(0, (end as usize - start as usize + 1) % size_of);
                (start..end).step_by(size_of).map(|a| $read_fn(a as c_int) as $integer_type).collect()
            }

            unsafe fn read(&self, addr: u32) -> $integer_type {
                $read_fn(addr as i32) as $integer_type
            }

            unsafe fn write_range(&self, _start: u32, _end: u32, _source: &[$integer_type]) {
                unreachable!()
            }

            unsafe fn write(&self, _addr: u32, _value: $integer_type) {
                unreachable!()
            }
        }
    );
);

macro_rules! impl_write_access (
    ($for_type:ident, $integer_type:ident, $as_unsigned:ident, $read_fn:ident, $write_fn:ident) => (
        impl<'a> MemoryAccess<$integer_type, true> for $for_type<'a, $integer_type> {
            unsafe fn read_range(&self, start: u32, end: u32) -> Vec<$integer_type> {
                let size_of = std::mem::size_of::<$integer_type>();
                assert_eq!(0, (end as usize - start as usize + 1) % size_of);
                (start..end).step_by(size_of).map(|a| $read_fn(a as c_int) as $integer_type).collect()
            }

            unsafe fn read(&self, addr: u32) -> $integer_type {
                $read_fn(addr as i32) as $integer_type
            }

            unsafe fn write_range(&self, start: u32, end: u32, source: &[$integer_type]) {
                let size_of = std::mem::size_of::<$integer_type>();
                assert_eq!(0, (end as usize - start as usize + 1) % size_of);
                assert_eq!((end as usize - start as usize + 1) / size_of, source.len());
                for (addr, value) in std::iter::zip((start..end).step_by(size_of), source) {
                    $write_fn(addr as c_int, *value as $as_unsigned)
                }
            }

            unsafe fn write(&self, addr: u32, value: $integer_type) {
                $write_fn(addr as c_int, value as $as_unsigned)
            }
        }
    );
);

