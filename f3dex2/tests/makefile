AS = bass
ASFLAGS=-create

dltest:
	$(AS) $(ASFLAGS) -o dl-test.bin dl-test.asm

triangle:
	$(AS) $(ASFLAGS) -o triangle.bin triangle.asm

vertex:
	$(AS) $(ASFLAGS) -o vertex-test.bin vertex-test.asm

.PHONY : clean
clean:
	-rm *.bin *.obj *.mtl
