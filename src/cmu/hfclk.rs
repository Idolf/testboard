use super::{HfRco, HfXo, LfRco, LfXo, UsHfRcoDiv};
use efm32hg309f64;
use typenum;

clock_switch_and_divide!(
    /// This type represents ownership over the `HFCLK`, the primary High-Frequency Clock which most
    /// high-frequency peripherals depend on.
    HfClk
);

macro_rules! hfclk_div {
    ($meth:ident $val:tt $div:ident) => {
        /// Updates the `HfClk` divider by setting the `HFCLKDIV` subfield in `CMU_CTRL`.
        #[inline]
        pub fn $meth(self) -> HfClk<'source, Source, typenum::$div> {
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.ctrl.modify(|_, w| unsafe { w.hfclkdiv().bits($val-1) });
            unsafe { self.transmute_state() }
        }
    }
}

macro_rules! hfclk_source {
    ($meth:ident $typ:ident) => {
        /// Updates the `HfClk` source by setting the `HFCLKSEL` subfield in `CMU_CMD`.
        #[inline]
        pub fn $meth<'new_source, Frequency>(
            self,
            $meth: &'new_source $typ<Frequency>,
        ) -> HfClk<'new_source, $typ<Frequency>, Division>
        where
            $typ<Frequency>: super::Clock,
        {
            let _ = $meth;
            let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
            cmu.cmd.write(|w| w.hfclksel().$meth());
            unsafe { self.transmute_state() }
        }
    }
}

impl<'source, Source, Division> HfClk<'source, Source, Division> {
    hfclk_div!(div1 1 U1);
    hfclk_div!(div2 2 U2);
    hfclk_div!(div3 3 U3);
    hfclk_div!(div4 4 U4);
    hfclk_div!(div5 5 U5);
    hfclk_div!(div6 6 U6);
    hfclk_div!(div7 7 U7);
    hfclk_div!(div8 8 U8);

    hfclk_source!(hfrco HfRco);
    hfclk_source!(hfxo HfXo);
    hfclk_source!(lfrco LfRco);
    hfclk_source!(lfxo LfXo);

    /// Updates the `HfClk` source by setting the `HFCLKSEL` subfield in `CMU_CMD`.
    #[inline]
    pub fn ushfrcodiv<'new_source, InnerSource, InnerDivision>(
        self,
        ushfrcodiv: &'new_source UsHfRcoDiv<'new_source, InnerSource, InnerDivision>,
    ) -> HfClk<'new_source, UsHfRcoDiv<'new_source, InnerSource, InnerDivision>, Division>
    where
        UsHfRcoDiv<'new_source, InnerSource, InnerDivision>: super::Clock,
    {
        let _ = ushfrcodiv;
        let cmu = unsafe { &*efm32hg309f64::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().ushfrcodiv2());
        unsafe { self.transmute_state() }
    }
}
