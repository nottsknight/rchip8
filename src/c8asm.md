# c8asm language

## General semantics
All numbers are hexadecimal and this is the only accepted format.

Registers are written as `Vx` where `x` is a single hexadecimal digit. The
index register is written as `I`. Although not registers, the sound and delay
timers are accessed like registers using the names `S` and `D` respectively.

## Instructions

| opcode | instruction       |
| =======|===================|
| 0x0nnn | mc      nnn       |
| 0x00e0 | clr               |
| 0x00ee | retn              |
| 0x1nnn | jmp     nnn       |
| 0x2nnn | call    nnn       |
| 0x3xnn | skipeq  Vx, nn    |
| 0x4xnn | skipne  Vx, nn    |
| 0x5xy0 | skipeq  Vx, Vy    |
| 0x6xnn | mov     Vx, nn    |
| 0x7xnn | add     Vx, nn    |
| 0x8xy0 | mov     Vx, Vy    |
| 0x8xy1 | or      Vx, Vy    |
| 0x8xy2 | and     Vx, Vy    |
| 0x8xy3 | xor     Vx, Vy    |
| 0x8xy4 | addc    Vx, Vy    |
| 0x8xy5 | sub     Vx, Vy    |
| 0x8xy6 | rshift  Vx, Vy    |
| 0x8xy7 | subr    Vx, Vy    |
| 0x8xye | lshift  Vx, Vy    |
| 0x9xy0 | skipne  Vx, Vy    |
| 0xannn | mov     I, nnn    |
| 0xbnnn | jmpv    nnn       |
| 0xcxnn | rndmov  Vx, nn    |
| 0xdxyn | draw    Vx, Vy, n |
| 0xex9e | skipkeq Vx        |
| 0xexa1 | skipkne Vx        |
| 0xfx07 | mov     Vx, D     |
| 0xfx0a | input   Vx        |
| 0xfx15 | mov     D, Vx     |
| 0xfx18 | mov     S, Vx     |
| 0xfx1e | add     I, Vx     |
| 0xfx29 | sprite  Vx        |
| 0xfx33 | bcd     Vx        |
| 0xfx55 | store   Vx        |
| 0xfx65 | load    Vx        |

## Data

Data can be placed directly in the assembly. Use the pseudo-instruction `data` followed
by either two or four hex digits. The two groups of two can optionally be separated
by a space.

## Grammar

S := "clr" | "retn" | J
M := "mov" M1
M1 := V "," V
      | V "," N
      | "I" "," N
N := [0-9a-f]N | epsilon
V := "V1" | "V2" | "V3" | "V4" | "V5" | "V6" | "V7" | "V8" | "V9"
     | "Va" | "Vb" | "Vc" | "Vd" | "Ve" | "Vf"
