# CPU plan:

## Physical:

16-bit CPU.

16 GP registers

other registers: PC, SP

1 word instructions

64kb 16-bit RAM

## instructions:

2 modes, immediate always works on R0, and register addressed works on 2 registers, putting the result in the first named register. First bit specifies which mode. 0 = register based, 1 = immediate. If instruction does not have immediate mode it will not execute as immediate

### Arithmetic

NOP
- 00__ -> Does nothing
- 80__ -> Does nothing
ADD
- 01rs -> register r = register r + register s
- 81xx -> register 0 = register 0 + xx (xx in interpreted as uint8)
SUB
- 02rs -> register r = register r - register s
- 82xx -> register 0 = register 0 - xx (xx is interpreted as uint8)
AND (bitwise AND)
- 03rs -> register r = register r && register s
- 83xx -> register 0 = register 0 && xx
NAND (bitwise NAND)
- 04rs -> register r = !(register r && register s)
- 84xx -> register 0 = !(register 0 && xx)
OR (bitwise OR)
- 05rs -> register r = register r || register s
- 85xx -> register 0 = register 0 || xx
NOR (bitwise NOR)
- 06rs -> register r = !(register r || register s)
- 86xx -> register 0 = !(register 0 || xx)
XOR (bitwise XOR)
- 07rs -> register r = register r ^ register s
- 87xx -> register 0 = register 0 ^ xx
XNOR (bitwise XNOR)
- 08rs -> register r = !(register r ^ register s)
- 88xx -> register 0 = !(register 0 ^ xx)
NOT (bitwise NOT)
- 09r_ -> register r = !register r (second operand is not used)
- 89__ -> Does nothing
CMP (compare registers)
- 0Ars -> register r = CMP(register r, register s)
- 8Axx -> register 0 = CMP(register 0, xx)
SFT
- 0Brs -> register r = register r >> register s (if register s contains negative, left shift, else right shift)
- 8Bxx -> register 0 = register 0 >> xx (if xx is negative left shift, else right shift)
NOP (reserved for future upgrades)
- 0C-0F__ -> Does nothing
- 8C-8F__ -> Does nothing

### Jumps, subroutines and halting

NOP
- 10__ -> Does nothing
- 90__ -> Does nothing
JMP
- 11r_ -> register PC = register r (second operand is not used)
- 91xx -> register PC = register PC + xx (xx is interpreted as int8)
CJMP
- 12rs -> register PC = register r IF register s != 0
- 92xx -> register PC = register PC + xx IF register 0 != 0 (xx is interpreted as int8)
NJMP
- 13rs -> register PC = register r IF register s == 0
- 93xx -> register PC = register PC + xx IF register 0 == 0 (xx is interpreted as int8)
JSR
- 14r_ -> RAM[register SP] = register PC, register SP--, register PC = regiser r (second operand not used)
- 94xx -> RAM[register SP] = register PC, register SP--, register PC = register PC + xx (xx is interpreted as int8)
RTN
- 15__ -> register SP++, register PC = RAM[register SP] (both operands are not used)
- 95__ -> Does nothing
NOP (reserved for future upgrades)
- 16-1E__ -> Does nothing
- 96-9E__ -> Does nothing
HLT
- 1F__ -> signal to simulation that execution is done and to exit with code 0. (both operands not used)
- 9Fxx -> Signal to simulation that execution is done and to exit with code xx (i.e. error exit)

### Data moving

NOP
- 20__ -> Does nothing
- A0__ -> Does nothing
MOV
- 21rs -> register r = register s
- A1xx -> register r = xx (xx is interpreted as uint8)
LDR
- 22rs -> register r = RAM[register s]
- A2xx -> register 0 = RAM[xx] (xx is interpreted as uint8)
STR
- 23rs -> RAM[register s] = register r
- A2xx -> RAM[xx] = register 0 (xx is interpreted as uint8)
PSH
- 24r_ -> RAM[register SP] = register r, register SP-- (second operand is not used)
- A4xx -> RAM[register SP] = xx, register SP-- (xx is interpreted as uint8)
POP
- 25r_ -> register SP++, register r = RAM[register SP] (second operand is not used)
- A5__ -> Does nothing
SPR
- 26r_ -> register r = register SP (second operand is not used)
- A6__ -> Does nothing
SPW
- 27r_ -> register SP = register r (second operand is not used)
- A7xx -> register SP = xx (xx is interpreted as uint8)
PCR
- 28r_ -> register r = register PC (second operand is not used)
- A8__ -> Does nothing
PCW
- 29r_ -> register PC = register r (second operand is not used)
- A9xx -> register PC = xx (xx is interpreted as uint8)
NOP (reserved for future use cases)
- 2A-2F__ -> Does nothing
- AA-AF__ -> Does nothing

### Input and Output:

NOP
- 30__ -> Does nothing
- B0__ -> Does nothing
INP
- 31r_ -> register r = first character from stdin (second operand is unused, character is consumed, character value in ascii)
- B1__ -> Does nothing
OUP
- 32r_ -> output the character in register r to stdout (second operand is unused, character is ascii value)
- B2xx -> output the character from xx to stdout (charachter is ascii value)

### All other opcodes

- 40-7F__ -> Does nothing
- C0-FF__ -> Does nothing

### Additional functions

CMP(x, y): 
- bit 0: set if x == y
- bit 1: set if x < y
- bit 2: set if x > y
- bit 3: set if x + y would overflow
- bit 4: set if x - y would underflow
- bit 5-15 set to 0

## Running:

The CPU will run by calling the executable, followed by a program file. Additional flags may be provided in CLI, or in the program file e.g. to set stepping mode(helpfull for debugging of programs).

The program file is a json file that should look like the following:

```json
{
	"CPU_VERSION": 1,
	"FLAGS": ["List of flags", "To include", "When running the program", "On simple_cpu"],
	"REGISTER_FILE": {
		"PC": "0x0000",
		"SP": "0xFFFF",
		"REGISTERS": ["0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000", "0x0000"]
	},
	"RAM": ["0x1F00", "0x0000*65535"]
}
```

In this example, all possible sections of the program file have been included. Explanations are below:

### CPU_VERSION

This field is a required section, signalling to the processor for which version the code was created. As of now only a single version exists, but when a new version is created, a version 2 CPU should still be able to run version 1 code, but a version 1 CPU might not give the desired result when running version 2 code, thus it should give an error and exit.

### FLAGS

This is an optional field, as the CPU can execute the program file without any flags, and the flags can also be added to the CLI. It is planned for the following flags to work:

--debug | -D | "DEBUG" -> Executes the program file in debug mode. For debug mode see below:
--ramdump | -R | "RAMDUMP" -> After executing generate a new program file with the state of the registers and ram after halting, or sending sig-kill, as well as the flags used in execution, and the version of the code.

### REGISTER_FILE

This is an optional field. If included "PC", "SP", and "REGISTERS"  are required subfields, and the registers will be set according to the given values. Each register is 16 bits, and thus each register value must be 4 hex characters. furthermore, "REGISTERS" must be a 16 long list. 

### RAM

This is a required field. Program execution begins at 0x0000, and the stackpointer begins at 0xFFFF (Unless otherwise specified by the "REGISTER_FILE" field). "RAM" must be a list of strings, where each string is either a 16-bit value in hex format (i.e. "0x0000"), or a 16-bit value in hex format followed by "*" and a number N. This will repeat said 16-bit value N times in RAM.

When the RAM field does not give enough values to fill up RAM, it will be assumed that all other values are meant to be "0x0000". When the RAM field gives more than 65536 values after expanding repeated values, an error will occur as the program file will be too large.

### Normal execution:

In normal execution, the CPU simulator will run as fast as possible, completing the program, and printing the returning code specified by the HLT opcode. When any of the ramdump flags have been set, the complete ram and register file will be written to a new program file named the same as the input.

Execution of code is done in steps. Each step executes a single instruction. This means that, during a step, the codeline is retrieved from RAM, at the address specified by *register PC*, *register PC* is incremented, and finally, the retrieved codeline is executed. 

### Debug execution:

In debug execution, the CPU simulator will run only when specified. This mode of execution is selected when any of the debug flags is set. Once in debug execution mode, the console will accept commands, and output information based on these commands. Each command is case insensitive. The commands accepted are the following:

**step**: When a number is specified after step, it will step this many times. If no number is specified, it will step once.

**read**: Read expects extra arguments. When none are supplied, output will be shown stating as such. The possible arguments are:
 - Register: adding a register will print its contents _r0_, _r1_, ..., _rF_
 - Register range: adding a register, then a colon (:) then another register will print all registers in this range, including the specified registers.
 - Ram: adding a 16-bit hex value will output the value at that address in ram _0x0000_, _0x0001_, ..., _0xFFFF_
 - Ram range: adding a 16-bit hex value, then a colon (:) then another 16-bit hex value will print all ram contents in this range, including the specified addresses.
For each range: it is important to note that the first item in the range must be smaller than the second.
While the command itself is case insensitive, the arguments are case sensitive.

**write**: Write outputs the current ram and register contents, as well as the flags in use and program version number to a new program file. When a second argument is passed, it will be used as the file name for the new program file. If no name is specified, it will be using the *debug_* prefix on the name of the currently executing program file.

**reset**: When called, the CPU will reset to the state specified by the program file currently running. If any updates were made to it, these will be reflected immediately. If a file name is passed, it will be checked for a valid program file. If this is found, the CPU will start execution from the point specified by this new program file.

**quit**: Used to quit the CPU simulation.
