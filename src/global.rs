use crate::*;

use std::mem::MaybeUninit;
use std::str::FromStr;
use std::sync::Once;

/// A interned string in the global symbol table.
///
/// This requires the `global` feature on the crate.
///
/// [`GlobalSymbol`] is a wrapper around [`Symbol`] that knows to refer to a
/// built-in, global [`SymbolTable`]. Strings into the global table are never freed.
///
/// This enables a lot of convience methods and trait implementations over
/// [`GlobalSymbol`] (see below). In particular,
///   you can convert it to `&'static str`,
///   convert [`From`] and [`Into`] a `&str`,
///   and de/serialize using [`serde`](https://serde.rs) if the `serde` feature is enabled.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "&str", into = "&'static str"))]
pub struct GlobalSymbol(Symbol);

impl From<NonZeroU32> for GlobalSymbol {
    fn from(n: NonZeroU32) -> Self {
        Self(Symbol::from(n))
    }
}

impl From<GlobalSymbol> for NonZeroU32 {
    fn from(n: GlobalSymbol) -> Self {
        n.0.into()
    }
}

fn singleton() -> &'static SymbolTable {
    static mut SINGLETON: MaybeUninit<SymbolTable> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    // SAFETY:
    // - writing to the singleton is OK because we only do it one time
    // - the ONCE guarantees that SINGLETON is init'ed before assume_init_ref
    unsafe {
        ONCE.call_once(|| {
            SINGLETON.write(SymbolTable::new());
        });
        SINGLETON.assume_init_ref()
    }
}

impl GlobalSymbol {
    /// Intern a string into the global symbol table.
    pub fn new(s: impl AsRef<str>) -> Self {
        s.as_ref().into()
    }

    /// Convert this symbol into the string in the static, global symbol table.
    pub fn as_str(&self) -> &'static str {
        (*self).into()
    }
}

impl From<&str> for GlobalSymbol {
    fn from(s: &str) -> Self {
        GlobalSymbol(singleton().intern(s))
    }
}

impl From<String> for GlobalSymbol {
    fn from(s: String) -> Self {
        GlobalSymbol(singleton().intern(&s))
    }
}

impl From<&String> for GlobalSymbol {
    fn from(s: &String) -> Self {
        GlobalSymbol(singleton().intern(s))
    }
}

impl FromStr for GlobalSymbol {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<GlobalSymbol> for &'static str {
    fn from(sym: GlobalSymbol) -> Self {
        singleton().resolve(sym.0)
    }
}

impl std::fmt::Debug for GlobalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl std::fmt::Display for GlobalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}
