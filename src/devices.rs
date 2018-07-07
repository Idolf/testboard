use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;

/// Trait for zero-sized marker traits the represents ownership over a device.
pub trait Device {}

static GLOBAL_UNIT_VALUE: () = ();
static GLOBAL_UNIT_VALUE_PTR: &() = &GLOBAL_UNIT_VALUE;

pub struct Finalized<T: 'static>(PhantomData<T>);

impl<T> Finalized<T> {
    #[inline]
    pub(crate) fn new(value: T) -> Finalized<T> {
        mem::forget(value);
        Finalized(PhantomData)
    }

    #[inline]
    pub fn get_ref(&self) -> &'static T {
        let result: &'static () = &GLOBAL_UNIT_VALUE;
        unsafe { ::core::mem::transmute(result) }
    }
}

impl<T> Deref for Finalized<T> {
    type Target = &'static T;
    fn deref(&self) -> &&'static T {
        let result: &&'static () = &GLOBAL_UNIT_VALUE_PTR;
        unsafe { ::core::mem::transmute(result) }
    }
}

/// Trait for devices for which it makes sense to "finalized" them, i.e. turn
pub trait FinalizeDevice: Device + Sized + 'static {
    /// Locks in the state of the device, so it will not be changed again nor will it be disabled on
    /// drop.
    fn finalize(self) -> Finalized<Self>;
}
