# c8asm language

## General semantics
All numbers are hexadecimal and this is the only accepted format.

Registers are written as `Vx` where `x` is a single hexadecimal digit. The
index register is written as `I`. Although not registers, the sound and delay
timers are accessed like registers using the names `S` and `D` respectively.

## Instructions

| opcode | instruction         |
| =======|=====================|
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

Data can be placed directly in the assembly. Use the pseudo-instruction `data` followed
by either two or four hex digits. The two groups of two can optionally be separated
by a space.

## Grammar

Prog := Inst Prog
Prog := epsilon

Inst := "mc" LitHex3
Inst := "clr"
Inst := "retn"
Inst := "jmp" LitHex3
Inst := "call" LitHex3
Inst := "skipeq" SkipArgs
Inst := "skipne" SkipArgs
Inst := "mov" Mov
Inst := "add" Add
Inst := "or" RegPair
Inst := "and" RegPair
Inst := "xor" RegPair
Inst := "addc" RegPair
Inst := "sub" RegPair
Inst := "rshift" RegPair
Inst := "subr" RegPair
Inst := "lshift" RegPair
Inst := "jmpv" LitHex3
Inst := "rndmov" RegLit
Inst := "draw" Draw
Inst := "input" GenReg
Inst := "sprite" GenReg
Inst := "bcd" GenReg
Inst := "store" GenReg
Inst := "load" GenReg
Inst := Data 

LitHex4 := r"[0-9a-fA-F]{4}"
LitHex3 := r"[0-9a-fA-F]{3}"
LitHex2 := r"[0-9a-fA-F]{2}"

Data := LitHex4
Data := LitHex2 LitHex2

SkipArgs := RegLit
SkipArgs := RegPair

GenReg := r"V[0-9a-fA-F]"

RegPair := GenReg "," GenReg

RegLit := GenReg "," LitHex2

Mov := GenReg "," MovGenReg
Mov := "D" "," GenReg
Mov := "S" "," GenReg
Mov := "I" "," LitHex3

MovGenReg := LitHex2
MovGenReg := GenReg
MovGenReg := "D"

Add := RegLit
Add := "I" "," LitHex3

Draw := GenReg "," GenReg "," r"[0-9a-fA-F]"

## First and follow

first(Prog) = { "mc", "clr", "retn", "jmp", "call", "skipeq", "skipne", "mov", "add",
    "or", "and", "xor", "addc", "sub", "rshift", "subr", "lshift", "jmpv",
    "rndmov", "draw", "input", "sprite", "bcd", "store", "load", epsilon }

first(Inst) = { "mc", "clr", "retn", "jmp", "call", "skipeq", "skipne", "mov", "add",
    "or", "and", "xor", "addc", "sub", "rshift", "subr", "lshift", "jmpv",
    "rndmov", "draw", "input", "sprite", "bcd", "store", "load" }

first(LitHex2) = [0-9a-fA-F]

first(LitHex3) = [0-9a-fA-F]

first(LitHex4) = [0-9a-fA-F]

first(SkipArgs) = "V"

first(GenReg) = "V"

first(RegPair) = "V"

first(RegLit) = "V"

first(Mov) = { "V", "D", "S", "I" }

first(MovGenReg) = { [0-9a-fA-F], "V", "D" }

first(Add) = { "V", "I" }

first(Draw) = "V"

follow(Prog) = { $ }

follow(Inst) = { "mc", "clr", "retn", "jmp", "call", "skipeq", "skipne", "mov", "add",
            "or", "and", "xor", "addc", "sub", "rshift", "subr", "lshift", "jmpv",
            "rndmov", "draw", "input", "sprite", "bcd", "store", "load", $ }

follow(LitHex3) = follow(Inst)

follow(LitHex2) = follow(Inst)

follow(SkipArgs) = follow(Inst)

follow(Mov) = follow(Inst)

follow(Add) = follow(Inst)

follow(RegPair) = follow(Inst)

follow(Draw) = follow(Inst)

follow(RegLit) = follow(Inst)

    follow(GenReg) = "," u follow(Inst)

follow(MovGenReg) = follow(Inst)
