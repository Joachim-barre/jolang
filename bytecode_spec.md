# Jolang bytecode object format

the whole file uses little endian for integer values

## header

- 4-byte string ```\0JOO```
- 1-byte major version of the targeted runtime
- 1-byte minor version of the targeted runtime
- 1-byte patch version of the targeted runtime
- header table :
    - 4-bytes entry count for the table
    - 4-bytes offset for start to the table

there are 3 tables in the header and they should be in this order:

1. external functions
2. blocks
3. local variable slots

## external functions table

each entry has the following fields:

- 4-bytes name size
- name
- 1-byte arg count

## block table

for each block:

- 4-bytes quantity of instructions in the block
- 4-bytes offset for the start of file to the block's instructions

## Local variable slots

for each local variable slot:

- 1-bytes size of the variable (0 is reference size) 
## instructions table

the instruction table is a list of instruction. each instruction is 16-byte long ( 3-padding bytes, the opcode 1 byte, 12 for the arguments) the padding bytes can have any value and should be ignored by the runtime, arguments should be put in the order they are laid out in the table below and if an instruction do not use all argument bytes the remaining space should be ignored

effect on the stack only impact the top of the stack

ids are always 4 bytes :
	- blkid are for blocks and start at zero
	- fnid are for external functions and start at zero
	- varid are for local variables they start at zero and 
other types are :
- isize : 4 byte value indicating the size of an int 0 means word size and the only other supported size are 8, 16, 32, 64
- imm : 8 byte immediate value that might get down casted
- int integer that has a size indicated by an instruction operand (an isize in most of the cases)

reference size depends of the architecture however references might not be pointer sized 

the following table uses word as a short way of saying 2 bytes please note that the Ir is supporting architectures that don't use 2 byte words, dword means 4 bytes and qword means 8 bytes

| opcode | name     | operands       | effect on stack                  | description                                                                                                     |
| ------ | -------- | -------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| 00     | nop      |                |                                  | does nothing                                                                                                    |
| 01     | pop      |                | byte ->                          | pop a byte from the stack                                                                                       |
| 02     | pop2     |                | word ->                          | pop 2 bytes from the stack                                                                                      |
| 03     | pop4     |                | dword ->                         | pop 4 bytes from the stack                                                                                      |
| 04     | pop8     |                | qword ->                         | pop 8 bytes from the stack                                                                                      |
| 05     | dup      |                | byte -> byte, byte               | duplicate a byte on top of the stack                                                                            |
| 06     | dup2     |                | word -> word, word               | duplicate 2 bytes on top of the stack                                                                           |
| 07     | dup4     |                | dword -> dword, dword            | duplicate 4 bytes on top of the stack                                                                           |
| 08     | dup8     |                | qword -> qword, qword            | duplicate 8 bytes on top of the stack                                                                           |
| 09     | swap     |                | byte1, byte2 -> byte2, byte1     | swap the two bytes on top of the stack                                                                          |
| 0A     | swap2    |                | word1, word2 -> word2, word1     | swap the two words on top of the stack                                                                          |
| 0B     | swap4    |                | dword1, dword2 -> dword2, dword1 | swap the two dword on top of the stack                                                                          |
| 0C     | swap8    |                | qword1, qword2 -> qword2, qword1 | swap the two qword on top of the stack                                                                          |
| 0D     | br       | blkid          | \[no change\]                    | unconditionally branch to another block                                                                         |
| 0E     | briz     | blkid1, blkid2 | byte ->                          | jump to the first block  if the byte on top of the stack is equal to zero otherwise jump to the second          |
| 0F     | call     | fnid           | \[arg1, arg2, ...\] -> return    | call a function taking arguments from the top of the stack                                                      |
| 10     | varref   |                | varid -> ref                     | get the reference to a local variable                                                                           |
| 11     | iconst   | isize, imm     | -> int                           | push an integer constant only takes the least significant bits in imm if isize is less than 64                  |
| 12     | iload    | isize          | ref -> int                       | load the referenced integer                                                                                     |
| 13     | istore   | isize          | ref, int ->                      | store the integer at reference                                                                                  |
| 14     | iret     | isize          | int -> \[returns\]               | return an integer from the function                                                                             |
| 15     | inot     | isize          | int -> int                       | perform a bitwise not on an integer                                                                             |
| 16     | ior      | isize          | int, int -> int                  | perform a bitwise or on two integers                                                                            |
| 17     | iand     | isize          | int, int -> int                  | perform a bitwise and on two integers                                                                           |
| 18     | ixor     | isize          | int, int -> int                  | perform a bitwise xor on two integers                                                                           |
| 19     | ilshr    | isize          | int1, int2 -> int                | perform a logical right shift on two integers where int1 is shifted by int2                                     |
| 1A     | iashr    | isize          | int, int ->  int                 | perform an arithmetic right shift on two integers where int1 is shifted by int2                                 |
| 1B     | ishl     | isize          | int, int -> int                  | perform a left shift on two integers where int1 is shifted by int2                                              |
| 1C     | ineg     | isize          | int -> int                       | negate an integer                                                                                               |
| 1D     | iadd     | isize          | int, int -> int                  | add two integers and push the result on top of the stack                                                        |
| 1E     | isub     | isize          | int, int -> int                  | subtract int2 from int1 and push the result on top of the stack                                                 |
| 1F     | imul     | isize          | int, int -> int                  | multiply two integers and push the result                                                                       |
| 20     | idiv     | isize          | int1, int2 -> int                | divide int1 by int2 assumes that the two are signed and push the quotient                                       |
| 21     | udiv     | isize          | int1, int2 -> int                | divide int1 by int2 assumes that the two are unsigned and push the quotient                                     |
| 22     | irem     | isize          | int1, int2 -> int                | divide int1 by int2 assumes that the two are signed and push the remainder                                      |
| 23     | urem     | isize          | int1, int2 -> int                | divide int1 by int2 assumes that the two are unsigned and push the remainder                                    |
| 24     | ieq      | isize          | int1, int2 -> byte               | if int1 is equal to int2 push 1 on the stack otherwise push 0                                                   |
| 25     | ine      | isize          | int1, int2 -> byte               | if int1 isn't equal to int2 push 1 on the stack otherwise push 0                                                |
| 26     | ige      | isize          | int1, int2 -> byte               | if int1 is greater or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are signed   |
| 27     | igt      | isize          | int1, int2 -> byte               | if int1 is greater than int2 push 1 on the stack otherwise push 0 assumes that the integers are signed          |
| 28     | uge      | isize          | int1, int2 -> byte               | if int1 is greater or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned |
| 29     | ugt      | isize          | int1, int2 -> byte               | if int1 is greater than int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned        |
| 2A     | ilt      | isize          | int1, int2 -> byte               | if int1 is lesser than int2 push 1 on the stack otherwise push 0 assumes that the integers are signed           |
| 2B     | ile      | isize          | int1, int2 -> byte               | if int1 is lesser or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are signed    |
| 2C     | ule      | isize          | int1, int2 -> byte               | if int1 is lesser than int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned         |
| 2D     | ult      | isize          | int1, int2 -> byte               | if int1 is lesser or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned  |
| 2E     | iconv    | isize,isize    | int -> int                       | convert an integer either by sign extension or truncation                                                       |
| 2F     | uconv    | isize, isize   | int -> int                       | convert an integer either by zero extension or truncation                                                       |
| 30     | reserved |                |                                  | reserved for future use                                                                                         |
| 31     | reserved |                |                                  | reserved for future use                                                                                         |

