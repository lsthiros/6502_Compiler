# Kaleidoscope to 6502 asm compiler

## Project Overview

### Motivation
This project is intended to be a dual exploration into both Rust and a compiler
tech. The idea to start this project came from the 6502 assembler created
earlier. This project should also help further understand the 6502 architecture
for creating an NES emulator. The Kaleidoscope language was chosen for its use
as the stereotypical LLVM compiler tutorial, and its simplicity. The 6502 ASI
was chosen as part of an ongoing attempt to create a NES emulator.

This project will be completed in Rust. This language was chosen after the use
of C in the 6502 Assembler project. While C's lack of standard container
structures was a non-issue, the lack of template safety was. Rust will also
enforce memory safety, which can otherwise be time consuming in C. The use of
Rust will make the project more complicated, it will provide a learning
opportuity.

The learning goals of this project are as follows:

* DAG creation
* SSA creation
* Machine Code selection and emitting:
  * Register allocation
  * Code selection
* The Rust programming language

### Scope
This project will create a compiler that is capable of transforming a *single*
Kaleidoscope code file into a hex file 

