// See section 11.3 in the EFM32HF-RM for an overview of these clocks

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

    /// Locks in the state of the clock, so it will not be changed again nor will it be disabled on
    /// drop.
    fn finalize(self) -> &'static Self;
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
