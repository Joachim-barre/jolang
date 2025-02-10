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
- 4-byte signature length
- signature

### signature format

the signature is a string, composed itself of smaller strings indicating the arguments and return type, the first substring is always the return value and is mandatory

the string is a list of typenames separated by slashes (/)

with the current type name being:
 - i8, i16, i32, i64 for integers (signed and unsigned are not distinguished by the signature)
 - void only for returns meaning the function do not return any value

for exemple:
- the function : void foo(int, long, short) has this signature : "void/i32/i64/i16"
- the function : int bar(long, byte) has this signature : i32/i64/i8
- the function void baz() has this signature void
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
- size : 4 byte value indicating the size of a 0 means reference size and the only other supported sizes and are byte counts  are 1, 2, 4, 8
- imm : 8 byte immediate value that might get down casted
- value: a value that has a size indicated by an instruction operand (a size operand in most of the cases)

reference size depends of the architecture however references might not be pointer sized 

| opcode | name     | operands       | effect on stack                  | description                                                                                                     |
| ------ | -------- | -------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| 00     | nop      |                |                                  | does nothing                                                                                                    |
| 01     | pop      | size           | value ->                         | pop a value from the stack                                                                                      |
| 02     | dup      | size           | value -> value, value            | duplicate the value on top of the stack                                                                         |
| 03     | swap     | size           | value1, value2 -> value2, value1 | swap the two values on top of the stackj                                                                        |
| 04     | br       | blkid          | \[no change\]                    | unconditionally branch to another block                                                                         |
| 05     | briz     | blkid1, blkid2 | byte ->                          | jump to the first block  if the byte on top of the stack is equal to zero otherwise jump to the second          |
| 06     | call     | fnid           | \[arg1, arg2, ...\] -> return    | call a function taking arguments from the top of the stack                                                      |
| 07     | varref   |                | varid -> ref                     | get the reference to a local variable                                                                           |
| 08     | iconst   | size, imm      | -> value                         | push an integer constant only takes the least significant bits in imm if size is less than 8                    |
| 09     | iload    | size           | ref -> value                     | load the referenced integer                                                                                     |
| 0A     | istore   | size           | ref, value ->                    | store the integer at reference                                                                                  |
| 0B     | iret     | size           | value -> \[returns\]             | return an integer from the function                                                                             |
| 0C     | inot     | size           | value -> value                   | perform a bitwise not on an integer                                                                             |
| 0D     | ior      | size           | value, value -> value            | perform a bitwise or on two integers                                                                            |
| 0E     | iand     | size           | value, value -> value            | perform a bitwise and on two integers                                                                           |
| 0F     | ixor     | size           | value, value -> value            | perform a bitwise xor on two integers                                                                           |
| 10     | ilshr    | size           | value1, value2 -> value          | perform a logical right shift on two integers where int1 is shifted by int2                                     |
| 11     | iashr    | size           | value, value ->  value           | perform an arithmetic right shift on two integers where int1 is shifted by int2                                 |
| 12     | ishl     | size           | value, value -> value            | perform a left shift on two integers where int1 is shifted by int2                                              |
| 13     | ineg     | size           | value -> value                   | negate an integer                                                                                               |
| 14     | iadd     | size           | value, value -> value            | add two integers and push the result on top of the stack                                                        |
| 15     | isub     | size           | value, value -> value            | subtract int2 from int1 and push the result on top of the stack                                                 |
| 16     | imul     | size           | value, value -> value            | multiply two integers and push the result                                                                       |
| 17     | idiv     | size           | value1, value2 -> value          | divide int1 by int2 assumes that the two are signed and push the quotient                                       |
| 18     | udiv     | size           | value1, value2 -> value          | divide int1 by int2 assumes that the two are unsigned and push the quotient                                     |
| 19     | irem     | size           | value1, value2 -> value          | divide int1 by int2 assumes that the two are signed and push the remainder                                      |
| 1A     | urem     | size           | value1, value2 -> value          | divide int1 by int2 assumes that the two are unsigned and push the remainder                                    |
| 1B     | ieq      | size           | value1, value2 -> byte           | if int1 is equal to int2 push 1 on the stack otherwise push 0                                                   |
| 1C     | ine      | size           | value1, value2 -> byte           | if int1 isn't equal to int2 push 1 on the stack otherwise push 0                                                |
| 1D     | ige      | size           | value1, value2 -> byte           | if int1 is greater or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are signed   |
| 1E     | igt      | size           | value1, value2 -> byte           | if int1 is greater than int2 push 1 on the stack otherwise push 0 assumes that the integers are signed          |
| 1F     | uge      | size           | value1, value2 -> byte           | if int1 is greater or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned |
| 20     | ugt      | size           | value1, value2 -> byte           | if int1 is greater than int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned        |
| 21     | ilt      | size           | value1, value2 -> byte           | if int1 is lesser than int2 push 1 on the stack otherwise push 0 assumes that the integers are signed           |
| 22     | ile      | size           | value1, value2 -> byte           | if int1 is lesser or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are signed    |
| 23     | ule      | size           | value1, value2 -> byte           | if int1 is lesser than int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned         |
| 24     | ult      | size           | value1, value2 -> byte           | if int1 is lesser or equal to int2 push 1 on the stack otherwise push 0 assumes that the integers are unsigned  |
| 25     | iconv    | size,size      | value -> value                   | convert an integer either by sign extension or truncation                                                       |
| 26     | uconv    | size, size     | value -> value                   | convert an integer either by zero extension or truncation                                                       |
| 27     | reserved |                |                                  | reserved for future use                                                                                         |
| 28     | reserved |                |                                  | reserved for future use                                                                                         |
