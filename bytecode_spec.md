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

each instruction in the bytecode is either : 
- exiting from the block/program
- returning a value
the only exception is call of a void function

an instruction is composed of an opcode then the operands<br>
there is multiple types of operands : 
- imm (immediate) 8-bytes raw integer value
- varid (variable id) 8-bytes id of a variable stored on the stack
- blkid (block id) 8-bytes id of a block
- result 8-bytes id of a instruction that output a value
- fnid (function id) 8-bytes id of a function

there are the following opcodes : 

| id | name       | operands                | description                                                                |
| -- | --         | --                      | --                                                                         |
| 00 | ret        |                         | return nothing from the function                                           |
| 10 | reti       | result                  | return a value from the function                                           |
| 11 | varget     | varid                   | get the value of a variable                                                | 
| 12 | iconst     | imm                     | an integer constant                                                        |
| 13 | br         | blkid                   | unconditionally branch to a block                                          |
| 14 | pusharg    | result                  | add a argument to the argument list                                        |
| 15 | call       | fnid                    | call a function pass the argument list and clear it after the call         |
| 16 | neg        | result                  | negate the value                                                           |
| 20 | varset     | varid, result           | set the value of a variable                                                |
| 21 | add        | result,result           | add the two values                                                         |
| 22 | sub        | result,result           | do val1-val2                                                               |
| 23 | mul        | result,result           | do val1*val2                                                               |
| 24 | div        | result,result           | do val1/val2                                                               |
| 25 | eq         | result,result           | do val1==val2                                                              |
| 26 | ne         | result,result           | do val1!=val2                                                              |
| 27 | gt         | result,result           | do val1>val2                                                               |
| 28 | ge         | result,result           | do val1>=val2                                                              |
| 29 | le         | result,result           | do val1<=val2                                                              |
| 2A | lt         | result,result           | do val1<val2                                                               |
| 2B | lsh        | result,result           | do val1<<val2                                                              |
| 2C | rsh        | result,result           | do val1>>val2                                                              |
| 30 | briz       | blkid, blkid, result    | branch to the first block if the value is 0 otherwise branch to the second |

as you can see the first byte of an opcode is the number of operand (this might change later)<br>
for each instruction there is the opcode and then the operands
