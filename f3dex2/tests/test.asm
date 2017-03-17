//bass-n64

arch n64.cpu
endian msb

macro align(bound) {
  while (pc() % {bound}) {
    db 0x00
  }
}

include "../flags.bass"
include "../vertex.bass"
//include "../opcodes.bass"

print "F3DEX2 Macros included OKAY\n\n"

origin 0x00
base   0x00
//-----Vertex Tests
//---Vertex_Color
scope vtx_color {
  align(16)
  db "Begin vtx_color tests: "
  
  align(16)
  Vertex_Color(0x10, 0x20, 0x08, 0, 100, 255, 255, 0, 255)

  // manual vertex
  align(16)
  dh  0x10, 0x20, 0x08
  dh  0
  dh  0
  dh  3200
  db  255, 255, 0, 255
  align(16)
  db "Should be equal"

  align(16)
  fill 16, 0

  Vertex_Color(0xFFFF, -1, -0x50, 101.5, -1024, 180, 100, 0xFF, 0xD0)
  // manual vertex
  align(16)
  dh  0xFFFF, -1, -0x50
  dh  0
  dh  3248
  dh  32768
  db  180, 100, 255, 0xD0
  align(16)
  db "Should be equal"

  align(16)
  fill 16, 0

  Vertex_Color(-10, -20, -50, -201.25, 1000.3, 180, 100, 0xFF, 0xD0)
  // manual vertex
  align(16)
  dh  -10, -20, -50
  dh  0
  dh  39208
  dh  32009
  db  180, 100, 255, 0xD0
  align(16)
  db "Should be equal"
}
