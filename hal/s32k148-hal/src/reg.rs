#![no_std]

use core::cell::UnsafeCell;

pub trait RegisterValue: Copy + Clone {
    fn set_re(&mut self, value: u8) -> &mut Self;
    fn set_te(&mut self, value: u8) -> &mut Self;
    fn set_sbr(&mut self, value: u8) -> &mut Self;
    fn set_m(&mut self, value: u8) -> &mut Self;
    fn set_pe(&mut self, value: u8) -> &mut Self;
    fn set_data(&mut self, value: u8) -> &mut Self;
    fn tdre(&self) -> bool;
    fn rdrf(&self) -> bool;
    fn data(&self) -> u8;
}

impl RegisterValue for u8 {
    fn set_re(&mut self, value: u8) -> &mut Self {
        *self = (*self & !0x04) | ((value & 0x01) << 2);
        self
    }

    fn set_te(&mut self, value: u8) -> &mut Self {
        *self = (*self & !0x08) | ((value & 0x01) << 3);
        self
    }

    fn set_sbr(&mut self, value: u8) -> &mut Self {
        *self = (*self & !0x1F) | (value & 0x1F);
        self
    }

    fn set_m(&mut self, value: u8) -> &mut Self {
        *self = (*self & !0x10) | ((value & 0x01) << 4);
        self
    }

    fn set_pe(&mut self, value: u8) -> &mut Self {
        *self = (*self & !0x02) | ((value & 0x01) << 1);
        self
    }

    fn set_data(&mut self, value: u8) -> &mut Self {
        *self = value;
        self
    }

    fn tdre(&self) -> bool {
        (*self & 0x80) != 0
    }

    fn rdrf(&self) -> bool {
        (*self & 0x20) != 0
    }

    fn data(&self) -> u8 {
        *self
    }
}

pub struct Register<T: RegisterValue> {
    value: UnsafeCell<T>,
}

impl<T: RegisterValue> Register<T> {
    pub fn read(&self) -> T {
        unsafe { *self.value.get() }
    }

    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        unsafe {
            let mut value = self.read();
            f(&mut value);
            *self.value.get() = value;
        }
    }

    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(T, &mut T),
    {
        unsafe {
            let value = self.read();
            let mut new_value = value;
            f(value, &mut new_value);
            *self.value.get() = new_value;
        }
    }
} 
