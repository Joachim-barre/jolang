# Jolang object format

## header

- 4-byte string ```\0JOO```
- 1-byte major version of the targeted runtime
- 1-byte minor version of the targeted runtime
- 1-byte patch version of the targeted runtime
- header table :
    - 8-bytes entry count for the table
    - 8-bytes offset for start to the table

there are 2 tables in the header and they should be in this order:

1. external functions
3. blocks

## external functions table

each entry has the following fields:

- 4-bytes name size
- name
- 1-byte arg count
- bool (1-byte) does the object expect a return

## block table

for each block:

- 8-bytes quantity of instructions in the block
- 1-bytes block argument count
- 8-bytes offset for the start of file to the block's instructions

## instructions table

each instruction is 17-bytes (1 for the opcode and 16 for the operands)

stack is not kept between blocks but blocks arguments are pushed on top of the stack.

st[x] indicate a value on the stack with a offset from the top of x.
for exemple st[0] is the top of the stack 

an instruction is composed of an opcode then the operands<br>
there is multiple types of operands : 
- imm (immediate) 8-bytes raw integer value
- blkid (block id) 8-bytes id of a block
- fnid (function id) 8-bytes id of a function

there are the following opcodes : 

| id | name       | operands                | description                                                                    |
| -- | --         | --                      | --                                                                             |
| 00 | ret        |                         | return nothing from the function                                               |
| 10 | reti       |                         | return the top of the stack                                                    |
| 12 | iconst     | imm                     | push a integer constant                                                        |
| 13 | br         | blkid                   | unconditionally branch to a block passing the top of the stack as argument     |
| 14 | dup        |                         | duplicate the top of the stack                                                 |
| 15 | dupx       | imm                     | duplicate the value st[offset] where offset is the immediate value             |
| 15 | call       | fnid                    | call a function pass the top of the stack as argument and pop the value passed |
| 16 | neg        |                         | pop the top of the stack and push the negated value                            |
| 21 | add        |                         | do st[0] + st[1] pop them and push the result                                  |
| 22 | sub        |                         | do st[0] + st[1] pop them and push the result                                  |
| 23 | mul        |                         | do st[0] + st[1] pop them and push the result                                  |
| 24 | div        |                         | do st[0] + st[1] pop them and push the result                                  |
| 25 | eq         |                         | do st[0] + st[1] pop them and push the result                                  |
| 26 | ne         |                         | do st[0] + st[1] pop them and push the result                                  |
| 27 | gt         |                         | do st[0] + st[1] pop them and push the result                                  |
| 28 | ge         |                         | do st[0] + st[1] pop them and push the result                                  |
| 29 | le         |                         | do st[0] + st[1] pop them and push the result                                  |
| 2A | lt         |                         | do st[0] + st[1] pop them and push the result                                  |
| 2B | lsh        |                         | do st[0] + st[1] pop them and push the result                                  |
| 2C | rsh        |                         | do st[0] + st[1] pop them and push the result                                  |
| 30 | briz       | blkid, blkid            | branch to the first block if st[0] is 0 otherwise branch to the second         |

for each instruction there is the opcode and then the operands
