
https://github.com/user-attachments/assets/77789547-2074-4bd0-94d3-5e3ac3f61126




| Symbol | Use Case                              |
|--------|---------------------------------------|
| `S`    |   marks the start of execution for your program and is chosen by the `S` closet to the top of the program                      |
| `"`    | `"` are how we denote strings and when you use a string it prints it out to the terminal         |
| `@`    |   haults execution of the program and exits                       |
| `#`    |   allows you to go up one line and travel to the token that is directly above this token if there is no token above it then it may have unseen effects                     |
| `~`    |   allows you to go down one line and travel to the token that is directly below this token if there is no token below it then it may have unseen effects                       |
| `<`    |   shifts the current stack pointer index to the left                  |
| `>`    |   shifts the current stack pointer index to the right                  |
| `0`    |   pushes zero to the stack                      |
| `+`    |   adds the pointed value on the stack with the value to the right of it                        |
| `-`    | symbol subtracts the pointed value on the stack with the value to the right of i                        |
| `i`    | adds one to the pointed value on the stack                        |
| `d`    | subtracts one to the pointed value on the stack                        |
| `[`    | moves the stack pointer to the front of the stack                      |
| `]`    | moves the stack pointer to the back of the stack                       |
| `.`    | used after a ladder or stair and is called the stepping stone for if you do not use this after coming off of a snake or a ladder it may have unforseen effects                       |
| `$`    | prints the pointed value on the stack to the termina              |
| `,`    | takes in a single character input  from the user and pushes it onto the stack as a number                      |
| `?`    | sets all of the flags according to the value pointed at on the stack and the value next to it                   |
| `l`    | checks if the less than flag is true and if it is will act as a ladder else it will function as a snake                   |
| `L`    | checks if the less than or equal to  flag is true and if it is will act as a ladder else it will function as a snake            |
| `g`    | checks if the greater than flag is true and if it is will act as a ladder else it will function as a snake                |
| `G`    | checks if the greater than  or equal to flag is true and if it is will act as a ladder else it will function as a snake        |
| `=`    | checks if the equal to flag is true and if it is will act as a ladder else it will function as a snake                  |
| `!`    | checks if the not equal to flag is true and if it is will act as a ladder else it will function as a snake                 |
| `_`    | takes in a single numerical input  from the user and pushes it onto the stack                   |
| `C`    | Copies the pointed value on the stack and pushes it to the front of the stack                    |
| `P`    | Pops the pointed value out of the stack                     |
| `/`    |   divides the pointed value on the stack with the value to the right of i                        |
| `*`    |   multiplys the pointed value on the stack with the value to the right of i                       |
| `%`    |   mods the pointed value with on the stack the value to the right of i                        |
| `\`   | goes to the closest availble token to the left (can be used in place of a stepping stone without problems)                   |
