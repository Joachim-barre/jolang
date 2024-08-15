# Jolang object format

## header

- 4-byte string ```\0JOO```
- 1-byte major version of the targeted runtime
- 1-byte minor version of the targeted runtime
- 1-byte patch version of the targeted runtime
- header table :
    - 8-bytes entry count for the table
    - 8-bytes offset for start to the table

there are 3 tables in the header and they should be in this order:

1. external functions
2. variables
3. blocks

## external functions table

each entry has the following fields:

- 4-bytes name size
- name
- 1-byte arg count
- bool (1-byte) does the object expect a return

## variables table

for each variable : 8-bytes for the default value

## block table

for each block:

- 8-bytes quantity of instructions in the block
- 8-bytes offset for the start of file to the block's instructions

## instructions table

the vm uses a stack for temporary values, it is separated by frames, most of the time each statemnt uses it's own frame<br>

an instruction is composed of an opcode then the operands<br>
there is multiple types of operands : 
- imm (immediate) 8-bytes raw integer value
- varid (variable id) 8-bytes id of a variable stored on the stack
- blkid (block id) 8-bytes id of a block
- offset 8-bytes offset from the bottom of the stack frame
- fnid (function id) 8-bytes id of a function

there are the following opcodes : 

| id | name       | operands         | description                                                 |
| -- | --         | --               | --                                                          |
| 00 | exit       |                  | exit the programe with the following exit code              |
| 01 | mkfr       |                  | create a new stack frame                                    |
| 02 | delfr      |                  | delete the current stack frame                              | 
| 10 | pushi      | imm              | push the immediate value on top of the stack                |
| 11 | pushv      | varid            | push a variable on top of the stack                         |
| 12 | pusht      | offset           | push a stack value on top of the stack                      |
| 13 | br         | blkid            | unconditionally branch to a block                           |
| 14 | call       | fnid             | call a function passing the top of the stack as argument    |
| 15 | neg        | offset           | negate the value at the offset and push it on the stack     |
| 20 | briz       | blkid, offset    | branch to a block if the value at the offset is zero        |
| 21 | store      | varid, offset    | store the value at the offset in the variable               |
| 22 | add        | offset,offset    | add the two value and push the result on top of the stack   |
| 23 | sub        | offset,offset    | do val1-val2 and push the result on top of the stack        |
| 24 | mul        | offset,offset    | do val1*val2 and push the result on top of the stack        |
| 25 | div        | offset,offset    | do val1/val2 and push the result on top of the stack        |
| 26 | eq         | offset,offset    | do val1==val2 and push the result on top of the stack       |
| 27 | ne         | offset,offset    | do val1!=val2 and push the result on top of the stack       |
| 28 | gt         | offset,offset    | do val1>val2 and push the result on top of the stack        |
| 29 | ge         | offset,offset    | do val1>=val2 and push the result on top of the stack       |
| 2A | le         | offset,offset    | do val1<=val2 and push the result on top of the stack       |
| 2B | lt         | offset,offset    | do val1<val2 and push the result on top of the stack        |
| 2C | lsh        | offset,offset    | do val1<<val2 and push the result on top of the stack       |
| 2D | rsh        | offset,offset    | do val1>>val2 and push the result on top of the stack       |

as you can see the first byte of an opcode is the number of operand (this might change later)<br>
for each instruction there is the opcode and then the operands
