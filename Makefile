TARGET := Kernel/target/KRust-x86_64/debug/bootimage-KRust.bin

build:
	cd Kernel && \
	cargo build && \
	cargo bootimage

run:
	qemu-system-x86_64 -drive format=raw,file=$(TARGET)

setup:
	rustup override set nightly
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
	rustup component add llvm-tools-preview
	cargo install bootimage

clean:
	cd Kernel && \
	cargo clean
