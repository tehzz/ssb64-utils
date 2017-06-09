//bass-n64

arch n64.cpu
endian msb

macro align(bound) {
  while (pc() % {bound}) {
    db 0x00
  }
}

macro printHex(variable value) {
  if value > 15 {
    printHex(value >> 4)
  }
  value = value & 15
  putchar(value < 10 ? '0' + value : 'A' + value -10)
}

include "../flags.bass"
include "../vertex.bass"
include "../opcodes.bass"

print "F3DEX2 Macros included OKAY\n\n"
print "0x"; printHex(displaylist); print "\n\n"
print "0x"; printHex(dlMan); print "\n\n"
print "0x"; printHex(dl2); print "\n\n"

origin 0x00
base   0x00
db "0x5 Single Triangle"

align(16)
Bank0:
  Vertex_Normal(0, 40, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(-20, -10, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(20, -10, 0, 0, 0, 0, 0, 127, 0)

align(8)
displaylist:
  gsDPPipeSync()
  dw 0xFCFFFFFF, 0xFFFE7D3E //gsDPSetCombineLERP
  gsSPDisplayList(0x0E000010)
  gsSPTexture(0, 0, 0, 0, 0)
  gsSPVertex(Bank0, 3, 0)
  gsSP1Triangle(0, 1, 2, 0)
  gsSPEndDisplayList()

align(16)
db "Manually Converted Vertices and Display List"

align(16)
BankMan:
  dh  0, 40, 0; dh 0; dh 0, 0; db 0, 0, 127, 0
  dh  -20, -10, 0; dh 0; dh 0, 0; db 0, 0, 127, 0
  dh  20, -10, 0; dh 0; dh 0, 0; db 0, 0, 127, 0

align(8)
dlMan:
  dw  0xE7000000, 0x00000000
  dw  0xFCFFFFFF, 0xFFFE7D3E //gsDPSetCombineLERP
  dw  0xDE000000, 0x0E000010
  dw  0xD7000000, 0x00000000
  dw  0x01003006, BankMan
  dw  0x05000204, 0x00000000
  dw  0xDF000000, 0x00000000

align(16)
db "0x6 Two Triangles"

align(16)
Bank1:
  Vertex_Normal(0, 40, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(-20, -10, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(20, -10, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(0, -80, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(-20, -30, 0, 0, 0, 0, 0, 127, 0)
  Vertex_Normal(20, -30, 0, 0, 0, 0, 0, 127, 0)

align(8)
dl2:
  gsDPPipeSync()
  dw 0xFCFFFFFF, 0xFFFE7D3E //gsDPSetCombineLERP
  gsSPDisplayList(0x0E000010)
  gsSPTexture(0, 0, 0, 0, 0)
  gsSPVertex(Bank1, 6, 0)
  gsSP2Triangles(0, 1, 2, 0, 3, 4, 5, 0)
  gsSPEndDisplayList()
