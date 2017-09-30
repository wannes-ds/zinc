#!/bin/bash
set +x
arm-none-eabi-gcc "-mtune=cortex-a8" "-Wl,--defsym,STACKSIZE=0x1C000" "-Wl,--defsym,HEAPSIZE=0x400" "-mfloat-abi=hard" "-Tlayout.ld" "$@" "-lm" "-lgcc" "-lnosys"
