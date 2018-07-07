use super::{Clock, HfCoreClkLeDiv, LfRco, LfXo, Off, ULfRco};
use efm32hg309f64;
use typenum;

clock_switch!(
    /// This type represents ownership over the `LFBCLK`, the Low-Frequency Clock B. The `LFBCLK`
    /// can be used to drive the Low-Energy UART (`LEUART0`).
    LfbClk
);
clock_switch_and_divide!(
    /// This type represents ownership over the `LFBCLKLEUART0`, the clock used to drive the
    /// Low-Energy UART (`LEUART0`).
    LfbClkLeuart0
);

macro_rules! lfb_source {
    ($cmu:ident $meth:ident $name:ident $typ:ident { $($code:tt)* }) => {
        /// Enables the `LFBCLK` and sets its source by setting the `LFB` and `LFBE` subfields in
        /// `CMU_LFCLKSEL`.
        #[inline]
        pub fn $meth<'new_source, Frequency>(
            self,
            $name: &'new_source $typ<Frequency>,
        ) -> LfbClk<'new_source, $typ<Frequency>>
        where
            $typ<Frequency>: Clock
        {
            let _ = $name;
            let $cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            $($code)*;
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source> LfbClk<'source, Source> {
    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.lfclksel
            .modify(|_, w| w.lfbe().clear_bit().lfb().disabled());
    }

    /// Disables the `LFBCLK` by clearing the `LFB` and `LFBE` subfields in `CMU_LFCLKSEL`.
    #[inline]
    pub fn disable(mut self) -> LfbClk<'static, Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }

    lfb_source!(cmu enable_lfrco lfrco LfRco {
        cmu.lfclksel
            .modify(|_, w| w.lfbe().clear_bit().lfb().lfrco());
    });

    lfb_source!(cmu enable_lfxo lfxo LfXo {
        cmu.lfclksel
            .modify(|_, w| w.lfbe().clear_bit().lfb().lfxo());
    });

    lfb_source!(cmu enable_ulfrco ulfrco ULfRco {
        cmu.lfclksel
            .modify(|_, w| w.lfbe().set_bit().lfb().disabled());
    });

    /// Enables the LFB and sets its source by setting the `LFB` and `LFBE` subfields in
    /// `CMU_LFCLKSEL`.
    #[inline]
    pub fn enable_hfcoreclklediv<'new_source, InnerSource, InnerDivision>(
        self,
        hfcoreclklediv: &'new_source HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>,
    ) -> LfbClk<'new_source, HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>>
    where
        HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>: Clock,
    {
        let _ = hfcoreclklediv;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.lfclksel
            .modify(|_, w| w.lfbe().clear_bit().lfb().hfcoreclklediv2());
        unsafe { self.transmute_state() }
    }
}

macro_rules! lfbclk_div {
    ($meth:ident $fun:ident $div:ident) => {
        /// Enables the `LFBCLKLEUART0` and updates its divider by setting the `LEUART0` bit in
        /// `CMU_LFBCLKEN0` and setting the `LEUART0` subfield in `CMU_LFBPRESC0`.
        ///
        /// This function will not write to `CMU_LFBPRESC0` or `CMU_LFBCLKEN0` until the relevant
        /// bits in `CMU_SYNCBUSY` are clear.
        #[inline]
        pub fn $meth<'new_source, InnerSource>(
            self,
            lfbclk: &'new_source LfbClk<'new_source, InnerSource>,
        ) -> LfbClkLeuart0<'new_source, LfbClk<'new_source, InnerSource>, typenum::$div>
        where
            LfbClk<'new_source, InnerSource>: Clock,
        {
            let _ = lfbclk;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

            while cmu.syncbusy.read().lfbpresc0().bit_is_set() {}
            cmu.lfbpresc0.write(|w| w.leuart0().$fun());
            while cmu.syncbusy.read().lfbclken0().bit_is_set() {}
            cmu.lfbclken0.write(|w| w.leuart0().set_bit());
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source, Division> LfbClkLeuart0<'source, Source, Division> {
    lfbclk_div!(enable_div1 div1 U1);
    lfbclk_div!(enable_div2 div2 U2);
    lfbclk_div!(enable_div4 div4 U4);
    lfbclk_div!(enable_div8 div8 U8);

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

        while cmu.syncbusy.read().lfbclken0().bit_is_set() {}
        cmu.lfbclken0.write(|w| w.leuart0().clear_bit());
    }

    /// Disables the `LFBCLKLEUART0` by clearing the `RTC` bit in `CMU_LFBCLKEN0`.
    ///
    /// This function will not write to `CMU_LFBCLKEN0` until the relevant bit in `CMU_SYNCBUSY`
    /// is clear.
    #[inline]
    pub fn disable(mut self) -> LfbClkLeuart0<'static, super::Off, super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}
