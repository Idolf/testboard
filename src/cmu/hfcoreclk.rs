use super::{Clock, HfClk};
use efm32hg309f64;
use typenum;

clock_switch_and_divide!(
    /// This type represents ownership over the `HFCORECLK`, the High-Frequency Core Clock which is
    /// used to drive the CPU and for peripherals tightly coupled to the CPU.
    ///
    /// # Drop semantics
    /// This clock cannot be turned off in the hardware, and doing so would in any case be
    /// inadvisable since the CPU is tied to it.
    ///
    /// This means that unlike most other clocks, dropping the value will not turn off the clock.
    HfCoreClk
);

clock_switch!(
    /// This type represents ownership over the `HFCORECLKAES`, the High-Frequency Clock used for
    /// the AES peripheral.
    HfCoreClkAes
);

clock_switch!(
    /// This type represents ownership over the `HFCORECLKDMA`, the High-Frequency Clock used for
    /// the DMA peripheral.
    HfCoreClkDma
);

clock_switch!(
    /// This type represents ownership over the `HFCORECLKUSB`, the High-Frequency Clock used for
    /// the USB peripheral.
    HfCoreClkUsb
);

macro_rules! hfcoreclk_div {
    ($meth:ident $fun:ident $div:ident) => {
        /// Sets the `HfCoreClk` divider by updating the `HFCORECLKDIV` subfield in
        /// `CMU_HFCORECLKDIV`.
        #[inline]
        pub fn $meth<'new_source, InnerSource, InnerDivision>(
            self,
            hfclk: &'new_source HfClk<'new_source, InnerSource, InnerDivision>,
        ) -> HfCoreClk<'new_source, HfClk<'new_source, InnerSource, InnerDivision>, typenum::$div>
        where HfClk<'new_source, InnerSource, InnerDivision>: Clock {
            let _ = hfclk;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.hfcoreclkdiv.write(|w| w.hfcoreclkdiv().$fun());

            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source, Division> HfCoreClk<'source, Source, Division> {
    hfcoreclk_div!(div1 hfclk U1);
    hfcoreclk_div!(div2 hfclk2 U2);
    hfcoreclk_div!(div4 hfclk4 U4);
    hfcoreclk_div!(div8 hfclk8 U8);
    hfcoreclk_div!(div16 hfclk16 U16);
    hfcoreclk_div!(div32 hfclk32 U32);
    hfcoreclk_div!(div64 hfclk64 U64);
    hfcoreclk_div!(div128 hfclk128 U128);
    hfcoreclk_div!(div256 hfclk256 U256);
    hfcoreclk_div!(div512 hfclk512 U512);

    #[inline]
    fn _disable(&mut self) {}
}

macro_rules! hfcoreclk_subclock {
    ($typ:ident $fun:ident) => {
        impl<'source, Source> $typ<'source, Source> {
            /// Enables the clock by setting the relevant bit in `CMU_HFCORECLKEN0`.
            #[inline]
            pub fn enable<'new_source, InnerSource, InnerDivision>(
                self,
                hfcoreclk: &'new_source HfCoreClk<'new_source, InnerSource, InnerDivision>,
            ) -> $typ<'new_source, HfCoreClk<'new_source, InnerSource, InnerDivision>>
            where
                HfCoreClk<'new_source, InnerSource, InnerDivision>: Clock,
            {
                let _ = hfcoreclk;
                let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
                cmu.hfcoreclken0.modify(|_, w| w.$fun().set_bit());
                unsafe { self.transmute_state() }
            }

            #[inline]
            fn _disable(&mut self) {
                let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
                cmu.hfcoreclken0.modify(|_, w| w.$fun().clear_bit());
            }

            /// Disables the clock by clearing the relevant bit in `CMU_HFCORECLKEN0`.
            #[inline]
            pub fn disable(mut self) -> $typ<'static, super::Off> {
                self._disable();
                unsafe { self.transmute_state() }
            }
        }
    };
}

hfcoreclk_subclock!(HfCoreClkAes aes);
hfcoreclk_subclock!(HfCoreClkDma dma);
hfcoreclk_subclock!(HfCoreClkUsb usb);
