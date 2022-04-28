# `symbol_table`

This crate provides an easy-to-use `SymbolTable`
 that's fast, suitable for concurrent access,
 and provides stable `&str` references for resolved symbols.

With the `global` feature enabled, the
 provided `GlobalSymbol` type
 provides a lot of convenience methods and trait implementations
 for converting to/from strings.