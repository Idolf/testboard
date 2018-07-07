use super::{Clock, LfRco, LfXo, UsHfRco};
use efm32hg309f64;

clock_switch!(
    /// This type represents ownership over the `HFCORECLKUSBC`, the High-Frequency Clock used for
    /// the USB Core peripheral.
    ///
    /// # Quirks
    /// The name of this clock implies that it is sourced from the `HFCORECLK`, however it can
    /// actually be sourced from either the `LFXO`, `LFRCO` or `USHFRCO`.
    HfCoreClkUsbC
);

macro_rules! hfcoreclkusbc_source {
    ($meth:ident $typ:ident $fun1:ident $fun2:ident) => {
        /// Enables the `HFCORECLKUSBC` and sets the source by setting the relevant bit in
        /// `CMU_HFCORECLKEN0` and updating the `USBCCLKSEL` field in `CMU_CMD`.
        #[inline]
        pub fn $meth<'new_source, Frequency>(
            self,
            hfcoreclk: &'new_source $typ<Frequency>,
        ) -> HfCoreClkUsbC<'new_source, $typ<Frequency>>
        where
            $typ<Frequency>: Clock,
        {
            let _ = hfcoreclk;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.cmd.write(|w| w.usbcclksel().$fun1());
            cmu.hfcoreclken0.modify(|_, w| w.usbc().set_bit());
            while cmu.status.read().$fun2().bit_is_clear() {}
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source> HfCoreClkUsbC<'source, Source> {
    hfcoreclkusbc_source!(enable_lfxo LfXo lfxo usbclfxosel);
    hfcoreclkusbc_source!(enable_lfrco LfRco lfrco usbclfrcosel);
    hfcoreclkusbc_source!(enable_ushfrco UsHfRco ushfrco usbcushfrcosel);

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.hfcoreclken0.modify(|_, w| w.usbc().clear_bit());
    }

    /// Disables the `HFCORECLKUSBC` by clearing the `USBC` bit in `CMU_HFCORECLKEN0`.
    #[inline]
    pub fn disable(mut self) -> HfCoreClkUsbC<'static, super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}
