use std::{cell::RefCell, rc::Rc};

use num_bigint::BigUint;
use num_integer::Integer;

use crate::parser::{parse, Ast, TokenizerBuilder};

pub enum RootType {
    AddFunction,
    LetBinding,
    LambdaDefinition,
    Identifier,
    Eval,
}

impl RootType {
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
        Self::from(match value.0 {
            true => 0u8,
            false => 1u8,
        })
    }
}

pub struct I64(i64);

impl From<I64> for BigUint {
    fn from(value: I64) -> Self {
        Self::from(TryInto::<u64>::try_into(i128::from(value.0) - i128::from(i64::MIN)).unwrap())
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
            Ast::String(_v) => (1, todo!()),
            Ast::Identifier(_v) => (2, todo!()),
            Ast::List(_v) => (3, todo!()),
        };
        todo!()
    }
}

pub trait Address {
    fn set(&mut self, value: BigUint);
    fn get(&self) -> BigUint;
}

pub trait Allocator
where
    Self::AddressType: Address,
{
    type AddressType;

    fn allocate(this: Rc<RefCell<Self>>, possibilities: BigUint) -> Self::AddressType;
}

#[derive(Debug)]
pub struct BumpOnlyAllocator {
    possibilities: BigUint,
    inner: BigUint,
}

impl BumpOnlyAllocator {
    #[must_use]
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            possibilities: BigUint::from(1u8),
            inner: BigUint::from(0u8),
        }))
    }
}

// memory: 0
// store new value: 5
// memory: 5
// store new value: 7 (dont move old values!), value * memory_possibilities + old_memory

impl Allocator for BumpOnlyAllocator {
    type AddressType = BumpOnlyAddress;

    fn allocate(this: Rc<RefCell<Self>>, possibilities: BigUint) -> Self::AddressType {
        let address = this.borrow().possibilities.clone();
        this.borrow_mut().possibilities *= &possibilities;
        Self::AddressType {
            allocator: this,
            possibilities,
            address,
        }
    }
}

#[derive(Debug)]
pub struct BumpOnlyAddress {
    allocator: Rc<RefCell<BumpOnlyAllocator>>,
    #[cfg(debug_assertions)]
    possibilities: BigUint,
    address: BigUint,
}

impl Address for BumpOnlyAddress {
    fn set(&mut self, new_value: BigUint) {
        debug_assert!(new_value < self.possibilities);

        let mut allocator_value = self.allocator.borrow_mut();

        let (your_value_and_higher_values, lower_values) =
            allocator_value.inner.div_rem(&self.address);

        let (higher_values, _our_value) = your_value_and_higher_values.div_rem(&self.possibilities);

        let new_your_value_and_higher_values = higher_values * &self.possibilities + &new_value;

        allocator_value.inner = new_your_value_and_higher_values * &self.address + lower_values;
    }

    fn get(&self) -> BigUint {
        let allocator_value = self.allocator.borrow();

        let (your_value_and_higher_values, _lower_values) =
            allocator_value.inner.div_rem(&self.address);

        let (_higher_values, our_value) = your_value_and_higher_values.div_rem(&self.possibilities);

        debug_assert!(our_value < self.possibilities);
        our_value
    }
}

#[test]
fn test_allocator() {
    let allocator = BumpOnlyAllocator::new();
    println!("{:?}", allocator);

    let mut addr0 = BumpOnlyAllocator::allocate(allocator.clone(), BigUint::from(7u8));
    println!("{:?}", addr0);

    addr0.set(BigUint::from(0u8));
    println!("{:?}", addr0);

    addr0.set(BigUint::from(5u8));
    println!("{:?}", addr0);

    assert_eq!(addr0.get(), BigUint::from(5u8));

    let mut addr1 = BumpOnlyAllocator::allocate(allocator.clone(), BigUint::from(11u8));
    println!("{:?}", addr1);

    addr1.set(BigUint::from(3u8));
    println!("{:?}", allocator);

    assert_eq!(addr1.get(), BigUint::from(3u8));

    assert_eq!(addr0.get(), BigUint::from(5u8));
}

#[test]
fn test_eval() {
    let span = TokenizerBuilder::from_string(r#"(add 1 1)"#);
    let value = parse(&mut span.peekable()).unwrap();
    println!("{value:?}");

    RootType::Eval.execute(value.0.into())
}
