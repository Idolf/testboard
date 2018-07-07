use efm32hg309f64;

clock_source!(
    /// This type represents ownership over the `LFRCO`, the Low-Frequency RC Oscillator, which
    /// oscillates at 32768 Hz (except when turned off).
    LfRco
);

clock_source!(
    /// This type represents ownership over the `ULFRCO`, the Ultra Low-Frequency RC Oscillator,
    /// which is always oscillates at 1000 Hz.
    ///
    /// # Drop semantics
    /// This clock cannot be turned off in the hardware or even configured in any ways.
    ///
    /// This means that unlike most other clocks, dropping the value will not turn off the clock.
    ULfRco
);

impl<Frequency> LfRco<Frequency> {
    /// Enables the `LFRCO` by setting the `LFRCOEN` bit in `CMU_OSENCMD`.
    ///
    /// This function will block until the `LFRCO` as ready, by waiting for the `LFRCORDY`
    /// bit to be set in `CMU_STATUS`.
    #[inline]
    pub fn enable_32768hz(self) -> LfRco<super::Hz32768> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

        cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
        while cmu.status.read().lfrcordy().bit_is_clear() {}

        unsafe { self.transmute_state() }
    }

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfrcodis().set_bit());
    }

    /// Enables the `LFRCO` by setting the `LFRCODIS` bit in `CMU_OSENCMD`.
    #[inline]
    pub fn disable(mut self) -> LfRco<super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}

impl<Frequency> ULfRco<Frequency> {
    #[inline]
    fn _disable(&mut self) {}
}
