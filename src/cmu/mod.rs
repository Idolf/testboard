//! This module contains a nicer API for interacting with the clocks that are part of the CMU. For
//! an overview of these clocks, see section 11.3 in EFM32HF-RM.pdf.
//!
//! With only a few exceptions, the types in this modules corresponds directly to that diagram. The
//! exceptions are:
//!
//! * The clock corresponding to the type `HfCoreClkLeDiv` is not named in the diagram. It is the
//! clock that comes out of the box with the title "/2 or /4" that comes after `HFCORECLKLE`. Other
//! places in the manual refer to this clock as `HFCORECLKLEDIV2`, however this name is misleading
//! as the divisor can either be divided by 2 or 4.
//!
//! * The clock corresponding to the type `UsHfRcoDiv` is similarly not named. It is the clock that
//! comes out of the box with the title "/1 or /2" that flows into the `HFCLK`. In other places in
//! the manual this clock is referred to as `USHFRCODIV2` though this is again misleading as the
//! divisor can be either 1 or 2.
//!
//! * The clock `HFCORECLKLE` has no corresponding type. If a type was created, it's only
//! functionality would be to turn it on or off, and it would always be used together with the
//! `HfCoreClkLeDiv`. Instead the ability to turn on and off the `HFCORECLKLE` has been put inside
//! the `HfCoreClkLeDiv` type.
//!
//! * The clock named `HFCORECLKCM0` is the clock for running the CPU itself. It was not deemed
//! useful to represent it here.
//!
//! * The clocks named name `PCNTnCLK` in the diagram (i.e. only `PCNT0CLK` since there is only one
//! pulse counter) have not been represented yet. Doing so might be useful but has not yet
//! implemented.
//!
//! * The clock named name `WDOGCLK` has not been represented yet. Doing so might be useful but has
//! not yet implemented.

use core::mem;
use efm32hg309f64;
use frequencies;

#[macro_use]
mod macros;
pub mod hfclk;
pub mod hfcoreclk;
pub mod hfcoreclkle;
pub mod hfcoreclkusbc;
pub mod hfperclk;
pub mod hfrco;
pub mod lfa;
pub mod lfb;
pub mod lfc;
pub mod lfrco;
pub mod ushfrco;
pub mod xo;

pub use self::hfclk::HfClk;
pub use self::hfcoreclk::{HfCoreClk, HfCoreClkAes, HfCoreClkDma, HfCoreClkUsb};
pub use self::hfcoreclkle::HfCoreClkLeDiv;
pub use self::hfcoreclkusbc::HfCoreClkUsbC;
pub use self::hfperclk::{
    HfPerClk, HfPerClkAcmp0, HfPerClkAdc0, HfPerClkGpio, HfPerClkI2c0, HfPerClkIdac0, HfPerClkPrs,
    HfPerClkTimer0, HfPerClkTimer1, HfPerClkTimer2, HfPerClkUsart0, HfPerClkUsart1, HfPerClkVcmp,
};
pub use self::hfrco::{AuxHfRco, HfRco};
pub use self::lfa::{LfaClk, LfaClkRtc};
pub use self::lfb::{LfbClk, LfbClkLeuart0};
pub use self::lfc::{LfcClk, LfcClkUsbLe};
pub use self::lfrco::{LfRco, ULfRco};
pub use self::ushfrco::{UsHfRco, UsHfRcoDiv};
pub use self::xo::{HfXo, LfXo};

/// State for clocks that are turned off
pub struct Off;
/// State for clocks that are already running but for which it would be inconvenient to put their
/// actual state in the `InitialCmuState`.
pub struct Uninitialized;

pub trait Clock {
    /// The frequency of the clock.
    const FREQUENCY: f64;
}

pub struct InitialCmuState {
    pub hfclk: HfClk<'static, Uninitialized, Uninitialized>,
    pub hfcoreclk: HfCoreClk<'static, Uninitialized, Uninitialized>,
    pub hfperclk: HfPerClk<'static, Uninitialized, Uninitialized>,

    pub hfcoreclkaes: HfCoreClkAes<'static, Off>,
    pub hfcoreclkdma: HfCoreClkDma<'static, Off>,
    pub hfcoreclkusb: HfCoreClkUsb<'static, Off>,
    pub hfcoreclklediv: HfCoreClkLeDiv<'static, Off, Off>,
    pub hfcoreclkusbc: HfCoreClkUsbC<'static, Off>,

    pub hfperclkacmp0: HfPerClkAcmp0<'static, Off>,
    pub hfperclkadc0: HfPerClkAdc0<'static, Off>,
    pub hfperclkgpio: HfPerClkGpio<'static, Off>,
    pub hfperclki2c0: HfPerClkI2c0<'static, Off>,
    pub hfperclkidac0: HfPerClkIdac0<'static, Off>,
    pub hfperclkprs: HfPerClkPrs<'static, Off>,
    pub hfperclktimer0: HfPerClkTimer0<'static, Off>,
    pub hfperclktimer1: HfPerClkTimer1<'static, Off>,
    pub hfperclktimer2: HfPerClkTimer2<'static, Off>,
    pub hfperclkusart0: HfPerClkUsart0<'static, Off>,
    pub hfperclkusart1: HfPerClkUsart1<'static, Off>,
    pub hfperclkvcmp: HfPerClkVcmp<'static, Off>,

    pub lfaclk: LfaClk<'static, Off>,
    pub lfaclkrtc: LfaClkRtc<'static, Off, Off>,
    pub lfbclk: LfbClk<'static, Off>,
    pub lfbclkleuart0: LfbClkLeuart0<'static, Off, Off>,
    pub lfcclk: LfcClk<'static, Off>,
    pub lfcclkusble: LfcClkUsbLe<'static, Off>,

    pub hfrco: HfRco<frequencies::Mhz14>,
    pub auxhfrco: AuxHfRco<Off>,
    pub lfrco: LfRco<Off>,
    pub ulfrco: ULfRco<frequencies::Hz1000>,
    pub ushfrco: UsHfRco<Off>,
    pub ushfrcodiv: UsHfRcoDiv<'static, Off, Off>,
    pub hfxo: HfXo<Off>,
    pub lfxo: LfXo<Off>,
}

impl InitialCmuState {
    /// Gets the initial cmu state.
    ///
    /// # Safety
    /// This function assumes that the `CMU` given in the argument is in its initial state.
    pub unsafe fn get_initial_state(cmu: efm32hg309f64::CMU) -> InitialCmuState {
        let _ = cmu;
        mem::transmute::<(), InitialCmuState>(())
    }
}
