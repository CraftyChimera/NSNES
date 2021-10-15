# Registers:

## 16-bit:

### Accumulator (C or A and B):
There is one 16-bit register C.When the 5th bit(M) of register P is 1,C is divided into 2 8-bit registers A(the register that is used) and B(usually hidden but can be intechanged with A)

### Index Registers (X and Y):
These are 2 index registers X and Y. When the 4th bit(X) of register P is 1,These registers are seen as 8 bit registers.

### Direct Page Register (D):
The Direct Page Register D stores the start of the "zero page"(useful for direct page accessing mode)

### Stack Pointer (S):
The stack pointer S points to the next available(unused) location on the stack. 

## 8-bit:

### Program Bank Register (PBR):
The Program Bank Register (PBR) holds the bank address of all instruction fetches.

### Data Bank Register (DBR):
The Data bank register (DBR) holds the default bank for memory transfers. 

### Processor Status Register(P):
This register holds various flags,which alter behaviour of the CPU.





