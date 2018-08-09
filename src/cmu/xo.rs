use efm32hg309f64;

clock_source!(
    /// This type represents ownership over the `HFXO`, the High-Frequency Crystal Oscillator, which
    /// uses an external crystal to oscillate at 4-25MHz.
    ///
    /// Support for this clock has not been implemented yet.
    HfXo
);
clock_source!(
    /// This type represents ownership over the `LFXO`, the Low-Frequency Crystal Oscillator, which
    /// uses an external crystal to oscillate at 32768Hz.
    ///
    /// Support for this clock has not been implemented yet.
    LfXo
);

impl<Frequency> HfXo<Frequency> {
    /// Disables the `HFXO` by setting the `HFXODIS` bitsin `CMU_OSENCMD`.
    #[inline]
    pub fn disable(self) -> HfXo<super::Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfxodis().set_bit());
        unsafe { self.transmute_state() }
    }
}

impl<Frequency> LfXo<Frequency> {
    /// Disables the `LFXO` by setting the `HFXODIS` bitsin `CMU_OSENCMD`.
    #[inline]
    pub fn disable(self) -> LfXo<super::Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfxodis().set_bit());
        unsafe { self.transmute_state() }
    }
}
