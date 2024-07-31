# Jolang

A jit interpreted language created by joachim barre
 
all variables are 64-bit integer<br>

## file formats

| extention | description            |
| --        | --                     |
| .joo      | compiled jolang object |
| .jol      | source code            |

## syntax

### TODO better documentation

a source file currently looks like that: <br>
```
// this is a line comment
/* block comment*/
builtin_function(arg1, arg2); // call to a builtin function
var varible; // variable declaration default value is zero
varible = input(); // variable assigment
if (varible == sqrt(4)) { // if statement
    print(variable); 
} else {
    return 1;
}
var i = 0;
while(i <= 4) {
    i = i + 1;
    print(i*i);
}
loop {
    var v = input();
    print(v-1);
    if ( v < 0 ) {
        break; // exit the loop
    }else 
        continue;
}
return 0; // exit the program with exit code 0
```

these builtin functions are currently available: 
| name    | arg count | description                                      |
| --      | --        | --                                               |
| print   | 1         | print a variable to stdout                       |
| input   | 0         | read a variable from stdin                       |
| sqrt    | 1         | square root                                      |
| sin     | 1         | sine of the argument                             |
| cos     | 1         | cosine of the argument                           |
| tan     | 1         | tangent of the argument                          |
| pow     | 2         | first argument to the power of the second        |
| randint | 2         | generate a random interger between arg1 and arg2 |

## binary object format

the generated binary object follow this format

- an header composed of four bytes to indicate that it is the binary format that can be executed that is "\0JOO"
### TODO binary format

