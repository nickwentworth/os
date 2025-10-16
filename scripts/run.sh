set -e

cargo clean

mkdir target
aarch64-elf-as src/boot.s -o target/boot.o
aarch64-elf-as src/vec.s -o target/vec.o

cargo build --release
aarch64-elf-objcopy target/aarch64-unknown-none/release/os target/kernel.bin

qemu-system-aarch64 -M raspi4b -kernel target/kernel.bin -serial stdio -s
