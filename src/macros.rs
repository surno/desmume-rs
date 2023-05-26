macro_rules! impl_read_write_access (
    ($for_type:ident, $integer_type:ident, $as_unsigned:ident, $read_fn:ident, $write_fn:ident) => (
        impl<M> MemoryReadAccess<$integer_type> for $for_type<M, $integer_type> where M: std::convert::AsRef<crate::mem::DeSmuMEMemory> {
            fn read_range(&self, start: u32, end: u32) -> Vec<$integer_type> {
                let size_of = std::mem::size_of::<$integer_type>();
                assert_eq!(0, (end as usize - start as usize + 1) % size_of);
                (start..end).step_by(size_of).map(|a| unsafe { $read_fn(a as c_int) as $integer_type }).collect()
            }

            fn read(&self, addr: u32) -> $integer_type {
                unsafe { $read_fn(addr as i32) as $integer_type }
            }
        }

        impl<M> MemoryWriteAccess<$integer_type> for $for_type<M, $integer_type> where M: std::convert::AsRef<crate::mem::DeSmuMEMemory> {
            fn write_range(&mut self, start: u32, end: u32, source: &[$integer_type]) {
                let size_of = std::mem::size_of::<$integer_type>();
                assert_eq!(0, (end as usize - start as usize + 1) % size_of);
                assert_eq!((end as usize - start as usize + 1) / size_of, source.len());
                for (addr, value) in std::iter::zip((start..end).step_by(size_of), source) {
                    unsafe { $write_fn(addr as c_int, *value as $as_unsigned) }
                }
            }

            fn write(&mut self, addr: u32, value: $integer_type) {
                unsafe { $write_fn(addr as c_int, value as $as_unsigned) }
            }
        }
    );
);
