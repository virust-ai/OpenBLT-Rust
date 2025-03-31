#![no_std]

pub struct Peripheral<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Peripheral<T> {
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn into_ref(self) -> PeripheralRef<T> {
        PeripheralRef {
            _phantom: self._phantom,
        }
    }
}

pub struct PeripheralRef<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> From<Peripheral<T>> for PeripheralRef<T> {
    fn from(peripheral: Peripheral<T>) -> Self {
        peripheral.into_ref()
    }
} 
