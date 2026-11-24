# Patches

This fork of [Rhai](https://github.com/rhaiscript/rhai) carries the changes MailCraft needs on top
of a released version, one commit each, until they're upstream. MailCraft pins this branch by
commit in its `[patch.crates-io]`.

Base: `v1.27.0`.

1. **`smartstring` is optional** (the `smartstring` feature, on by default, as before). With the
   `compact_str` feature on and default features off, `smartstring` (MPL-2.0+, unmaintained:
   RUSTSEC-2026-0249) is no longer built or in the dependency graph. Without either feature, the
   build stops with an error saying so.
2. **A limit on how deep arrays and maps nest** (`Engine::set_max_data_depth`, off by default).
   Rhai measures, clones, drops and prints a value a level of the native stack at a time, and
   nothing bounded the nesting: `let a = []; loop { a = [a]; }` overflowed the stack and aborted
   the process before the array size limit was reached. The limit is checked wherever a value's
   size is (literals, assignments, function results, `pad`, Grain's literals). As with the size
   limits, a call or assignment on an element measures that element, so nesting through a path
   written in the script can pass the limit by that path's length, which the expression depth
   bounds.
