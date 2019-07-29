# Vm
A stack-based virtual machine

## Opcodes

* **Jump** - Jumps to the nth instruction
* **JumpIfFalse** - Pops the value from the stack, if it's false, jumps to the nth instruction
* **Store** - Pops the value from the stack, stores it inside the environment with the given name
* Load** - Loads the variable from the environment
* **BuildList** - Pops x values from the stack, creates a list, pushes the result onto the stack
* **BinaryAdd** - Pops two values from the stack, compute addition, pushes the result onto the stack
* **BinarySub** - Pops two values from the stack, compute subtraction, pushes the result onto the stack
* **BinaryMult** - Pops two values from the stack, compute multiplication, pushes the result onto the stack
* **BinaryDiv** - Pops two values from the stack, compute division, pushes the result onto the stack
