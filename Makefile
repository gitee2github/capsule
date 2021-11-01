KERNEL_DIR := /lib/modules/5.14.0-rc3+/build
PWD := $(shell pwd)

all:
	make -C $(KERNEL_DIR) M=$(PWD) LLVM=1

clean:
	make -C $(KERNEL_DIR) M=$(PWD) LLVM=1 clean


obj-m		+= capsule-core.o

capsule-core-y += main.o