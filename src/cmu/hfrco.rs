use device_information;
use efm32hg309f64;

clock_source!(
    /// This type represents ownership over the `HFRCO`, the High-Frequency RC Oscillator, which can
    /// oscillate at 1-21MHz.
    HfRco
);

clock_source!(
    /// This type represents ownership over the `AUXHFRCO`, the Auxiliary High-Frequency RC
    /// Oscillator, which can oscillate at 1-21MHz.
    AuxHfRco
);

macro_rules! hfrco_frequency {
    ($meth:ident $freq:ident $fun:ident $calib:ident) => {
        /// Enables the `HFRCO` by setting the `HFRCOEN` bit in `CMU_OSENCMD` and sets the frequency
        /// by setting the `BAND` and `TUNING` fields in `CMU_HFRCOCTRL`.
        ///
        /// This function will block until the `HFRCO` as ready, by waiting for the `HFRCORDY` bit
        /// to be set in `CMU_STATUS`.
        #[inline]
        pub fn $meth(self) -> HfRco<super::$freq> {
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.hfrcoctrl.write(|w| {
                unsafe {
                    w.band().$fun().tuning().bits(device_information::$calib())
                }
            });

            cmu.oscencmd.write(|w| w.hfrcoen().set_bit());
            while cmu.status.read().hfrcordy().bit_is_clear() {}

            unsafe { self.transmute_state() }
        }
    };
}

impl<Frequency> HfRco<Frequency> {
    hfrco_frequency!(enable_1mhz Mhz1 _1mhz get_hfrco_calib_band_1);
    hfrco_frequency!(enable_7mhz Mhz7 _7mhz get_hfrco_calib_band_7);
    hfrco_frequency!(enable_11mhz Mhz11 _11mhz get_hfrco_calib_band_11);
    hfrco_frequency!(enable_14mhz Mhz14 _14mhz get_hfrco_calib_band_14);
    hfrco_frequency!(enable_21mhz Mhz21 _21mhz get_hfrco_calib_band_21);

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfrcodis().set_bit());
    }

    /// Disables the `HFRCO` by setting the `HFRCODIS` bit in `CMU_OSCENCMD`.
    #[inline]
    pub fn disable(mut self) -> HfRco<super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}

macro_rules! auxhfrco_frequency {
    ($meth:ident $freq:ident $fun:ident $calib:ident) => {
        /// Enables the `AUXHFRCO` by setting the `AUXHFRCOEN` bit in `CMU_OSENCMD` and sets the
        /// frequency by setting the `BAND` and `TUNING` fields in `CMU_AUXHFRCOCTRL`.
        ///
        /// This function will block until the `AUXHFRCO` as ready, by waiting for the `AUXHFRCORDY`
        /// bit to be set in `CMU_STATUS`.
        #[inline]
        pub fn $meth(self) -> AuxHfRco<super::$freq> {
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.auxhfrcoctrl.write(|w| {
                unsafe {
                    w.band().$fun().tuning().bits(device_information::$calib())
                }
            });

            cmu.oscencmd.write(|w| w.auxhfrcoen().set_bit());
            while cmu.status.read().auxhfrcordy().bit_is_clear() {}

            unsafe { self.transmute_state() }
        }
    };
}

impl<Frequency> AuxHfRco<Frequency> {
    auxhfrco_frequency!(enable_1mhz Mhz1 _1mhz get_auxhfrco_calib_band_1);
    auxhfrco_frequency!(enable_7mhz Mhz7 _7mhz get_auxhfrco_calib_band_7);
    auxhfrco_frequency!(enable_11mhz Mhz11 _11mhz get_auxhfrco_calib_band_11);
    auxhfrco_frequency!(enable_14mhz Mhz14 _14mhz get_auxhfrco_calib_band_14);
    auxhfrco_frequency!(enable_21mhz Mhz21 _21mhz get_auxhfrco_calib_band_21);

    #[inline]
    fn _disable(&mut self) {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.oscencmd.write(|w| w.auxhfrcodis().set_bit());
    }

    /// Disables the `AUXHFRCO` by setting the `AUXHFRCODIS` bit in `CMU_OSCENCMD`.
    #[inline]
    pub fn disable(mut self) -> AuxHfRco<super::Off> {
        self._disable();
        unsafe { self.transmute_state() }
    }
}
