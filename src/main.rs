// This file is part of rchip8.
//
// rchip8 is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// rchip8 is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with rchip8.
// If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
mod machine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Chip8Args {
    /// Path to the ROM file to run
    rom_file: String,
    /// If supplied, run in Original mode
    #[arg(long, short)]
    original: bool,
}

fn main() {
    let args = Chip8Args::parse();

    let mode = if args.original {
        machine::Chip8Mode::Original
    } else {
        machine::Chip8Mode::Modern
    };
    let mut vm = machine::Chip8Machine::new(mode);
    match vm.load_program(&args.rom_file) {
        Ok(_) => vm.run(),
        Err(e) => panic!("{:?}", e),
    }
}
