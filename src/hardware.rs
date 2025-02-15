use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use crate::cpu::Cpu;

pub trait Numeric {}
macro_rules! impl_numeric {
    ($($type:ty),*) => {
        $(
            impl Numeric for $type {}
        )*
    };
}

impl_numeric!(u8, u16, u32);

pub struct Config {
    pub cpu: Rc<RefCell<Cpu>>,
    pub id: u8,
}

pub trait Hardware {
    fn init(&mut self, config: Config);
    fn read(&self) -> u8;
    fn write(&mut self, b: u8);
}