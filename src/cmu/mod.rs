use core::marker::PhantomData;
use efm32hg309f64;
use frequencies::*;

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

pub struct Cmu(PhantomData<&'static efm32hg309f64::CMU>);

pub struct Off;
pub struct On;

pub trait Clock {
    /// The frequency of the clock.
    const FREQUENCY: f64;

    /// Locks in the state of the clock, so it will not be changed again nor will it be disabled on
    /// drop.
    fn finalize(self) -> &'static Self;
}
