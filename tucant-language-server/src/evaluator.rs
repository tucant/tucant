use num_bigint::BigUint;
use num_integer::Integer;

use crate::parser::Ast;

pub enum RootType {
    AddFunction,
    LetBinding,
    LambdaDefinition,
    Identifier,
    Eval,
}

impl RootType {
    #[allow(unused, clippy::needless_pass_by_value)]
    fn execute(&self, _data: BigUint) {
        match self {
            Self::Eval => {}
            _ => todo!(),
        }
    }
}

impl AnyData for RootType {
    fn get_possibilities(&self) -> BigUint {
        BigUint::from(5u8)
    }
}

impl From<RootType> for BigUint {
    fn from(value: RootType) -> Self {
        match value {
            RootType::AddFunction => Self::from(0u8),
            RootType::LetBinding => Self::from(1u8),
            RootType::LambdaDefinition => Self::from(2u8),
            RootType::Identifier => Self::from(3u8),
            RootType::Eval => Self::from(4u8),
        }
    }
}

pub trait AnyData {
    fn get_possibilities(&self) -> BigUint;
}

pub struct Bool(bool);

impl AnyData for Bool {
    fn get_possibilities(&self) -> BigUint {
        BigUint::from(2u8)
    }
}

impl From<Bool> for BigUint {
    fn from(value: Bool) -> Self {
        Self::from(u8::from(value.0))
    }
}

pub struct I64(i64);

impl From<I64> for BigUint {
    fn from(value: I64) -> Self {
        #[allow(clippy::cast_sign_loss)]
        Self::from(value.0 as u64)
    }
}

impl AnyData for Ast {
    fn get_possibilities(&self) -> BigUint {
        todo!()
    }
}

impl From<BigUint> for Ast {
    fn from(_value: BigUint) -> Self {
        todo!()
    }
}

impl From<Ast> for BigUint {
    fn from(value: Ast) -> Self {
        let (_base, _data): (u8, Self) = match value {
            Ast::Number(v) => (0, I64(v).into()),
            Ast::String(_v) => todo!(),
            Ast::Identifier(_v) => todo!(),
            Ast::List(_v) => todo!(),
        };
        todo!()
    }
}

pub trait Address
where
    Self::AllocatorType: Allocator,
{
    type AllocatorType;

    fn set(&self, allocator: &mut Self::AllocatorType, value: BigUint);
    fn get(&self, allocator: &Self::AllocatorType) -> BigUint;
}

pub trait Allocator
where
    Self::AddressType: Address,
{
    type AddressType;

    fn allocate(&mut self, possibilities: BigUint) -> Self::AddressType;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BumpOnlyAllocator {
    possibilities: BigUint,
    inner: BigUint,
}

impl BumpOnlyAllocator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            possibilities: BigUint::from(1u8),
            inner: BigUint::from(0u8),
        }
    }
}

// memory: 0
// store new value: 5
// memory: 5
// store new value: 7 (dont move old values!), value * memory_possibilities + old_memory

impl Allocator for BumpOnlyAllocator {
    type AddressType = BumpOnlyAddress;

    fn allocate(&mut self, possibilities: BigUint) -> Self::AddressType {
        if possibilities == BigUint::from(0u8) {
            Self::AddressType {
                possibilities,
                address: BigUint::from(1u8),
            }
        } else {
            let address = self.possibilities.clone();
            self.possibilities *= &possibilities;
            Self::AddressType {
                possibilities,
                address,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BumpOnlyAddress {
    possibilities: BigUint,
    address: BigUint,
}

impl Address for BumpOnlyAddress {
    type AllocatorType = BumpOnlyAllocator;

    fn set(&self, allocator: &mut BumpOnlyAllocator, new_value: BigUint) {
        assert!(new_value < self.possibilities);

        let (your_value_and_higher_values, lower_values) = allocator.inner.div_rem(&self.address);

        let (higher_values, _our_value) = your_value_and_higher_values.div_rem(&self.possibilities);

        let new_your_value_and_higher_values = higher_values * &self.possibilities + &new_value;

        allocator.inner = new_your_value_and_higher_values * &self.address + lower_values;
    }

    fn get(&self, allocator: &BumpOnlyAllocator) -> BigUint {
        let (your_value_and_higher_values, _lower_values) = allocator.inner.div_rem(&self.address);

        // TODO FIXME should you be able to get a value when there are no possibilities?
        let (_higher_values, our_value) = your_value_and_higher_values.div_rem(&self.possibilities);

        assert!(our_value < self.possibilities);
        our_value
    }
}

#[test]
fn test_allocator() {
    let mut allocator = BumpOnlyAllocator::new();
    println!("{allocator:?}");

    let addr0 = BumpOnlyAllocator::allocate(&mut allocator, BigUint::from(7u8));
    println!("{addr0:?}");

    addr0.set(&mut allocator, BigUint::from(0u8));
    println!("{addr0:?}");

    addr0.set(&mut allocator, BigUint::from(5u8));
    println!("{addr0:?}");

    assert_eq!(addr0.get(&allocator), BigUint::from(5u8));

    let addr1 = BumpOnlyAllocator::allocate(&mut allocator, BigUint::from(11u8));
    println!("{addr1:?}");

    addr1.set(&mut allocator, BigUint::from(3u8));
    println!("{allocator:?}");

    assert_eq!(addr1.get(&allocator), BigUint::from(3u8));

    assert_eq!(addr0.get(&allocator), BigUint::from(5u8));
}

// https://github.com/rust-lang/rust-analyzer/issues/12661
#[cfg(test)]
mod tests {
    use crate::evaluator::RootType;
    use crate::parser::{parse, TokenizerBuilder};

    #[test]
    #[ignore]
    fn test_eval() {
        let span = TokenizerBuilder::from_string(r#"(add 1 1)"#);
        let value = parse(&mut span.peekable()).unwrap();
        println!("{value:?}");

        RootType::Eval.execute(value.0.into());
    }
}
