set -e

cargo clean

mkdir target
aarch64-elf-as src/boot.s -o target/boot.o
aarch64-elf-as src/vec.s -o target/vec.o

cargo build
cp target/aarch64-unknown-none/debug/os target/kernel.elf
aarch64-elf-objcopy target/aarch64-unknown-none/debug/os target/kernel.bin

qemu-system-aarch64 \
    -machine virt -cpu cortex-a72 -m 4096M \
    -kernel target/kernel.elf \
    -nographic \
    -s
