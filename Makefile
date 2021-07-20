TARGET := kernel/target/KRust-x86_64/debug/bootimage-KRust.bin
DIRECTORIES := vga/ kernel/  # $(sort $(dir $(wildcard ./*/)))

build:
	@for folders in $(DIRECTORIES) ; do \
		echo -n "Compiling " && \
		echo -n $$folders && \
		echo -e " ...\n" && \
		cd $$folders && cargo build && cd ../ && echo -e '\n' ; \
	done

	cd kernel && cargo bootimage

run:
	qemu-system-x86_64 -drive format=raw,file=$(TARGET)

setup:
	rustup override set nightly
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	rustup component add llvm-tools-preview
	cargo install bootimage

clean:
	for folders in $(DIRECTORIES); do \
		cd folders && cargo clean; \
	done;
