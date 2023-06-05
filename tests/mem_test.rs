use rs_desmume::mem::{IndexMove, IndexSet, Processor, Register};
use rs_desmume::DeSmuME;
use std::env::current_dir;
use std::sync::atomic::{AtomicBool, Ordering};

static PRINT_ADDR_CHECK: AtomicBool = AtomicBool::new(false);

extern "C" fn print_addr(addr: u32, size: i32) -> i32 {
    assert_eq!(addr, 0x200000e);
    assert_eq!(size, 2);
    PRINT_ADDR_CHECK.store(true, Ordering::Relaxed);
    1
}

#[test]
fn test_memory_and_hooks() {
    let mut emu = DeSmuME::init().unwrap();

    let rom_path = current_dir().unwrap().join("tests/touchtest.nds");

    let rom_path_str = rom_path.to_str().unwrap();

    emu.open(rom_path_str, false).unwrap();
    emu.resume(false);

    for _ in 0..1000 {
        emu.cycle();
    }

    let pc = emu.memory().get_reg(Processor::Arm9, Register::PC);
    assert_eq!(pc, 0x2001140);

    let at40 = emu.memory().u8().index_move(0x2000000);
    assert_eq!(at40, 1);
    emu.memory().u8().index_set(0x2000000, &76);
    let at40 = emu.memory().u8().index_move(0x2000000);
    assert_eq!(at40, 76);
    emu.memory().u8().index_set(0x2000000, &1);
    let at40 = emu.memory().u8().index_move(0x2000000);
    assert_eq!(at40, 1);

    let at40range = 0x2000000..0x2000010;
    let at40slice = emu.memory().u8().index_move(at40range.clone());
    assert_eq!(at40slice.len(), 16);
    assert_eq!(
        &at40slice,
        &[1, 3, 160, 227, 8, 2, 128, 229, 156, 17, 159, 229, 193, 15, 128, 226]
    );
    let at40replacement = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0xF];
    emu.memory()
        .u8()
        .index_set(at40range.clone(), &at40replacement);

    let at40slice = emu.memory().u8().index_move(at40range.clone());
    assert_eq!(at40slice.len(), 16);
    assert_eq!(&at40slice, &at40replacement);

    // TODO: Tests for other sizes.

    // Test hook
    emu.memory_mut()
        .register_read(0x200000E, 2, Some(print_addr));
    let _ = emu.memory().u16().index_move(at40range.clone());
    assert!(PRINT_ADDR_CHECK.load(Ordering::Relaxed))
}
