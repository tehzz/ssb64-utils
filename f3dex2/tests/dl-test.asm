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

origin 0x00
base   0x00
db "Space out..."

align(16)
Bank0:
Vertex_Normal(-51, 37, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-34, 57, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-43, 49, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-55, 27, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-57, 19, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-34, -4, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-58, 11, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-58, -4, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-51, -24, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-34, -17, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-54, -17, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-34, -46, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-47, -32, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-40, -41, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-2, 72, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(24, 72, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(8, 73, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(36, 68, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(44, 64, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(52, 58, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(61, 50, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(69, 38, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-2, -4, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(75, 25, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(77, 14, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(77, 3, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(77, -4, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-2, -17, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(69, -27, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(74, -17, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(59, -41, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(-2, -61, 0, 0, 0, 0, 0, 127,0)
Bank1:
Vertex_Normal(-2, -61, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(49, -50, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(59, -41, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(40, -55, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(31, -59, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(21, -61, 0, 0, 0, 0, 0, 127,0)
Vertex_Normal(7, -62, 0, 0, 0, 0, 0, 127,0)

align(8)
displaylist:
gsDPPipeSync()
dw 0xFCFFFFFF, 0xFFFE7D3E //gsDPSetCombineLERP
gsSPDisplayList(0x0E000000)
gsSPTexture(0, 0, 0, 0, 0)
gsSPVertex(Bank0, 32, 0)
gsSP2Triangles(0, 1, 2, 0, 3, 1, 0, 0)
gsSP2Triangles(4, 1, 3, 0, 4, 5, 1, 0)
gsSP2Triangles(6, 5, 4, 0, 7, 5, 6, 0)
gsSP2Triangles(8, 9, 10, 0, 8, 11, 9, 0)
gsSP2Triangles(12, 11, 8, 0, 13, 11, 12, 0)
gsSP2Triangles(14, 15, 16, 0, 14, 17, 15, 0)
gsSP2Triangles(14, 18, 17, 0, 14, 19, 18, 0)
gsSP2Triangles(14, 20, 19, 0, 14, 21, 20, 0)
gsSP2Triangles(22, 21, 14, 0, 22, 23, 21, 0)
gsSP2Triangles(22, 24, 23, 0, 22, 25, 24, 0)
gsSP2Triangles(22, 26, 25, 0, 27, 28, 29, 0)
gsSP2Triangles(27, 30, 28, 0, 31, 30, 27, 0)
gsDPPipeSync()
gsSPDisplayList(0x0E000000)
gsSPVertex(Bank1, 7, 0)
gsSP2Triangles(0, 1, 2, 0, 0, 3, 1, 0)
gsSP2Triangles(0, 4, 3, 0, 0, 5, 4, 0)
gsSP1Triangle(6, 5, 0, 0)
gsSPEndDisplayList()
