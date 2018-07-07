use super::{Clock, LfRco, LfXo};
use efm32hg309f64;

clock_switch!(
    /// This type represents ownership over the `LFCCLK`, the Low-Frequency Clock C. The `LFCCLK`
    /// can be used to drive the USB low-energy clock.
    LfcClk
);

clock_switch!(
    /// This type represents ownership over the `LFCCLKUSBLE`, the Low-Energy clock for USB.
    LfcClkUsbLe
);

macro_rules! lfc_source {
    ($meth:ident $fun:ident $typ:ident) => {
        /// Enables the `LFCCLK` and sets its source by setting the `LFC` subfield in `CMU_LFCLKSEL`.
        #[inline]
        pub fn $meth<'new_source, Frequency>(
            self,
            $fun: &'new_source $typ<Frequency>,
        ) -> LfcClk<'new_source, $typ<Frequency>>
        where
            $typ<Frequency>: Clock
        {
            let _ = $fun;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.lfclksel.modify(|_, w| w.lfc().$fun());
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source> LfcClk<'source, Source> {
    lfc_source!(enable_lfrco lfrco LfRco);
    lfc_source!(enable_lfxo lfxo LfXo);

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.lfclksel.modify(|_, w| w.lfc().disabled());
    }

    /// Disables the `LFCCLK` by clearing the `LFC` subfield in `CMU_LFCLKSEL`.
    #[inline]
    pub fn disable(mut self) -> LfcClk<'static, super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}

impl<'source, Source> LfcClkUsbLe<'source, Source> {
    /// Enables the `LFCCLKUSBLE` by setting the `USBLE` bit in `CMU_LFCCLKEN0`.
    ///
    /// This function will not write to `CMU_LFCCLKEN0` until it the relevant bit in `CMU_SYNCBUSY`
    /// is clear.
    #[inline]
    pub fn enable<'new_source, InnerSource>(
        self,
        lfcclk: &'new_source LfcClk<'new_source, InnerSource>,
    ) -> LfcClkUsbLe<'new_source, LfcClk<'new_source, InnerSource>>
    where
        LfcClk<'new_source, InnerSource>: Clock,
    {
        let _ = lfcclk;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

        while cmu.syncbusy.read().lfcclken0().bit_is_set() {}
        cmu.lfcclken0.write(|w| w.usble().set_bit());
        unsafe { self.transmute_state() }
    }

    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

        while cmu.syncbusy.read().lfcclken0().bit_is_set() {}
        cmu.lfcclken0.write(|w| w.usble().clear_bit());
    }

    /// Disables the `LFCCLKUSBLE` by clearing the `USBLE` bit in `CMU_LFCCLKEN0`.
    ///
    /// This function will not write to `CMU_LFCCLKEN0` until it the relevant bit in `CMU_SYNCBUSY`
    /// is clear.
    #[inline]
    pub fn disable(mut self) -> LfcClkUsbLe<'static, super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}
