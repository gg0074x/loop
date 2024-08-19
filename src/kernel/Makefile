BUILD_DIR?=build
ASM?=nasm
ASMFLAGS?=-f elf32
ELF_LD?=i386-elf-ld
RUST?=rustc
RUST_FLAGS?=-O --target i686-unknown-linux-gnu --crate-type lib --emit=obj -C link-args=-no-pie -C relocation-model=static
RUST_FLAGS_ASM?=-O --target i686-unknown-linux-gnu --crate-type lib --emit=asm -C link-args=-no-pie -C relocation-model=static -Zasm_comments


SOURCES_RUST=$(wildcard *.rs)
SOURCES_ASM=$(wildcard *.asm)
OBJECTS_RUST=$(patsubst %.rs, $(BUILD_DIR)/kernel/rust/%.o, $(SOURCES_RUST))
OBJECTS_RUST_ASM=$(patsubst %.rs, $(BUILD_DIR)/kernel/rust/%.s, $(SOURCES_RUST))
OBJECTS_ASM=$(patsubst %.asm, $(BUILD_DIR)/kernel/asm/%.o, $(SOURCES_ASM))

all: kernel

kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin: $(OBJECTS_RUST_ASM) $(OBJECTS_RUST) $(OBJECTS_ASM)
	$(ELF_LD) -o $(BUILD_DIR)/kernel.bin -Ttext 0x1000 --entry=entry $(OBJECTS_ASM) $(OBJECTS_RUST) --oformat binary

$(BUILD_DIR)/kernel/rust/%.o: %.rs always
	$(RUST) $(RUST_FLAGS) $< -o $@

$(BUILD_DIR)/kernel/rust/%.s: %.rs always
	$(RUST) $(RUST_FLAGS_ASM) $< -o $@

$(BUILD_DIR)/kernel/asm/%.o: %.asm always
	$(ASM) $(ASMFLAGS) $< -o $@

.PHONY: all kernel clean



always:
	mkdir -p $(BUILD_DIR)/kernel/asm
	mkdir -p $(BUILD_DIR)/kernel/rust

clean:
	rm -f $(BUILD_DIR)/kernel.bin
