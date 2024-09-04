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

each instruction is 32-bytes (7 padding bytes then 1 for the opcode and then 24 for the operands)

stack is not kept between blocks but blocks arguments are pushed on top of the stack.

st[x] indicate a value on the stack with a offset from the top of x.
for exemple st[0] is the top of the stack 

unless stated otherwise integer sizes are in bits<br>
if an instruction access the stack the values must have the same size and their size is deducted at runtime. the run may crash or throw an error in case of bad sizing.

an instruction is composed of an opcode then the operands<br>
there is multiple types of operands : 
- imm64 (immediate) 8-bytes raw integer value
- imm (immediate) 16-bytes raw value containing an integer a size that depens on another opcode (must have padding after if the integer value is smaller than 16 bytes)
- uimm64 8-bytes raw unsigned integer value
- blkid (block id) 8-bytes id of a block
- fnid (function id) 8-bytes id of a function

there are the following opcodes : 

| id | name       | operands                | description                                                                    |
| -- | --         | --                      | --                                                                             |
| 00 | ret        |                         | return nothing from the function                                               |
| 01 | reti       |                         | return the top of the stack                                                    |
| 02 | iconst     | uimm64, imm             | push a integer constant that have a size indicated by the first operand        |
| 03 | icast      | uimm                    | cast st[0] to another integer type by trimming or sign extanding               |
| 04 | br         | blkid                   | unconditionally branch to a block passing the top of the stack as argument     |
| 05 | dup        |                         | duplicate the top of the stack                                                 |
| 06 | dupx       | uimm64                  | duplicate st[stack_size-offset-1] where offset is the immediate value          |
| 07 | swap       |                         | swap the two values on top of the stack                                        |
| 08 | call       | fnid                    | call a function pass the top of the stack as argument and pop the value passed |
| 09 | neg        |                         | pop the top of the stack and push the negated value                            |
| 0A | add        |                         | do st[1] + st[0] pop them and push the result                                  |
| 0B | sub        |                         | do st[1] - st[0] pop them and push the result                                  |
| 0C | mul        |                         | do st[1] * st[0] pop them and push the result                                  |
| 0D | div        |                         | do st[1] / st[0] pop them and push the result                                  |
| 0E | eq         |                         | do st[1] == st[0] pop them and push the result                                 |
| 0F | ne         |                         | do st[1] != st[0] pop them and push the result                                 |
| 10 | gt         |                         | do st[1] > st[0] pop them and push the result                                  |
| 11 | ge         |                         | do st[1] >= st[0] pop them and push the result                                 |
| 12 | le         |                         | do st[1] <= st[0] pop them and push the result                                 |
| 13 | lt         |                         | do st[1] < st[0] pop them and push the result                                  |
| 14 | lsh        |                         | do st[1] >> st[0] pop them and push the result                                 |
| 15 | rsh        |                         | do st[1] << st[0] pop them and push the result                                 |
| 16 | briz       | blkid, blkid            | branch to the first block if st[0] is 0 otherwise branch to the second         |

for each instruction there is the opcode and then the operands
