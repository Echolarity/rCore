# 变量定义
QEMU := qemu-system-riscv64
MACHINE := virt
NOGRAPHIC := -nographic
BIOS := ../bootloader/rustsbi-qemu.bin
KERNEL_BIN := target/riscv64gc-unknown-none-elf/release/os.bin
KERNEL_ELF := target/riscv64gc-unknown-none-elf/release/os
ADDR := 0x80200000
OBJCOPY := riscv64-unknown-elf-objcopy

# 日志等级，默认 INFO，如果 LOG 没有传入则使用 INFO
LOG_LEVEL ?= $(LOG)
LOG_LEVEL := $(if $(LOG_LEVEL),$(LOG_LEVEL),INFO)

# 根据 LOG_LEVEL 设置编译特性 (编译时使用)
ifeq ($(LOG_LEVEL),TRACE)
    RUST_FEATURE := --features log_trace
else ifeq ($(LOG_LEVEL),DEBUG)
    RUST_FEATURE := --features log_debug
else ifeq ($(LOG_LEVEL),INFO)
    RUST_FEATURE := --features log_info
else ifeq ($(LOG_LEVEL),WARN)
    RUST_FEATURE := --features log_warn
else ifeq ($(LOG_LEVEL),ERROR)
    RUST_FEATURE := --features log_error
else
    RUST_FEATURE := --features log_info  # 默认使用INFO
endif

# 编译并生成二进制文件
.PHONY: build
build:
	cargo build --release $(RUST_FEATURE)
	$(OBJCOPY) -O binary $(KERNEL_ELF) $(KERNEL_BIN)

# 运行 QEMU 并传递日志等级
.PHONY: run
run: build
	@echo "Running with LOG level: $(LOG_LEVEL)"
	@export LOG=$(LOG_LEVEL); $(QEMU) -machine $(MACHINE)  $(NOGRAPHIC) -bios $(BIOS) -device loader,file=$(KERNEL_BIN),addr=$(ADDR) || \
	{ echo "QEMU exited with code $?"; exit 0; }

# 清理构建文件
.PHONY: clean
clean:
	cargo clean
