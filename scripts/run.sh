set -e

# -------------------- Parse Args -------------------- #

DEBUG=false
RELEASE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --debugger) DEBUG=true;     shift;;
        --release)  RELEASE=true;   shift ;;
        *) echo "Unknown option: $1"; exit 1;;
    esac
done

if $DEBUG; then
    QEMU_FLAGS="-s -S"
fi

if $RELEASE; then
    TARGET_DIR="release"
    CARGO_FLAGS="--release"
else
    TARGET_DIR="debug"
fi

# -------------------- Clean -------------------- #

cargo clean

# -------------------- Build -------------------- #

mkdir target
aarch64-elf-as src/boot.s -o target/boot.o
aarch64-elf-as src/vec.s -o target/vec.o

cargo build $CARGO_FLAGS

if $DEBUG; then
    # directly provide the built .elf for gdb, only needed if debugging
    cp target/aarch64-unknown-none/$TARGET_DIR/os target/kernel.elf
fi

# always provide a .bin for qemu
aarch64-elf-objcopy -O binary target/aarch64-unknown-none/$TARGET_DIR/os target/kernel.bin

# -------------------- Run -------------------- #

qemu-system-aarch64 \
    -machine virt -cpu cortex-a72 -m 4096M \
    -kernel target/kernel.bin \
    -nographic \
    $QEMU_FLAGS
