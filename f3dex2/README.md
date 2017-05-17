# Bass Macros for F3DEX2 / F3DZEX

This is a collection of macros for ARM9's fork of byuu's bass assembler that allow for
"assembling" of C-style F3DEX2/F3DZEX macros into data.

## How to Use

### Note on Vertices
Due to Nintendo's fixed point format for vertex data and bass' inability to perform floating point math,
some values need to be pre-computed into the proper fixed pointer representation and used in bass as an 8/16/32 bit int.

* list of data input

## Testing
