use typenum::{self, B0, B1, UInt, UTerm};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U7000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U11000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U14000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U21000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U24000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub type U48000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;

pub type Hz1000 = typenum::U1000;
pub type Hz32768 = typenum::U32768;
pub type Mhz1 = typenum::U1000000;
pub type Mhz7 = U7000000;
pub type Mhz11 = U11000000;
pub type Mhz14 = U14000000;
pub type Mhz21 = U21000000;
pub type Mhz24 = U24000000;
pub type Mhz48 = U48000000;
