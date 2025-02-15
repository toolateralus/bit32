use std::sync::{Arc, Mutex};

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
    pub cpu: Arc<Mutex<Cpu>>,
    pub id: u8,
}

pub trait Hardware {
    fn init(this: Arc<Mutex<Self>>, config: Config);
    fn read<T: Numeric>(&self) -> T;
    fn write<T: Numeric>(&mut self, b: T);
}