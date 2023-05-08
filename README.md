# rCHIP-8

A CHIP-8 interpreter/emulator written in Rust. Currently the interpreter only
runs on UNIX terminals because of the method used to control the display.

## Executables

This crate provides two executables:

- `rchip8` is the main emulator program that runs CHIP-8 ROMs
- `c8asc` is an assembler for the language described in `c8asc.md`

Both executables provide help when run with the `-h` flag, reproduced below:

## rchip8
```
Usage: rchip8 [OPTIONS] <ROM_FILE>

Arguments:
  <ROM_FILE>  Path to the ROM file to run

Options:
  -o, --original     Run in original mode
  -a, --addresses    Output addresses when disassembling (starting at 0x200)
  -d, --disassemble  Disassemble the ROM instead of executing it
  -h, --help         Print help
  -V, --version      Print version
```

## c8asc
```
Usage: c8asc [OPTIONS] <FILE>

Arguments:
  <FILE>  Assembly file to compile

Options:
  -o <OUTFILE>      Name of ROM file to generate [default: a.out]
  -h, --help        Print help
 ```
