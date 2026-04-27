use arbitrary_int::u2;
use squaregb::Machine;
use squaregb::R8::*;
use squaregb::Tile;
use squaregb::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_MAP_BASE, TILE_MAP_WIDTH, VIDEO_RAM_BASE};

#[test]
fn it_adds_two_integers() {
    let mut machine = Machine::new();

    let binary = [0b00_000_110, 0x06, 0b00_111_110, 0x04, 0b1000_0000];

    // Something like this.
    machine.set_memory(0x0, &binary);

    machine.set_pc(0x0000);

    machine.eval(1);
    assert_eq!(machine.get_r8(B), 6);
    machine.eval(1);
    assert_eq!(machine.get_r8(A), 4);
    machine.eval(1);
    assert_eq!(machine.get_r8(A), 10);
}

#[test]
fn it_runs_boot_rom() {
    // TODO: Expects LCD Y register to be updated during execution.
    // See https://gbdev.io/pandocs/STAT.html

    let mut machine = Machine::new();

    // Nintendo boot rom. (DMG0)
    let binary = [
        0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff,
        0x0e, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e,
        0xfc, 0xe0, 0x47, 0x21, 0x04, 0x01, 0xe5, 0x11, 0xcb, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0x6b,
        0x23, 0x7d, 0xfe, 0x34, 0x20, 0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86,
        0x20, 0x5a, 0xd1, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0xa9, 0x00, 0xcd, 0xaa, 0x00, 0x13, 0x7b,
        0xfe, 0x34, 0x20, 0xf3, 0x3e, 0x18, 0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x32, 0x3d, 0x28, 0x09,
        0x0d, 0x20, 0xf9, 0x11, 0xec, 0xff, 0x19, 0x18, 0xf1, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42,
        0x3e, 0x91, 0xe0, 0x40, 0x04, 0x1e, 0x02, 0xcd, 0xbc, 0x00, 0x0e, 0x13, 0x24, 0x7c, 0x1e,
        0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06, 0x7b, 0xe2, 0x0c, 0x3e,
        0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15, 0x20, 0xdd, 0x05, 0x20, 0x69, 0x16, 0x20,
        0x18, 0xd6, 0x3e, 0x91, 0xe0, 0x40, 0x1e, 0x14, 0xcd, 0xbc, 0x00, 0xf0, 0x47, 0xee, 0xff,
        0xe0, 0x47, 0x18, 0xf3, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
        0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20,
        0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2, 0xc9, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00,
        0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89,
        0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e,
        0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0xff, 0xff, 0x3c, 0xe0,
        0x50,
    ];

    machine.set_memory(0x0, &binary);
    machine.set_pc(0x000);

    // Shouldn't panic
    machine.run_until_pc(0x1000, 100_000);
}

#[test]
fn ppu_renders_black_background() {
    let mut machine = Machine::new();

    let tile_data: [u8; 16] = [0x00; 16];

    machine.set_memory(VIDEO_RAM_BASE, &tile_data as &[u8]);

    let screen_data: [u2; SCREEN_WIDTH * SCREEN_HEIGHT] = machine.ppu_render_screen(0);
    let black = u2::new(0);
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pixel = screen_data[(SCREEN_WIDTH * y) + x];
            assert_eq!(pixel, black);
        }
    }
}

#[test]
fn ppu_renders_white_background() {
    let mut machine = Machine::new();

    let tile_data: [u8; 16] = [0xFF; 16];

    let tile_map_data = [0x00; 256];

    machine.set_memory(VIDEO_RAM_BASE, &tile_data as &[u8]);
    machine.set_memory(TILE_MAP_BASE, &tile_map_data);

    let screen_data: [u2; SCREEN_WIDTH * SCREEN_HEIGHT] = machine.ppu_render_screen(0);
    let white = u2::new(3);
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pixel = screen_data[(SCREEN_WIDTH * y) + x];
            assert_eq!(pixel, white);
        }
    }
}

#[test]
fn ppu_renders_tiling_checkerbox() {
    let mut machine = Machine::new();

    let white_tile: [u8; 16] = [0xFF; 16];
    let black_tile: [u8; 16] = [0x00; 16];

    machine.set_memory(VIDEO_RAM_BASE, &black_tile as &[u8]);
    machine.set_memory(VIDEO_RAM_BASE + Tile::BYTE_SIZE, &white_tile as &[u8]);

    for x in 0..32 {
        for y in 0..32 {
            let tile_map_index: usize = (y * TILE_MAP_WIDTH) + x;
            let tile_index: usize = (x + y) % 2;
            let tile_index: u8 = tile_index.try_into().unwrap();
            machine.set_memory(TILE_MAP_BASE + tile_map_index, &[tile_index]);
        }
    }

    let screen_data: [u2; SCREEN_WIDTH * SCREEN_HEIGHT] = machine.ppu_render_screen(0);
    let white = u2::new(3);
    let black = u2::new(0);
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pixel = screen_data[(SCREEN_WIDTH * y) + x];

            let expected_color = if ((x / 8) + (y / 8)) % 2 == 0 {
                black
            } else {
                white
            };

            assert_eq!(pixel, expected_color, "At pixel ({x},{y})");
        }
    }
}

#[test]
fn ppu_renders_tiling_checkerbox_offset() {
    let mut machine = Machine::new();

    let white_tile: [u8; 16] = [0xFF; 16];
    let black_tile: [u8; 16] = [0x00; 16];

    machine.set_memory(VIDEO_RAM_BASE, &black_tile as &[u8]);
    machine.set_memory(VIDEO_RAM_BASE + Tile::BYTE_SIZE, &white_tile as &[u8]);

    for x in 0..32 {
        for y in 0..32 {
            let tile_map_index: usize = (y * TILE_MAP_WIDTH) + x;
            let tile_index: usize = (x + y) % 2;
            let tile_index: u8 = tile_index.try_into().unwrap();
            machine.set_memory(TILE_MAP_BASE + tile_map_index, &[tile_index]);
        }
    }

    let screen_data: [u2; SCREEN_WIDTH * SCREEN_HEIGHT] = machine.ppu_render_screen(8);
    let white = u2::new(3);
    let black = u2::new(0);
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let pixel = screen_data[(SCREEN_WIDTH * y) + x];

            let expected_color = if ((x / 8) + (y / 8)) % 2 == 0 {
                white
            } else {
                black
            };

            assert_eq!(pixel, expected_color, "At pixel ({x},{y})");
        }
    }
}
