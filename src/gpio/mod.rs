#![allow(unused)]

use core::marker::PhantomData;
use core::mem;
use devices;
use efm32hg309f64;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

pub mod pin_modes;
use self::pin_modes::{PinMode, ValidPinMode};

macro_rules! make_pin {
    (
        $typ:ident,
        $modereg:ident,
        $modefun:ident,
        $outreg:ident,
        $setreg:ident,
        $clrreg:ident,
        $tglreg:ident,
        $inreg:ident,
        $bit:expr
    ) => {
        pub struct $typ<Mode> {
            non_send: PhantomData<*mut ()>,
            mode: PhantomData<Mode>,
        }
        unsafe impl<Mode> Sync for $typ<Mode> {}

        impl<Mode> $typ<Mode> {
            #[allow(unused)]
            #[inline]
            unsafe fn transmute_mode<NewMode>(self) -> $typ<NewMode> {
                mem::forget(self);
                $typ {
                    non_send: PhantomData,
                    mode: PhantomData,
                }
            }

            #[inline]
            pub fn mode<M: ValidPinMode>(self, pin_mode: M) -> $typ<M> {
                let _ = pin_mode;
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                gpio.$modereg.modify(|_, w| w.$modefun().bits(M::MODE));
                match M::DOUT {
                    Some(true) => gpio
                        .$setreg
                        .write(|w| unsafe { w.doutset().bits(1 << $bit) }),
                    Some(false) => gpio
                        .$clrreg
                        .write(|w| unsafe { w.doutclr().bits(1 << $bit) }),
                    None => (),
                }
                unsafe { self.transmute_mode() }
            }

            #[inline]
            pub unsafe fn claim_ownership() -> Self {
                $typ {
                    non_send: PhantomData,
                    mode: PhantomData,
                }
            }
        }

        impl<Mode> devices::Device for $typ<Mode> {}
        impl<Mode: 'static> devices::StaticDevice for $typ<Mode> {
            #[inline]
            fn finalize(self) -> &'static mut Self {
                assert_eq_size!(Self, ());
                unsafe { ::devices::make_static_mut(self) }
            }
        }

        impl<OutputMode> InputPin for $typ<PinMode<pin_modes::input_modes::Enabled, OutputMode>> {
            #[inline]
            fn is_high(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$inreg.read().din().bits() & (1 << $bit)) != 0
            }

            #[inline]
            fn is_low(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$inreg.read().din().bits() & (1 << $bit)) == 0
            }
        }

        impl<OutputMode> InputPin for $typ<PinMode<pin_modes::input_modes::Filtered, OutputMode>> {
            #[inline]
            fn is_high(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$inreg.read().din().bits() & (1 << $bit)) != 0
            }

            #[inline]
            fn is_low(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$inreg.read().din().bits() & (1 << $bit)) == 0
            }
        }

        impl<InputMode, StateHigh, StateLow> OutputPin
            for $typ<PinMode<InputMode, pin_modes::output_modes::MultiState<StateHigh, StateLow>>>
        {
            #[inline]
            fn set_low(&mut self) {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                gpio.$clrreg
                    .write(|w| unsafe { w.doutclr().bits(1 << $bit) })
            }

            #[inline]
            fn set_high(&mut self) {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                gpio.$setreg
                    .write(|w| unsafe { w.doutset().bits(1 << $bit) })
            }
        }

        impl<InputMode, StateHigh, StateLow> ToggleableOutputPin
            for $typ<PinMode<InputMode, pin_modes::output_modes::MultiState<StateHigh, StateLow>>>
        {
            #[inline]
            fn toggle(&mut self) {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                gpio.$tglreg
                    .write(|w| unsafe { w.douttgl().bits(1 << $bit) })
            }
        }

        impl<InputMode, StateHigh, StateLow> StatefulOutputPin
            for $typ<PinMode<InputMode, pin_modes::output_modes::MultiState<StateHigh, StateLow>>>
        {
            #[inline]
            fn is_set_high(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$outreg.read().dout().bits() & (1 << $bit)) != 0
            }

            #[inline]
            fn is_set_low(&self) -> bool {
                let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
                (gpio.$outreg.read().dout().bits() & (1 << $bit)) == 0
            }
        }
    };
}

make_pin!(Pa0, pa_model, mode0, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 0);
make_pin!(Pa1, pa_model, mode1, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 1);
make_pin!(Pa2, pa_model, mode2, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 2);
make_pin!(Pa3, pa_model, mode3, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 3);
make_pin!(Pa4, pa_model, mode4, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 4);
make_pin!(Pa5, pa_model, mode5, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 5);
make_pin!(Pa6, pa_model, mode6, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 6);
make_pin!(Pa7, pa_model, mode7, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 7);

make_pin!(Pa8, pa_modeh, mode8, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 8);
make_pin!(Pa9, pa_modeh, mode9, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 9);
make_pin!(Pa10, pa_modeh, mode10, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 10);
make_pin!(Pa11, pa_modeh, mode11, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 11);
make_pin!(Pa12, pa_modeh, mode12, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 12);
make_pin!(Pa13, pa_modeh, mode13, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 13);
make_pin!(Pa14, pa_modeh, mode14, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 14);
make_pin!(Pa15, pa_modeh, mode15, pa_dout, pa_doutset, pa_doutclr, pa_douttgl, pa_din, 15);

make_pin!(Pb0, pb_model, mode0, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 0);
make_pin!(Pb1, pb_model, mode1, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 1);
make_pin!(Pb2, pb_model, mode2, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 2);
make_pin!(Pb3, pb_model, mode3, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 3);
make_pin!(Pb4, pb_model, mode4, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 4);
make_pin!(Pb5, pb_model, mode5, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 5);
make_pin!(Pb6, pb_model, mode6, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 6);
make_pin!(Pb7, pb_model, mode7, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 7);

make_pin!(Pb8, pb_modeh, mode8, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 8);
make_pin!(Pb9, pb_modeh, mode9, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 9);
make_pin!(Pb10, pb_modeh, mode10, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 10);
make_pin!(Pb11, pb_modeh, mode11, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 11);
make_pin!(Pb12, pb_modeh, mode12, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 12);
make_pin!(Pb13, pb_modeh, mode13, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 13);
make_pin!(Pb14, pb_modeh, mode14, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 14);
make_pin!(Pb15, pb_modeh, mode15, pb_dout, pb_doutset, pb_doutclr, pb_douttgl, pb_din, 15);

make_pin!(Pc0, pc_model, mode0, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 0);
make_pin!(Pc1, pc_model, mode1, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 1);
make_pin!(Pc2, pc_model, mode2, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 2);
make_pin!(Pc3, pc_model, mode3, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 3);
make_pin!(Pc4, pc_model, mode4, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 4);
make_pin!(Pc5, pc_model, mode5, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 5);
make_pin!(Pc6, pc_model, mode6, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 6);
make_pin!(Pc7, pc_model, mode7, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 7);

make_pin!(Pc8, pc_modeh, mode8, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 8);
make_pin!(Pc9, pc_modeh, mode9, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 9);
make_pin!(Pc10, pc_modeh, mode10, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 10);
make_pin!(Pc11, pc_modeh, mode11, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 11);
make_pin!(Pc12, pc_modeh, mode12, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 12);
make_pin!(Pc13, pc_modeh, mode13, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 13);
make_pin!(Pc14, pc_modeh, mode14, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 14);
make_pin!(Pc15, pc_modeh, mode15, pc_dout, pc_doutset, pc_doutclr, pc_douttgl, pc_din, 15);

make_pin!(Pd0, pd_model, mode0, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 0);
make_pin!(Pd1, pd_model, mode1, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 1);
make_pin!(Pd2, pd_model, mode2, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 2);
make_pin!(Pd3, pd_model, mode3, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 3);
make_pin!(Pd4, pd_model, mode4, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 4);
make_pin!(Pd5, pd_model, mode5, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 5);
make_pin!(Pd6, pd_model, mode6, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 6);
make_pin!(Pd7, pd_model, mode7, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 7);

make_pin!(Pd8, pd_modeh, mode8, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 8);
make_pin!(Pd9, pd_modeh, mode9, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 9);
make_pin!(Pd10, pd_modeh, mode10, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 10);
make_pin!(Pd11, pd_modeh, mode11, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 11);
make_pin!(Pd12, pd_modeh, mode12, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 12);
make_pin!(Pd13, pd_modeh, mode13, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 13);
make_pin!(Pd14, pd_modeh, mode14, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 14);
make_pin!(Pd15, pd_modeh, mode15, pd_dout, pd_doutset, pd_doutclr, pd_douttgl, pd_din, 15);

make_pin!(Pe0, pe_model, mode0, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 0);
make_pin!(Pe1, pe_model, mode1, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 1);
make_pin!(Pe2, pe_model, mode2, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 2);
make_pin!(Pe3, pe_model, mode3, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 3);
make_pin!(Pe4, pe_model, mode4, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 4);
make_pin!(Pe5, pe_model, mode5, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 5);
make_pin!(Pe6, pe_model, mode6, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 6);
make_pin!(Pe7, pe_model, mode7, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 7);

make_pin!(Pe8, pe_modeh, mode8, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 8);
make_pin!(Pe9, pe_modeh, mode9, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 9);
make_pin!(Pe10, pe_modeh, mode10, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 10);
make_pin!(Pe11, pe_modeh, mode11, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 11);
make_pin!(Pe12, pe_modeh, mode12, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 12);
make_pin!(Pe13, pe_modeh, mode13, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 13);
make_pin!(Pe14, pe_modeh, mode14, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 14);
make_pin!(Pe15, pe_modeh, mode15, pe_dout, pe_doutset, pe_doutclr, pe_douttgl, pe_din, 15);

make_pin!(Pf0, pf_model, mode0, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 0);
make_pin!(Pf1, pf_model, mode1, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 1);
make_pin!(Pf2, pf_model, mode2, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 2);
make_pin!(Pf3, pf_model, mode3, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 3);
make_pin!(Pf4, pf_model, mode4, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 4);
make_pin!(Pf5, pf_model, mode5, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 5);
make_pin!(Pf6, pf_model, mode6, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 6);
make_pin!(Pf7, pf_model, mode7, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 7);

make_pin!(Pf8, pf_modeh, mode8, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 8);
make_pin!(Pf9, pf_modeh, mode9, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 9);
make_pin!(Pf10, pf_modeh, mode10, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 10);
make_pin!(Pf11, pf_modeh, mode11, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 11);
make_pin!(Pf12, pf_modeh, mode12, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 12);
make_pin!(Pf13, pf_modeh, mode13, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 13);
make_pin!(Pf14, pf_modeh, mode14, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 14);
make_pin!(Pf15, pf_modeh, mode15, pf_dout, pf_doutset, pf_doutclr, pf_douttgl, pf_din, 15);

pub struct InitialGpioState {
    pub pa0: Pa0<pin_modes::UnknownMode>,
    pub pb7: Pb7<pin_modes::UnknownMode>,
    pub pb8: Pb8<pin_modes::UnknownMode>,
    pub pb11: Pb11<pin_modes::UnknownMode>,
    pub pb13: Pb13<pin_modes::UnknownMode>,
    pub pb14: Pb14<pin_modes::UnknownMode>,
    pub pc0: Pc0<pin_modes::UnknownMode>,
    pub pc1: Pc1<pin_modes::UnknownMode>,
    pub pe12: Pe12<pin_modes::UnknownMode>,
    pub pe13: Pe13<pin_modes::UnknownMode>,
    pub pf0: Pf0<pin_modes::UnknownMode>,
    pub pf1: Pf1<pin_modes::UnknownMode>,
    pub pf2: Pf2<pin_modes::UnknownMode>,
}

impl InitialGpioState {
    /// Gets the initial gpio state.
    ///
    /// # Safety
    /// This function assumes that the `GPIO` given in the argument is in its initial state and that
    /// the function is only called once.
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    pub unsafe fn get_initial_state(gpio: efm32hg309f64::GPIO) -> InitialGpioState {
        let _ = gpio;
        InitialGpioState {
            pa0: Pa0::claim_ownership(),
            pb7: Pb7::claim_ownership(),
            pb8: Pb8::claim_ownership(),
            pb11: Pb11::claim_ownership(),
            pb13: Pb13::claim_ownership(),
            pb14: Pb14::claim_ownership(),
            pc0: Pc0::claim_ownership(),
            pc1: Pc1::claim_ownership(),
            pe12: Pe12::claim_ownership(),
            pe13: Pe13::claim_ownership(),
            pf0: Pf0::claim_ownership(),
            pf1: Pf1::claim_ownership(),
            pf2: Pf2::claim_ownership(),
        }
    }
}
