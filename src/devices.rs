use core::mem;

/// Trait for zero-sized marker traits the represents ownership over a device.
pub trait Device {}

/// Makes an owned zero-sized struct into static mutable reference to the same struct
#[inline]
pub(crate) unsafe fn make_static_mut<T: Sized + 'static>(val: T) -> &'static mut T {
    assert_eq!(mem::size_of_val(&val), 0);
    mem::forget(val);

    static GLOBAL_UNIT_VALUE: () = ();
    let result: &'static () = &GLOBAL_UNIT_VALUE;
    &mut *(result as *const () as *mut T)
}

/// Trait for devices for which it makes sense to "finalized" them, i.e. turn
pub trait StaticDevice: Device + Sized + 'static {
    /// Locks in the state of the device, so it will not be changed again nor will its destructors
    /// (if any) be run.
    fn finalize(self) -> &'static mut Self;
}
