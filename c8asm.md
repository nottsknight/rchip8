# c8asm language

## General semantics
All numbers are hexadecimal and this is the only accepted format.

Registers are written as `Vx` where `x` is a single hexadecimal digit. The
index register is written as `I`. Although not registers, the sound and delay
timers are accessed like registers using the names `S` and `D` respectively.
Literal values are given with a `#` in front, like `#1a`.

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
| 8xy4   | `add     Vx, Vy   ` |
| 8xy5   | `sub     Vx, Vy   ` |
| 8xy6   | `rshift  Vx, Vy   ` |
| 8xy7   | `subr    Vx, Vy   ` |
| 8xye   | `lshift  Vx, Vy   ` |
| 9xy0   | `skipne  Vx, Vy   ` |
| annn   | `mov     I, nnn   ` |
| bnnn   | `jmpv    nnn      ` |
| cxnn   | `rand    Vx, nn   ` |
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

## Named locations

Lines of code can be labelled, to enable jumps and calls to refer to them by
name instead of having to calculate what their final address will be. A label 
consists of a `$`, followed by a single letter, followed by any number of 
additional letters, numbers, or underscores. A label can then be supplied as the 
operand to a `jmp`, `call`, or `jmpv` instruction in place of a literal value.

## Data

Data can be placed directly into the file, for example to encode custom sprites.
Lines of data begin with the token `data:` followed by any number of pairs of
hex digits.

Example: `data: 2fa1334fc7`

## Example programs

The following program computes the first `n` natural numbers where `n` is given
by the user.

```
          input    V0
          mov      V2, #1
$loop     add      V1, V0
          sub      V0, V2
          skipeq   V0, #0
          jmp      $loop
          mov      I, #300
          bcd      V1
          load     V2
          mov      Va, #5
          mov      Vb, #5
          sprite   V0
          draw     Va, Vb, #5
          add      Va, #5
          sprite   V1
          draw     Va, Vb, #5
          add      Va, #5
          sprite   V2
          draw     Va, Vb, #5
$end      jmp      $end
```

This program prints out all the characters in the embedded font:

```
       mov     Vb, #23
       jmp     $loop
$loop  call    $print 
       add     V0, #1
       add     V1, #5
       mov     Va, V1
       subr    Va, Vb
       skipeq  Vf, #1
       mov     V1, #0
       skipeq  Vf, #1
       add     V2, #7
       skipeq  V0, #10 
       jmp     $loop
$end   jmp     $end
$print sprite  V0
       draw    V1, V2, #5
       retn
```
