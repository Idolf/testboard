use consts;
use device_information;
use efm32hg309f64;
use typenum;

clock_source!(
    /// This type represents ownership over the `USHFRCO`, the High-Frequency RC Oscillator for USB,
    /// which can oscillate at either 24MHz or 48MHz.
    UsHfRco
);

clock_switch_and_divide!(
    /// This type represents ownership over the clock named `USHFRCODIV2` in the reference manual.
    ///
    /// # Quirks
    /// Despite being named `USHFRCODIV2`, the clock can actually represent `USHFRCO` divided by
    /// either 1 or 2 as chosen by a bit in `CMU_USHFRCOCONF`. Since the name in the reference
    /// manual is misleading it has been renamed slightly in this API.
    UsHfRcoDiv
);

macro_rules! ushfrco_frequency {
    ($meth:ident $freq:ident $fun:ident $calib:ident) => {
        /// Enables the `USHFRCO` by setting the `USHFRCOEN` bit in `CMU_OSENCMD` and configures the
        /// frequency by setting the `BAND` subfield in `CMU_USHFRCOCONF`, the `TUNING` subfield in
        /// `CMU_USHFRCOCTRL` and the `FINETUNING` subfield in `CMU_USHFRCOTUNE`.
        ///
        /// This function will block until the `USHFRCO` as ready, by waiting for the `USHFRCORDY`
        /// bit to be set in `CMU_STATUS`.
        #[inline]
        pub fn $meth(self) -> UsHfRco<consts::$freq> {
            let (coarse, fine) = device_information::$calib();

            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };

            cmu.ushfrcoconf.write(|w| w.band().$fun());
            cmu.ushfrcoctrl.write(|w| unsafe { w.tuning().bits(coarse) });
            cmu.ushfrcotune.write(|w| unsafe { w.finetuning().bits(fine) });

            cmu.oscencmd.write(|w| w.ushfrcoen().set_bit());
            while cmu.status.read().ushfrcordy().bit_is_clear() {}

            unsafe { self.transmute_state() }
        }
    };
}

impl<Frequency> UsHfRco<Frequency> {
    ushfrco_frequency!(enable_24mhz Mhz24 _24mhz get_ushfrco_calib_band_24);
    ushfrco_frequency!(enable_48mhz Mhz48 _48mhz get_ushfrco_calib_band_48);

    /// Disables the `USHFRCO` by setting the `USHFRCODIS` bit in `CMU_OSENCMD`.
    #[inline]
    pub fn disable(self) -> UsHfRco<super::Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.ushfrcodis().set_bit());
        unsafe { self.transmute_state() }
    }
}

impl<'source, Source, Division> UsHfRcoDiv<'source, Source, Division> {
    /// Sets the `USHFRCODIV2` to not divide by two by setting the `USHFRCODIV2DIS` bit in
    /// `CMU_USHFRCOCONF`.
    #[inline]
    pub fn enable_div1<'new_source>(
        self,
        ushfrco: &'new_source UsHfRco<consts::Mhz24>,
    ) -> UsHfRcoDiv<'new_source, UsHfRco<consts::Mhz24>, typenum::U1> {
        let _ = ushfrco;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.ushfrcoconf.modify(|_, w| w.ushfrcodiv2dis().set_bit());
        unsafe { self.transmute_state() }
    }

    /// Sets the `USHFRCODIV2` to divide by two by clearing the `USHFRCODIV2DIS` bit in
    /// `CMU_USHFRCOCONF`.
    #[inline]
    pub fn enable_div2<'new_source, Frequency>(
        self,
        ushfrco: &'new_source UsHfRco<Frequency>,
    ) -> UsHfRcoDiv<'new_source, UsHfRco<Frequency>, typenum::U2>
    where
        UsHfRco<Frequency>: super::Clock,
    {
        let _ = ushfrco;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.ushfrcoconf
            .modify(|_, w| w.ushfrcodiv2dis().clear_bit());
        unsafe { self.transmute_state() }
    }

    /// Disables the `USHFRCODIV2`. This does not actually do anything at run-time, but before we
    /// can allow the `USHFRCO` to be reconfigured, we need to call this function to make sure that
    /// there are no users of the clock while it is being reconfigured.
    #[inline]
    pub fn disable(self) -> UsHfRcoDiv<'static, super::Off, super::Off> {
        unsafe { self.transmute_state() }
    }
}
