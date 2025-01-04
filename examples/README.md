# Explanation
## hello
```
// In order to write "Hello"
// we need to push the ASCII values of the each character
// into the stack, one character at a time

// First, we will need to push 72 into the stack to get 'H'
// There definitely is a shorter way to do this but I will stick
// to this implementation

fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2
fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2
fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2
fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2
fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2
fufufafa fufufafa fufu // Push 1, Push 1, Add -> 2

// Our stack: 2 2 2 2 2 2

// Multiply each stack
// fu -> 2 * 2 = 4
// Stack: 4 2 2 2 2
// fu -> 4 * 2 = 8
// Stack: 8 2 2 2
// fu -> 8 * 2 = 16
// Stack: 16 2 2
// fu -> 16 * 2 = 32
// Stack: 32 2
// fu -> 32 * 2 = 64
// Stack: 64
fu fu fu fu fu

// Same thing here
fufufafa fufufafa fufu // 2
fufufafa fufufafa fufu // 2
fufufafa fufufafa fufu // 2

// Stack: 2 2 2 64

// fu -> 2 * 2 = 4
// Stack: 4 2 64
// fu -> 4 * 2 = 8
// Stack: 8 64
fu fu

// fufu -> 8 + 64
// Stack: 72
fufu

// Print top value as ASCII character
ff
```
The same goes for 'e', 'l', and 'o'. Except, for 'l', since we need two 'l's, we can just print it twice.
```
fufufafa fufufafa fufu
fufufafa fufufafa fufu fufu
fufufafa fufufafa fufu fufu
fufufafa fufu fufu

// Print as ASCII character twice
ff ff
```
