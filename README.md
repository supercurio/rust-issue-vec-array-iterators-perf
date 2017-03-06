# Test and demo for rust-lang issue #40310
https://github.com/rust-lang/rust/issues/40310

This uses an intentionally simplified implementation of a 2nd order Biquad filter,
which is at the center of most audio DSP software.
The code is modeled after a working example but stripped of what's not strictl
required in order to make the MIR and ASM outputs easier to read.

The demo however replicates the same performance loss observed with the real
Biquad IIR filter.

### About build-all-targets.sh

This script helps to build binaries for various arm, armv7 and aarch64 platforms
provided the target toolchains are installed.