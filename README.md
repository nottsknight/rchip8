# rCHIP-8

A CHIP-8 interpreter/emulator written in Rust. Currently the interpreter only
runs on UNIX terminals because of the method used to control the display.

## Running

The emulator is built and run as normal using Cargo. The executable takes one
mandatory argument which is the path to the ROM file to run.

## Included ROMs

The ROMs in the `roms` directory came from the following sources:
- `bc-test.ch8`: <https://github.com/cj1128/chip8-emulator/blob/master/rom/BC_test.ch8>
- `chip8-test-rom.ch8`: <https://github.com/corax89/chip8-test-rom/blob/master/test_opcode.ch8>
- `ibm-logo.ch8`: <https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8>

