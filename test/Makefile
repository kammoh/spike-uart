OS_NAME := $(shell uname -s)

RISCV_TOOLCHAIN = riscv64-unknown-elf

RISCV_CC = $(RISCV_TOOLCHAIN)-gcc
RISCV_LD = $(RISCV_TOOLCHAIN)-ld

RISCV_CFLAGS=-mcmodel=medany -static -std=gnu99 -O2 -fno-common -fno-builtin-printf -fno-tree-loop-distribute-patterns
#-static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles
RISCV_LDFLAGS=-static -nostdlib -nostartfiles -Tlink.ld 

plugin_test: plugin_test.S Makefile
	riscv64-unknown-elf-gcc -nostdlib -static -Wl,-Ttext-segment,0x80000000 plugin_test.S -o plugin_test

%.riscv: %.c link.ld crt.S
	$(RISCV_CC) $(RISCV_CFLAGS) $(RISCV_LDFLAGS) $< syscalls.c crt.S -o $@

%.riscv: %.S link.ld
	$(RISCV_CC) $(RISCV_CFLAGS) -T link.ld $< -o $@

%.riscv.dump: %.riscv
	riscv64-unknown-elf-objdump -d $< > $@
	cat $@

################ C plugin #################
CFLAGS = -Wall -Werror -fPIC

ifeq ($(OS_NAME),Darwin)
	CFLAGS += -bundle -flat_namespace -undefined suppress -I$(shell brew --prefix)/include
else
	CFLAGS += -shared
endif

plugin.so: plugin.c
	$(CC) $(CFLAGS) -o plugin.so plugin.c

.PHONY: test clean

test: plugin_test plugin.so
	spike -m1 --extlib=plugin.so --device=uart_plugin,0x10000000,argument plugin_test

clean:
	$(RM) -f plugin_test
	$(RM) -f plugin.so

