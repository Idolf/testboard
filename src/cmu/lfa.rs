use super::{Clock, HfCoreClkLeDiv, LfRco, LfXo, Off, ULfRco};
use efm32hg309f64;
use typenum;

clock_switch!(
    /// This type represents ownership over the `LFACLK`, the Low-Frequency Clock A. The `LFACLK`
    /// can be used to drive the Real-Time Clock (`RTC`) and Pulse Counter (`PCNT0`) systems.
    LfaClk
);

clock_switch_and_divide!(
    /// This type represents ownership over the `LFACLKRTC`, the clock used to drive the Real-Time
    /// Clock (`RTC`) system.
    LfaClkRtc
);

macro_rules! lfa_source {
    ($cmu:ident $meth:ident $name:ident $typ:ident { $($code:tt)* }) => {
        /// Enables the `LFACLK` and sets its source by setting the `LFA` and `LFAE` subfields in
        /// `CMU_LFCLKSEL`.
        #[inline]
        pub fn $meth<'new_source, Frequency>(
            self,
            $name: &'new_source $typ<Frequency>,
        ) -> LfaClk<'new_source, $typ<Frequency>>
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

impl<'source, Source> LfaClk<'source, Source> {
    /// Disables the `LFACLK` by clearing the `LFA` and `LFAE` subfields in `CMU_LFCLKSEL`.
    #[inline]
    pub fn disable(self) -> LfaClk<'static, Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.lfclksel
            .modify(|_, w| w.lfae().clear_bit().lfa().disabled());
        unsafe { self.transmute_state() }
    }

    lfa_source!(cmu enable_lfrco lfrco LfRco {
        cmu.lfclksel
            .modify(|_, w| w.lfae().clear_bit().lfa().lfrco());
    });

    lfa_source!(cmu enable_lfxo lfxo LfXo {
        cmu.lfclksel
            .modify(|_, w| w.lfae().clear_bit().lfa().lfxo());
    });

    lfa_source!(cmu enable_ulfrco ulfrco ULfRco {
        cmu.lfclksel
            .modify(|_, w| w.lfae().set_bit().lfa().disabled());
    });

    /// Enables the `LFACLK` and sets its source by setting the `LFA` and `LFAE` subfields in
    /// `CMU_LFCLKSEL`.
    #[inline]
    pub fn enable_hfcoreclklediv<'new_source, InnerSource, InnerDivision>(
        self,
        hfcoreclklediv: &'new_source HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>,
    ) -> LfaClk<'new_source, HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>>
    where
        HfCoreClkLeDiv<'new_source, InnerSource, InnerDivision>: Clock,
    {
        let _ = hfcoreclklediv;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.lfclksel
            .modify(|_, w| w.lfae().clear_bit().lfa().hfcoreclklediv2());
        unsafe { self.transmute_state() }
    }
}

macro_rules! lfaclk_div {
    ($meth:ident $fun:ident $div:ident) => {
        /// Enables the `LFACLKRTC` and updates its divider by setting the `RTC` bit in
        /// `CMU_LFACLKEN0` and setting the `RTC` subfield in `CMU_LFAPRESC0`.
        ///
        /// This function will not write to `CMU_LFAPRESC0` or `CMU_LFACLKEN0` until the relevant
        /// bits in `CMU_SYNCBUSY` are clear.
        #[inline]
        pub fn $meth<'new_source, InnerSource>(
            self,
            lfaclk: &'new_source LfaClk<'new_source, InnerSource>,
        ) -> LfaClkRtc<'new_source, LfaClk<'new_source, InnerSource>, typenum::$div>
        where
            LfaClk<'new_source, InnerSource>: Clock,
        {
            let _ = lfaclk;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

            while cmu.syncbusy.read().lfapresc0().bit_is_set() {}
            cmu.lfapresc0.write(|w| w.rtc().$fun());
            while cmu.syncbusy.read().lfaclken0().bit_is_set() {}
            cmu.lfaclken0.write(|w| w.rtc().set_bit());
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source, Division> LfaClkRtc<'source, Source, Division> {
    lfaclk_div!(enable_div1 div1 U1);
    lfaclk_div!(enable_div2 div2 U2);
    lfaclk_div!(enable_div4 div4 U4);
    lfaclk_div!(enable_div8 div8 U8);
    lfaclk_div!(enable_div16 div16 U16);
    lfaclk_div!(enable_div32 div32 U32);
    lfaclk_div!(enable_div64 div64 U64);
    lfaclk_div!(enable_div128 div128 U128);
    lfaclk_div!(enable_div256 div256 U256);
    lfaclk_div!(enable_div512 div512 U512);
    lfaclk_div!(enable_div1024 div1024 U1024);
    lfaclk_div!(enable_div2048 div2048 U2048);
    lfaclk_div!(enable_div4096 div4096 U4096);
    lfaclk_div!(enable_div8192 div8192 U8192);
    lfaclk_div!(enable_div16384 div16384 U16384);
    lfaclk_div!(enable_div32768 div32768 U32768);

    /// Disables the `LFACLKRTC` by clearing the `RTC` bit in `CMU_LFACLKEN0`.
    ///
    /// This function will not write to `CMU_LFACLKEN0` until the relevant bit in `CMU_SYNCBUSY`
    /// is clear.
    #[inline]
    pub fn disable(self) -> LfaClkRtc<'static, super::Off, super::Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

        while cmu.syncbusy.read().lfaclken0().bit_is_set() {}
        cmu.lfaclken0.write(|w| w.rtc().clear_bit());

        unsafe { self.transmute_state() }
    }
}
