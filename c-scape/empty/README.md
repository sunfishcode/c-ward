This directory contains empty static libraries such as libc.a, libm.a, and
others, to prevent the linker from opening the platform's version of those
libraries, when we're in "take charge" mode.
