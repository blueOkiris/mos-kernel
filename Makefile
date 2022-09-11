# Author: Dylan Turner
# Description: Build the executable

# Options

## Assembly Build options
AS :=				nasm

## Cargo build options
RUSTC :=			cargo
ifeq ($(DEBUG),)
RUSTC_FLAGS :=		+nightly \
					rustc \
					--release \
					--target=x86_64-unknown-none \
					-- -C code-model=kernel -Z plt=y
else
RUSTC_FLAGS :=		+nightly \
					rustc \
					--target=x86_64-unknown-none \
					-- -C code-model=kernel -Z plt=y
endif

## Stage 1 bootloader options
STG1_SRC :=			$(wildcard boot/stage1/*.asm)
STG1_INC :=			-Iboot/stage1
STG1_AS_FLAGS :=	-f bin

## Stage 2 bootloader options
STG2_SRC :=			$(wildcard boot/stage2/*.asm)
STG2_INC :=			-Iboot/stage2
STG2_AS_FLAGS :=	-f elf64

## Rust kernel options
RUST_SRC :=			$(wildcard kernel/src/*.rs)

## Main binary
OBJNAME :=			cyubos.flp

# Targets

## Helper targets

.PHONY: all
all: $(OBJNAME)

.PHONY: clean
clean:
	rm -rf *.bin
	rm -rf *.o
	rm -rf kernel/target
	rm -rf kernel/Cargo.lock
	rm -rf *.tmp
	rm -rf *.flp

### The binaries making up the final thing

stage1.bin: $(STG1_SRC)
ifeq ($(DEBUG),)
	$(AS) $(STG1_AS_FLAGS) $(STG1_INC) -o $@ boot/stage1/stage1.asm
else
	$(AS) -g $(STG1_AS_FLAGS) $(STG1_INC) -o $@ boot/stage1/stage1.asm
endif

stage2.o: $(STG2_SRC)
ifeq ($(DEBUG),)
	$(AS) $(STG2_AS_FLAGS) $(STG2_INC) -o $@ boot/stage2/stage2.asm
else
	$(AS) -g $(STG2_AS_FLAGS) $(STG2_INC) -o $@ boot/stage2/stage2.asm
endif

kernel.o: $(RUST_SRC)
	cd kernel; cargo $(RUSTC_FLAGS)
ifeq ($(DEBUG),)
	cp kernel/target/x86_64-unknown-none/release/libcyub_os_kernel.a $@
else
	cp kernel/target/x86_64-unknown-none/debug/libcyub_os_kernel.a $@
endif

kernel.bin: stage2.o kernel.o
	ld -Tlink.ld

## Main targets
$(OBJNAME): stage1.bin kernel.bin
	rm -rf $@
	cat $^ >> $@
