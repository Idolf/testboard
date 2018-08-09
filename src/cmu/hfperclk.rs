use super::{Clock, HfClk};
use efm32hg309f64;
use typenum;

macro_rules! hfperclk_div {
    ($meth:ident $fun:ident $div:ident) => {
        /// Enables the `HFPERCLK` and sets its divider by updating the `HFPERCLKEN` and
        /// `HFPERCLKDIV` subfields in `CMU_HFPERCLKDIV`.
        #[inline]
        pub fn $meth<'new_source, InnerSource, InnerDivision>(
            self,
            hfclk: &'new_source HfClk<'new_source, InnerSource, InnerDivision>,
        ) -> HfPerClk<'new_source, HfClk<'new_source, InnerSource, InnerDivision>, typenum::$div>
        where
            HfClk<'new_source, InnerSource, InnerDivision>: Clock
        {
            let _ = hfclk;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.hfperclkdiv.write(|w| w.hfperclken().set_bit().hfperclkdiv().$fun());

            unsafe { self.transmute_state() }
        }
    }
}

macro_rules! hfperclk_subclock {
    ($typ:ident $fun:ident) => {
        impl<'source, Source> $typ<'source, Source> {
            /// Enables the clock by setting the relevant bit in `CMU_HFPERCLKEN0`.
            #[inline]
            pub fn enable<'new_source, InnerSource, InnerDivision>(
                self,
                hfperclk: &'new_source HfPerClk<'new_source, InnerSource, InnerDivision>,
            ) -> $typ<'new_source, HfPerClk<'new_source, InnerSource, InnerDivision>>
            where
                HfPerClk<'new_source, InnerSource, InnerDivision>: Clock,
            {
                let _ = hfperclk;
                let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
                cmu.hfperclken0.modify(|_, w| w.$fun().set_bit());
                unsafe { self.transmute_state() }
            }

            /// Disables the clock by clearing the relevant bit in `CMU_HFPERCLKEN0`.
            #[inline]
            pub fn disable(self) -> $typ<'static, super::Off> {
                let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
                cmu.hfperclken0.modify(|_, w| w.$fun().clear_bit());
                unsafe { self.transmute_state() }
            }
        }
    };
}

impl<'source, Source, Division> HfPerClk<'source, Source, Division> {
    hfperclk_div!(enable_div1 hfclk U1);
    hfperclk_div!(enable_div2 hfclk2 U2);
    hfperclk_div!(enable_div4 hfclk4 U4);
    hfperclk_div!(enable_div8 hfclk8 U8);
    hfperclk_div!(enable_div16 hfclk16 U16);
    hfperclk_div!(enable_div32 hfclk32 U32);
    hfperclk_div!(enable_div64 hfclk64 U64);
    hfperclk_div!(enable_div128 hfclk128 U128);
    hfperclk_div!(enable_div256 hfclk256 U256);
    hfperclk_div!(enable_div512 hfclk512 U512);

    /// Disables the `HFPERCLK` by clearing the relevant bit `HFPERCLKEN` bit in `CMU_HFPERCLKDIV`.
    #[inline]
    pub fn disable(self) -> HfPerClk<'static, super::Off, super::Off> {
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.hfperclkdiv.write(|w| w.hfperclken().clear_bit());
        unsafe { self.transmute_state() }
    }
}

clock_switch_and_divide!(
    /// This type represents ownership over the `HFPERCLK`, the High-Frequency Peripheral Clock.
    HfPerClk
);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKTIMER0`, the clock used for `TIMER0`.
    HfPerClkTimer0
);
hfperclk_subclock!(HfPerClkTimer0 timer0);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKTIMER1`, the clock used for `TIMER1`.
    HfPerClkTimer1
);
hfperclk_subclock!(HfPerClkTimer1 timer1);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKTIMER2`, the clock used for `TIMER2`.
    HfPerClkTimer2
);
hfperclk_subclock!(HfPerClkTimer2 timer2);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKUSART0`, the clock used for `USART0`.
    HfPerClkUsart0
);
hfperclk_subclock!(HfPerClkUsart0 usart0);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKUSART1`, the clock used for `USART1`.
    HfPerClkUsart1
);
hfperclk_subclock!(HfPerClkUsart1 usart1);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKACMP0`, the clock used for the Analog
    /// Comparator.
    HfPerClkAcmp0
);
hfperclk_subclock!(HfPerClkAcmp0 acmp0);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKPRS`, the clock used for the Peripheral
    /// Reflex System.
    HfPerClkPrs
);
hfperclk_subclock!(HfPerClkPrs prs);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKIDAC0`, the clock used for the Current
    /// Digital to Analog Converter.
    HfPerClkIdac0
);
hfperclk_subclock!(HfPerClkIdac0 idac0);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKGPIO`, the clock used for the General
    /// Purpose Input Output system.
    HfPerClkGpio
);
hfperclk_subclock!(HfPerClkGpio gpio);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKVCMP`, the clock used for the Voltage
    /// Comparator.
    HfPerClkVcmp
);
hfperclk_subclock!(HfPerClkVcmp vcmp);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKADC0`, the clock used for the Analog to
    /// Digital Converter.
    HfPerClkAdc0
);
hfperclk_subclock!(HfPerClkAdc0 adc0);

clock_switch!(
    /// This type represents ownership over the `HFPERCLKI2C0`, the clock used for the I²C system.
    HfPerClkI2c0
);
hfperclk_subclock!(HfPerClkI2c0 i2c0);
