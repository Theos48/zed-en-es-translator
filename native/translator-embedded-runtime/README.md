# translator-embedded-runtime

This directory owns the minimal C++17 one-shot boundary planned by ADR 0006.
Its production configuration links the exact Mozilla/Bergamot source recorded
in `ops/providers/embedded/source.lock.json`, accepts only the three fixed
verified object roles, and implements one ordered JSON batch. Building the
candidate does not approve or activate it: model/runtime license review and
real offline acceptance remain mandatory promotion gates.

A separate compile-time controlled-fixture mode exists only to test the strict
wire/process contract with public synthetic text; it is not eligible for
installation or promotion.

The executable accepts one bounded versioned JSON request on stdin,
returns one bounded JSON response on stdout, performs no network or update
operation, and exits. It is built only through the project container; no host
C++ toolchain is installed for this repository.
