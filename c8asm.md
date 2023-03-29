# c8asm language

## General semantics
All numbers are hexadecimal and this is the only accepted format.

Registers are written as `Vx` where `x` is a single hexadecimal digit. The
index register is written as `I`. Although not registers, the sound and delay
timers are accessed like registers using the names `S` and `D` respectively.

## Instructions

| opcode | instruction         |
|--------|---------------------|
| 0nnn   | `mc      nnn      ` |
| 00e0   | `clr              ` |
| 00ee   | `retn             ` |
| 1nnn   | `jmp     nnn      ` |
| 2nnn   | `call    nnn      ` |
| 3xnn   | `skipeq  Vx, nn   ` |
| 4xnn   | `skipne  Vx, nn   ` |
| 5xy0   | `skipeq  Vx, Vy   ` |
| 6xnn   | `mov     Vx, nn   ` |
| 7xnn   | `add     Vx, nn   ` |
| 8xy0   | `mov     Vx, Vy   ` |
| 8xy1   | `or      Vx, Vy   ` |
| 8xy2   | `and     Vx, Vy   ` |
| 8xy3   | `xor     Vx, Vy   ` |
| 8xy4   | `addc    Vx, Vy   ` |
| 8xy5   | `sub     Vx, Vy   ` |
| 8xy6   | `rshift  Vx, Vy   ` |
| 8xy7   | `subr    Vx, Vy   ` |
| 8xye   | `lshift  Vx, Vy   ` |
| 9xy0   | `skipne  Vx, Vy   ` |
| annn   | `mov     I, nnn   ` |
| bnnn   | `jmpv    nnn      ` |
| cxnn   | `rndmov  Vx, nn   ` |
| dxyn   | `draw    Vx, Vy, n` |
| ex9e   | `skipkeq Vx       ` |
| exa1   | `skipkne Vx       ` |
| fx07   | `mov     Vx, D    ` |
| fx0a   | `input   Vx       ` |
| fx15   | `mov     D, Vx    ` |
| fx18   | `mov     S, Vx    ` |
| fx1e   | `add     I, Vx    ` |
| fx29   | `sprite  Vx       ` |
| fx33   | `bcd     Vx       ` |
| fx55   | `store   Vx       ` |
| fx65   | `load    Vx       ` |

## Data

Data can be placed directly in the assembly. Use the pseudo-instruction `.data` 
followed by four hex digits. The two groups of two can optionally be 
separated by a space.

Examples:

```
.data 2f1a
.data 94 b1
```

## Example program

The following program computes the first `n` natural numbers where `n` is given
by the user.

```
input   V0
mov     V1, 0
skipne  V0, 0
jmp     2a6
addc    V1, V0
sub     V0, 1
jmp     206
jmp     2a6
```
