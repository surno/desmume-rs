use rs_desmume::mem::{IndexMove, IndexSet, Processor, Register};
use rs_desmume::DeSmuME;
use std::env::current_dir;
use std::thread::sleep;
use std::time::Duration;

extern "C" fn print_addr(addr: u32, size: i32) -> i32 {
    println!("{} : {}", addr, size);
    return 1;
}

#[test]
fn test_running() {
    let mut emu = DeSmuME::init().unwrap();

    loop {
        let rom_path = current_dir().unwrap().join("tests/touchtest.nds");

        let rom_path_str = rom_path.to_str().unwrap();

        emu.open(rom_path_str, false).unwrap();
        emu.resume(false);

        emu.memory_mut()
            .register_read(0x4000000, 0xFFFF, Some(print_addr));
        emu.open(rom_path_str, false).unwrap();

        let mut window = emu.create_sdl_window(true, true).unwrap();

        //while !window.has_quit() {
        for _ in 0..600 {
            window.process_input();
            window.draw();
            emu.cycle();
            println!(
                "PC: {}",
                emu.memory().get_reg(Processor::Arm9, Register::PC)
            );
            println!(
                "0x4000000 as u8: {}",
                emu.memory().u8().index_move(0x4000000)
            );
            println!(
                "0x4000000..+16 as u8: {:?}",
                emu.memory().u8().index_move(0x4000000..0x4000010)
            );
            println!(
                "0x4000000..+16 as u16: {:?}",
                emu.memory().u16().index_move(0x4000000..0x4000010)
            );
            let mut mem_i32 = emu.memory().i32().index_move(0x4000000..0x4000010);
            println!("0x4000000..+16 as i32: {:?}", mem_i32);
            mem_i32[3] += 1;
            emu.memory_mut()
                .i32_mut()
                .index_set(0x4000000..0x4000010, &mem_i32);
            println!(
                "after: 0x4000000 as u8: {}",
                emu.memory().u8().index_move(0x4000000)
            );
            println!(
                "after: 0x4000000..+16 as u8: {:?}",
                emu.memory().u8().index_move(0x4000000..0x4000010)
            );
            println!(
                "after: 0x4000000..+16 as u16: {:?}",
                emu.memory().u16().index_move(0x4000000..0x4000010)
            );
            println!(
                "after: 0x4000000..+16 as i32: {:?}",
                emu.memory().i32().index_move(0x4000000..0x4000010)
            );

            window = emu.create_sdl_window(true, true).unwrap();
            sleep(Duration::new(0, 16666667))
        }
        //println!("Accessing entire mem... stand by.");
        //println!("entire mem: {:?}", emu.memory().u8().index_move(..));
    }
}
