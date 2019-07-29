# Vm
A stack-based virtual machine

## Opcodes

* Jump - Jumps to the nth instruction
* JumpIfFalse - Pops the value from the stack, if it's false, jumps to the nth instruction
* Store - Pops the value from the stack, stores it inside the environment with the given name
* Load - Loads the variable from the environment
* BuildList - Pops x value from the stack, creates a list, pushes the result onto the stack
