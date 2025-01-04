# ffff
An esolang inspired by absolutely no one important.

# Commands
|  Command   | Description                                                         |
|:----------:|---------------------------------------------------------------------|
| `fufufafa` | Push a 1 onto the stack                                             |
| `fafafufu` | Pop top value from stack                                            |
|   `fufu`   | Add top two values on stack, push result                            |
|   `fafa`   | Subtract second value from top value on stack, push result          |
|    `fu`    | Multiply top two values on stack, push result                       |
|    `fa`    | Divide second value by top value on stack, push result              |
|    `f`     | Print top value as number                                           |
|    `ff`    | Print top value as ASCII character                                  |
|   `fff`    | Duplicate top value on stack                                        |
|   `ffff`   | Swap top two values on stack                                        |
|  `fffff`   | Input a number and push to stack                                    |
|  `ffffff`  | Start a loop (if top of stack is 0, jump to matching `fffffff`)     |
| `fffffff`  | End a loop (jump back to matching `ffffff` if top of stack isn't 0) |

# Usage
```
cargo run filename.ffff
```