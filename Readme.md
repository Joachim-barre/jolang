# Jolang
---

A jit inerpreted language created by joachim barre

it is based on a set of basic instruction an use a single register and a memory tape that as an initial state and size define in the object 
numbers are currently 64 bits
jumps can jump to label that aren't encountered yet
the first tape value should be the main label address
jump indexes start at zero but the index 0 is a jump to the first instruction of the program

## instruction set

currently the language is composed of ten instructions :

| id | symbol | decription                                                                                                   |
| -- | --     | --                                                                                                           |                         
| 0  | <      | move the memory tape backward                                                                                |
| 1  | >      | move the memory tape forward                                                                                 |
| 4  | L      | load a value from the tape into the register                                                                 |
| 3  | S      | store what in the rengister into the current memory tape entry                                               |
| 4  | +      | add the current value on the memory tape to the register                                                     |
| 5  | -      | subtract the current value on the memory tape from the register                                              |
| 6  | *      | multiply the current register value by the current memory tape value                                         |
| 7  | /      | divide the current register value by the current memory tape value                                           |
| 8  | P      | print the register value to stdout                                                                           |
| 9  | [      | label : set a jump label their index is simply their order in the source code                                | 
| 10 | ]      | jumps to the label who is at the index stored in the current memory tape value                               |
| 11 | }      | does the same as the ] instruction if the current register value is 0                                        |
| 12 | Q      | exit program with the exit code contained in the register                                                    |
| 13 | I      | increase the register value by 1                                                                             |
| 14 | D      | decrease register value by 1                                                                                 |
| 15 | C      | compare reg to the current tape value and if equal set the register to 0 if greater to 1 and if lesser to -1 |

## file formats

| extention | description            |
| --        | --                     |
| .joo      | compiled jolang object |
| .jol      | source code            |
