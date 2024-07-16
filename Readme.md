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

## syntax

current source file are composed of : 
a number indicating the tape size 
a number indicating the default value for the numbers on the tape
then two sections that starts by .SESSION_NAME:
    - TEXT that store the code
    - DATA that store default value of the first tape entries
the TEXT section should only contain instruction symbols line break or spaces
any line begining by a # is ignored
the DATA section contain one number per line 

like so : 
```
# calculate the first 10 numbers in the fibonnatchi sequence
6
1
.DATA
# jump table
0
1
2
# quantity of numbers to calulate
10
# progress counter (count how many numbers we have already calculated)
0
.TEXT
# main block index : 0 
>
# print one two times as it's the first 
LP
# main loop index : 1
[
# calulate a number in the sequence
+SP
# increase number progress counter
<LIS
# check if the program should exit
<C<}
# load the number that was calculated before
>>>L
# calculate the next number
>+SP
# increase progress couter
<<LIS
# check if the program should exit
<C<}
# loop back to the start of the loop
<]
# exit block index 2
# set M to zero
L-
E
```

