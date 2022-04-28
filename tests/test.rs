use std::collections::{hash_map::Entry, HashMap};
use symbol_table::*;

static TEXT: &str = include_str!("../tests/gulliver.txt");

#[test]
fn test_resolve() {
    let interner = SymbolTable::new();
    let mut resolved = Vec::new();
    let mut map = HashMap::new();

    for word in TEXT.split_whitespace().chain(TEXT.split_whitespace().rev()) {
        let sym = interner.intern(word);
        match map.entry(word) {
            Entry::Occupied(e) => assert_eq!(*e.get(), sym),
            Entry::Vacant(e) => {
                e.insert(sym);
            }
        }
        resolved.push(interner.resolve(sym));
        assert_eq!(interner.resolve(sym), word);
    }
}

#[cfg(feature = "global")]
#[test]
fn test_global() {
    let mut map = HashMap::new();

    for word in TEXT.split_whitespace().chain(TEXT.split_whitespace().rev()) {
        let sym = GlobalSymbol::from(word);
        match map.entry(word) {
            Entry::Occupied(e) => {
                assert_eq!(*e.get(), sym);
            }
            Entry::Vacant(e) => {
                e.insert(sym);
            }
        }

        assert_eq!(sym.to_string(), word);
        assert_eq!(sym.as_str(), word);
    }
}

#[test]
fn test_specific_strings() {
    let interner = SymbolTable::new();
    let strings = ["", "asdf", "🧵"];
    for word in strings {
        let sym = interner.intern(word);
        assert_eq!(interner.resolve(sym), word);
    }
}

#[cfg(feature = "global")]
#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    let sym = GlobalSymbol::from("foo");

    fn ser(_: impl serde::Serialize) {}
    fn de<'a>(_: impl serde::Deserialize<'a>) {}

    ser(sym);
    de(sym);
}
