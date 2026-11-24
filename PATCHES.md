# Patches

This fork of [Rhai](https://github.com/rhaiscript/rhai) carries the changes MailCraft needs on top
of a released version, one commit each, until they're upstream. MailCraft pins this branch by
commit in its `[patch.crates-io]`.

Base: `v1.27.0`.

1. **`smartstring` is optional** (the `smartstring` feature, on by default, as before). With the
   `compact_str` feature on and default features off, `smartstring` (MPL-2.0+, unmaintained:
   RUSTSEC-2026-0249) is no longer built or in the dependency graph. Without either feature, the
   build stops with an error saying so.
