use super::{Clock, HfCoreClk};
use efm32hg309f64;
use typenum;

clock_switch_and_divide!(
    /// This type represents ownership over the `HFCORECLKLE` and the clock named `HFCORECLKLEDIV2`
    /// in the reference manual.
    ///
    /// # Quirks
    /// Despite being named `HFCORECLKLEDIV2`, the clock can actually represent `HFCORECLKLE`
    /// divided by either 2 or 4 as chosen by a bit in `CMU_HFCORECLKDIV`. Since the name in the
    /// reference manual is misleading it has been renamed slightly in this API.
    HfCoreClkLeDiv
);

impl<'source, Source, Division> HfCoreClkLeDiv<'source, Source, Division> {
    /// Enables the `HFCORECLKLE` and sets the divider to 2 by setting the relevant bit in
    /// `CMU_HFCORECLKEN0` and clears the `HFCORECLKLEDIV` bit in `CMU_HFCORECLKDIV`.
    #[inline]
    pub fn enable_div2<'new_source, InnerSource, InnerDivision>(
        self,
        hfcoreclk: &'new_source HfCoreClk<'new_source, InnerSource, InnerDivision>,
    ) -> HfCoreClkLeDiv<'new_source, HfCoreClk<'new_source, InnerSource, InnerDivision>, typenum::U2>
    where
        HfCoreClk<'new_source, InnerSource, InnerDivision>: Clock,
    {
        let _ = hfcoreclk;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.hfcoreclkdiv
            .modify(|_, w| w.hfcoreclklediv().clear_bit());
        cmu.hfcoreclken0.modify(|_, w| w.le().set_bit());
        unsafe { self.transmute_state() }
    }

    /// Enables the `HFCORECLKLE` and sets the divider to 2 by setting the relevant bit in
    /// `CMU_HFCORECLKEN0` and sets the `HFCORECLKLEDIV` bit in `CMU_HFCORECLKDIV`.
    #[inline]
    pub fn enable_div4<'new_source, InnerSource, InnerDivision>(
        self,
        hfcoreclk: &'new_source HfCoreClk<'new_source, InnerSource, InnerDivision>,
    ) -> HfCoreClkLeDiv<'new_source, HfCoreClk<'new_source, InnerSource, InnerDivision>, typenum::U4>
    where
        HfCoreClk<'new_source, InnerSource, InnerDivision>: Clock,
    {
        let _ = hfcoreclk;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.hfcoreclkdiv.modify(|_, w| w.hfcoreclklediv().set_bit());
        cmu.hfcoreclken0.modify(|_, w| w.le().set_bit());
        unsafe { self.transmute_state() }
    }

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.hfcoreclken0.modify(|_, w| w.le().clear_bit());
    }

    /// Disables the `HFCORECLKLE` by clearing the relevant bit in `CMU_HFCORECLKLE0`.
    #[inline]
    pub fn disable(mut self) -> HfCoreClkLeDiv<'static, super::Off, super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}
