use quake3_qvm::bytecode::Address;
use std::collections::BTreeMap;

// Both Symbol and SymbolMap probably belong in quake3-qvm itself

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Symbol {
    value: Address,
    name: String,
}

impl Symbol {
    pub fn new<S>(value: Address, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value,
            name: name.into(),
        }
    }

    #[must_use]
    pub const fn value(&self) -> Address {
        self.value
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct SymbolMap {
    symbols: BTreeMap<Address, Symbol>,
}

impl Default for SymbolMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolMap {
    #[must_use]
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
        }
    }

    pub fn with_symbols<I>(symbols: I) -> Self
    where
        I: IntoIterator<Item = Symbol>,
    {
        let mut me = Self::new();
        for symbol in symbols {
            me.symbols.insert(symbol.value(), symbol);
        }
        me
    }

    #[must_use]
    pub fn symbol_name(&self, address: Address) -> Option<String> {
        use std::ops::Bound::Included;

        if self.symbols.is_empty() {
            None
        } else {
            let range = self
                .symbols
                .range((Included(Address::MIN), Included(address)));
            let closest = range.last().expect("symbols has at least one entry").1;
            if closest.value() == address {
                Some(closest.name().to_string())
            } else {
                Some(format!("{}+{}", closest.name(), address - closest.value()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_map_name_empty() {
        let map = SymbolMap::new();
        assert_eq!(map.symbol_name(42), None);
    }

    #[test]
    fn symbol_map_name_closest() {
        let map = SymbolMap::with_symbols(vec![Symbol::new(40, "name")]);
        assert_eq!(map.symbol_name(42), Some("name+2".into()));
    }

    #[test]
    fn symbol_map_name_exact() {
        let map = SymbolMap::with_symbols(vec![Symbol::new(42, "name")]);
        assert_eq!(map.symbol_name(42), Some("name".into()));
    }
}
