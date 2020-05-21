use quake3_qvm::{QVM, Instruction, bytecode::Address};
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

pub const STACK_SIZE: usize = 0x10000;

#[derive(Debug)]
pub struct Interpreter {
    instructions: Vec<Instruction>,
    program_counter: usize,
    stack_pointer: usize,
    data: Vec<u8>,
}

impl Interpreter {
    #[must_use]
    pub fn new(qvm: QVM) -> Self {
        let qvm_data = qvm.data();
        let lit = qvm.lit();
        let len: usize = qvm_data.len() * 4 + lit.len() + qvm.bss_length() as usize * 4;
        println!("len {}", len);
        let mut data: Vec<u8> = Vec::with_capacity(len);
        for word in qvm_data {
            data.push(((word & 0x000000FF) >>  0) as u8);
            data.push(((word & 0x0000FF00) >>  8) as u8);
            data.push(((word & 0x00FF0000) >> 16) as u8);
            data.push(((word & 0xFF000000) >> 24) as u8);
        }
        for byte in lit {
            data.push(*byte);
        }
        println!("data {:?}", data);

        Self {
            instructions: qvm.instructions().iter().map(|i| i.clone()).collect(), // FIXME: QVM API
            program_counter: 0,
            stack_pointer: len,
            data: data,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), ()> {
        todo!();
    }

    fn execute(&mut self) -> Result<(), ()> {
        todo!();
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

    #[test]
    fn interpreter_new() {
        let qvm = QVM::new(vec![Instruction::UNDEF], vec![0xDEADBEEF], vec![], 0).unwrap();
        let interpreter = Interpreter::new(qvm);
        eprintln!("interpreter {:?}", interpreter);
        assert_eq!(interpreter.program_counter, 0);
        assert_eq!(interpreter.stack_pointer, 4);
        assert_eq!(interpreter.data[0], 0xEF);
        assert_eq!(interpreter.data[1], 0xBE);
        assert_eq!(interpreter.data[2], 0xAD);
        assert_eq!(interpreter.data[3], 0xDE);
        assert_eq!(interpreter.instructions, vec![Instruction::UNDEF]);
    }
}
