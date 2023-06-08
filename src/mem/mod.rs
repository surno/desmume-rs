mod index;
mod read;

pub use crate::mem::index::{IndexMove, IndexSet};
pub use crate::mem::read::{MemIndexWrapper, TypedMemoryAccessor};
pub use desmume_sys::MemoryCbFnc;
use desmume_sys::*;
use std::ffi::CString;
use std::marker::PhantomData;

pub enum Processor {
    Arm9,
    Arm7,
}

impl Processor {
    fn get_name(&self) -> &str {
        match self {
            Processor::Arm9 => "arm9",
            Processor::Arm7 => "arm7",
        }
    }
}

#[non_exhaustive]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    CPSR,
    SPSR,
    SP, // Alias for R13
    LR, // Alias for R14
    PC, // Alias for R15
}

impl Register {
    fn get_name(&self) -> &str {
        match self {
            Self::R0 => "r0",
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::R3 => "r3",
            Self::R4 => "r4",
            Self::R5 => "r5",
            Self::R6 => "r6",
            Self::R7 => "r7",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
            Self::CPSR => "cpsr",
            Self::SPSR => "spsr",

            Self::SP => "r13",
            Self::LR => "r14",
            Self::PC => "r15",
        }
    }
}

/// Try from register number. If number is not a valid register, errors.
impl TryFrom<u32> for Register {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            8 => Ok(Self::R8),
            9 => Ok(Self::R9),
            10 => Ok(Self::R10),
            11 => Ok(Self::R11),
            12 => Ok(Self::R12),
            13 => Ok(Self::R13),
            14 => Ok(Self::R14),
            15 => Ok(Self::R15),
            _ => Err(()),
        }
    }
}

/// Access and manipulate the memory of the emulator.
pub struct DeSmuMEMemory(pub(crate) PhantomData<*mut u8>);

impl DeSmuMEMemory {
    /// Allows reading the memory using a &\[u8]-like type. Please note that reading memory always copies.
    ///
    /// Use the [`IndexMove`] trait to access the returned data type.
    ///
    /// At the time of writing there is no syntactic sugar that allows indexing to return non-references,
    /// so we have implemented [`IndexMove`] from https://github.com/rust-lang/rfcs/issues/997.
    ///
    /// # Usage example
    /// ```rs
    /// use rs_desmume::DeSmuMEMemory;
    /// use rs_desmume::mem::index::IndexMove;
    ///
    /// fn example(mem: DeSmuMEMemory) {
    ///     let a: u8 = mem.u8().index_move(123);
    ///     let b: Vec<u8> = mem.u8().index_move(123..456);
    /// }
    /// ```
    pub fn u8(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, u8>, u8> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u8]-like type.
    ///
    /// Use the [`IndexSet`] and [`IndexMove`] traits to access the returned data type.
    /// At the time of writing there is no syntactic sugar that allows indexing to return non-references,
    /// so we have implemented [`IndexSet`] and [`IndexMove`] from https://github.com/rust-lang/rfcs/issues/997.
    ///
    /// # Usage example
    /// ```rs
    /// use rs_desmume::DeSmuMEMemory;
    /// use rs_desmume::mem::index::{IndexMove, IndexSet};
    ///
    /// fn example(mem: DeSmuMEMemory) {
    ///     let a: u8 = mem.u8_mut().index_move(123);
    ///     let b: Vec<u8> = mem.u8().index_move(100..200);
    ///     mem.u8_mut().index_set(456, &a);
    ///     mem.u8_mut().index_set(500..600, &b);
    /// }
    /// ```
    pub fn u8_mut(&mut self) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, u8>, u8> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[u16]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn u16(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, u16>, u16> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u16]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn u16_mut(
        &mut self,
    ) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, u16>, u16> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[u32]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn u32(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, u32>, u32> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[u32]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn u32_mut(
        &mut self,
    ) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, u32>, u32> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i8]-like type. Please note that reading memory always copies.
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i8(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, i8>, i8> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i8]-like type.
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i8_mut(&mut self) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, i8>, i8> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i16]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i16(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, i16>, i16> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i16]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 2 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i16_mut(
        &mut self,
    ) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, i16>, i16> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows reading the memory using a &\[i32]-like type. Please note that reading memory always copies.
    /// The returned type is indexed using normal memory addresses (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8`] for info on how to use this type.
    pub fn i32(&self) -> MemIndexWrapper<TypedMemoryAccessor<&DeSmuMEMemory, i32>, i32> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Allows writing to the memory using a &mut \[i32]-like type.
    /// The returned type is indexed using normal memory addresses
    /// (so the size indexed by MUST be a multiple of 4 if you use ranges).
    ///
    /// See [`DeSmuMEMemory::u8_mut`] for info on how to use this type.
    pub fn i32_mut(
        &mut self,
    ) -> MemIndexWrapper<TypedMemoryAccessor<&mut DeSmuMEMemory, i32>, i32> {
        MemIndexWrapper(TypedMemoryAccessor(self, PhantomData), PhantomData)
    }

    /// Reads a CString (\0 terminated) starting at the given memory location.
    pub fn read_cstring(&self, start: u32) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(64);
        let mut addr = start as i32;
        let mut cur_byte = unsafe { desmume_memory_read_byte(addr) };
        while cur_byte != 0 {
            buffer.push(cur_byte);
            addr += 1;
            cur_byte = unsafe { desmume_memory_read_byte(addr) };
        }
        // SAFETY: cur_byte was never added to buffer when it was 0.
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub fn get_reg(&self, processor: Processor, reg: Register) -> u32 {
        let mut bytes = format!("{}.{}", processor.get_name(), reg.get_name()).into_bytes();
        bytes.push(0);
        let mut cchars = bytes
            .into_iter()
            .map(|b| b as c_char)
            .collect::<Vec<c_char>>();
        unsafe { desmume_memory_read_register(cchars.as_mut_ptr()) }
    }

    pub fn set_reg(&mut self, processor: Processor, reg: Register, value: u32) {
        let mut bytes = format!("{}.{}", processor.get_name(), reg.get_name()).into_bytes();
        bytes.push(0);
        let mut cchars = bytes
            .into_iter()
            .map(|b| b as c_char)
            .collect::<Vec<c_char>>();
        unsafe { desmume_memory_write_register(cchars.as_mut_ptr(), value) }
    }

    pub fn get_next_instruction(&self) -> u32 {
        unsafe { desmume_memory_get_next_instruction() }
    }

    pub fn set_next_instruction(&mut self, value: u32) {
        unsafe { desmume_memory_set_next_instruction(value) }
    }

    /// Add a memory callback for when the memory at the specified address was changed.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_write(&mut self, address: u32, size: u16, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_write(address as c_int, size as c_int, callback) }
    }

    /// Add a memory callback for when the memory at the specified address was read.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_read(&mut self, address: u32, size: u16, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_read(address as c_int, size as c_int, callback) }
    }

    /// Add a memory callback for when the memory at the specified address was read.
    ///
    /// Setting a callback will override the previously registered one for this address.
    /// Set callback to None, to remove the callback for this address.
    ///
    /// `size` is the maximum size that will be watched. If you set this to 4 for example,
    ///  a range of (address, address + 3) will be monitored.
    pub fn register_exec(&mut self, address: u32, callback: MemoryCbFnc) {
        unsafe { desmume_memory_register_exec(address as c_int, 2, callback) }
    }
}

impl AsRef<DeSmuMEMemory> for DeSmuMEMemory {
    fn as_ref(&self) -> &DeSmuMEMemory {
        self
    }
}

impl AsMut<DeSmuMEMemory> for DeSmuMEMemory {
    fn as_mut(&mut self) -> &mut DeSmuMEMemory {
        self
    }
}
