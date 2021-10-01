OS_NAME := $(shell uname -s)

plugin_test: plugin_test.S
	riscv64-unknown-elf-gcc -nostdlib -static -Wl,-Ttext-segment,0x80000000 plugin_test.S -o plugin_test

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
	spike -m1 --extlib=plugin.so --device=test_mmio_plugin,0x10000000,argument plugin_test

clean:
	$(RM) -f plugin_test
	$(RM) -f plugin.so

