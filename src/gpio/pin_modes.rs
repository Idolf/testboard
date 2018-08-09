//! This module encodes table 26.1 in EFM32-RM.pdf.

use core::marker::PhantomData;

pub mod input_modes {
    pub struct Disabled;
    pub struct Enabled;
    pub struct Filtered;
}

pub mod output_states {
    pub struct LowAlternateDrive;
    pub struct Low;
    pub struct PullDown;
    pub struct Floating;
    pub struct PullUp;
    pub struct High;
    pub struct HighAlternateDrive;
}

pub mod output_modes {
    use super::output_states::*;
    use core::marker::PhantomData;

    pub struct SingleState<State> {
        state: PhantomData<State>,
    }

    pub struct MultiState<StateHigh, StateLow> {
        state_high: PhantomData<StateHigh>,
        state_low: PhantomData<StateLow>,
    }

    pub type FixedLowAlternateDrive = SingleState<LowAlternateDrive>;
    pub type FixedLow = SingleState<Low>;
    pub type FixedPullDown = SingleState<PullDown>;

    pub type Disabled = SingleState<Floating>;
    pub type FixedFloating = SingleState<Floating>;

    pub type FixedPullUp = SingleState<PullUp>;
    pub type FixedHigh = SingleState<High>;
    pub type FixedHighAlternateDrive = SingleState<HighAlternateDrive>;

    pub type PushPull = MultiState<High, Low>;
    pub type PushPullAlternateDrive = MultiState<HighAlternateDrive, LowAlternateDrive>;

    pub type WiredOr = MultiState<High, Floating>;
    pub type WiredOrPullDown = MultiState<High, PullDown>;

    pub type WiredAnd = MultiState<Floating, Low>;
    pub type WiredAndPullUp = MultiState<PullUp, Low>;

    pub type WiredAndAlternateDrive = MultiState<Floating, LowAlternateDrive>;
    pub type WiredAndAlternateDrivePullUp = MultiState<PullUp, LowAlternateDrive>;

    pub type PullUpOrDown = MultiState<PullUp, PullDown>;
    pub type PullUpOrFloating = MultiState<PullUp, Floating>;
}

pub struct PinMode<InputMode, OutputMode> {
    input_mode: PhantomData<InputMode>,
    output_mode: PhantomData<OutputMode>,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl PinMode<input_modes::Disabled, output_modes::Disabled> {
    #[inline]
    pub fn new() -> PinMode<input_modes::Disabled, output_modes::Disabled> {
        PinMode {
            input_mode: PhantomData,
            output_mode: PhantomData,
        }
    }
}

impl<InputMode, OutputMode> PinMode<InputMode, OutputMode> {
    #[inline]
    fn transmute<NewInput, NewOutput>(self) -> PinMode<NewInput, NewOutput> {
        let _ = self;
        PinMode {
            input_mode: PhantomData,
            output_mode: PhantomData,
        }
    }

    #[inline]
    pub fn input_enable(self) -> PinMode<input_modes::Enabled, OutputMode> {
        self.transmute()
    }

    #[inline]
    pub fn input_filter(self) -> PinMode<input_modes::Filtered, OutputMode> {
        self.transmute()
    }

    #[inline]
    pub fn input_disable(self) -> PinMode<input_modes::Disabled, OutputMode> {
        self.transmute()
    }

    #[inline]
    pub fn output_disable(self) -> PinMode<InputMode, output_modes::Disabled> {
        self.transmute()
    }

    #[inline]
    pub fn set_low_alternate_drive(
        self,
    ) -> PinMode<InputMode, output_modes::FixedLowAlternateDrive> {
        self.transmute()
    }

    #[inline]
    pub fn set_low(self) -> PinMode<InputMode, output_modes::FixedLow> {
        self.transmute()
    }

    #[inline]
    pub fn set_pull_down(self) -> PinMode<InputMode, output_modes::FixedPullDown> {
        self.transmute()
    }

    #[inline]
    pub fn set_floating(self) -> PinMode<InputMode, output_modes::FixedFloating> {
        self.transmute()
    }

    #[inline]
    pub fn set_pull_up(self) -> PinMode<InputMode, output_modes::FixedPullUp> {
        self.transmute()
    }

    #[inline]
    pub fn set_high(self) -> PinMode<InputMode, output_modes::FixedHigh> {
        self.transmute()
    }

    #[inline]
    pub fn set_high_alternate_drive(
        self,
    ) -> PinMode<InputMode, output_modes::FixedHighAlternateDrive> {
        self.transmute()
    }

    #[inline]
    pub fn push_pull(self) -> PinMode<InputMode, output_modes::PushPull> {
        self.transmute()
    }

    #[inline]
    pub fn push_pull_alternate_drive(
        self,
    ) -> PinMode<InputMode, output_modes::PushPullAlternateDrive> {
        self.transmute()
    }

    #[inline]
    pub fn wired_or(self) -> PinMode<InputMode, output_modes::WiredOr> {
        self.transmute()
    }

    #[inline]
    pub fn wired_or_pull_down(self) -> PinMode<InputMode, output_modes::WiredOrPullDown> {
        self.transmute()
    }

    #[inline]
    pub fn wired_and(self) -> PinMode<InputMode, output_modes::WiredAnd> {
        self.transmute()
    }

    #[inline]
    pub fn wired_and_pull_up(self) -> PinMode<InputMode, output_modes::WiredAndPullUp> {
        self.transmute()
    }

    #[inline]
    pub fn wired_and_alternate_drive(
        self,
    ) -> PinMode<InputMode, output_modes::WiredAndAlternateDrive> {
        self.transmute()
    }

    #[inline]
    pub fn wired_and_alternate_drive_pull_up(
        self,
    ) -> PinMode<InputMode, output_modes::WiredAndAlternateDrivePullUp> {
        self.transmute()
    }

    #[inline]
    pub fn pull_up_or_down(self) -> PinMode<InputMode, output_modes::PullUpOrDown> {
        self.transmute()
    }

    #[inline]
    pub fn pull_up_or_floating(self) -> PinMode<InputMode, output_modes::PullUpOrFloating> {
        self.transmute()
    }
}

pub trait ValidPinMode {
    const MODE: u8;
    const DOUT: Option<bool>;
}

macro_rules! to_input_mode {
    (disabled __) => {
        input_modes::Disabled
    };
    (enabled __) => {
        input_modes::Enabled
    };
    (enabled on) => {
        input_modes::Filtered
    };
}

macro_rules! to_output_mode {
    (disabled __ __ __) => {
        output_modes::FixedFloating
    };
    (disabled on __ __) => {
        output_modes::FixedPullDown
    };
    (disabled __ on __) => {
        output_modes::FixedPullUp
    };
    (push_pull __ __ __) => {
        output_modes::PushPull
    };
    (push_pull __ __ on) => {
        output_modes::PushPullAlternateDrive
    };
    (open_source __ __ __) => {
        output_modes::WiredOr
    };
    (open_source on __ __) => {
        output_modes::WiredOrPullDown
    };
    (open_drain __ __ __) => {
        output_modes::WiredAnd
    };
    (open_drain __ on __) => {
        output_modes::WiredAndPullUp
    };
    (open_drain __ __ on) => {
        output_modes::WiredAndAlternateDrive
    };
    (open_drain __ on on) => {
        output_modes::WiredAndAlternateDrivePullUp
    };
}

macro_rules! dout_to_expr {
    (0) => {
        Some(false)
    };
    (1) => {
        Some(true)
    };
    (x) => {
        None
    };
}

macro_rules! pin_mode {
    ($mode:tt $input:tt $output:tt $dout:tt $pulldown:tt $pullup:tt $altstr:tt $filter:tt) => {
        impl ValidPinMode
            for PinMode<
                to_input_mode!($input $filter),
                to_output_mode!($output $pulldown $pullup $altstr),
            >
        {
            const MODE: u8 = $mode;
            const DOUT: Option<bool> = dout_to_expr!($dout);
        }
    };
}

pin_mode!(0b0000 disabled disabled    0 __ __ __ __);
pin_mode!(0b0000 disabled disabled    1 __ on __ __);
pin_mode!(0b0001 enabled  disabled    0 __ __ __ __);
pin_mode!(0b0001 enabled  disabled    1 __ __ __ on);
pin_mode!(0b0010 enabled  disabled    0 on __ __ __);
pin_mode!(0b0010 enabled  disabled    1 __ on __ __);
pin_mode!(0b0011 enabled  disabled    0 on __ __ on);
pin_mode!(0b0011 enabled  disabled    1 __ on __ on);
pin_mode!(0b0100 enabled  push_pull   x __ __ __ __);
pin_mode!(0b0101 enabled  push_pull   x __ __ on __);
pin_mode!(0b0110 enabled  open_source x __ __ __ __);
pin_mode!(0b0111 enabled  open_source x on __ __ __);
pin_mode!(0b1000 enabled  open_drain  x __ __ __ __);
pin_mode!(0b1001 enabled  open_drain  x __ __ __ on);
pin_mode!(0b1010 enabled  open_drain  x __ on __ __);
pin_mode!(0b1011 enabled  open_drain  x __ on __ on);
pin_mode!(0b1100 enabled  open_drain  x __ __ on __);
pin_mode!(0b1101 enabled  open_drain  x __ __ on on);
pin_mode!(0b1110 enabled  open_drain  x __ on on __);
pin_mode!(0b1111 enabled  open_drain  x __ on on on);

// These extra impls do not correspond 1-to-1 to a single line in the table. The first three impls
// are a result of combining two lines (effectively forming an x-line out of a 0 and a 1 line).
// The last 6 are a result of taking an x-line and fixing the value of x to either 0 or 1.
impl ValidPinMode for PinMode<input_modes::Disabled, output_modes::PullUpOrFloating> {
    const MODE: u8 = 0b0000;
    const DOUT: Option<bool> = None;
}

impl ValidPinMode for PinMode<input_modes::Enabled, output_modes::PullUpOrDown> {
    const MODE: u8 = 0b0010;
    const DOUT: Option<bool> = None;
}

impl ValidPinMode for PinMode<input_modes::Filtered, output_modes::PullUpOrDown> {
    const MODE: u8 = 0b0011;
    const DOUT: Option<bool> = None;
}

impl ValidPinMode for PinMode<input_modes::Enabled, output_modes::FixedLowAlternateDrive> {
    const MODE: u8 = 0b1100;
    const DOUT: Option<bool> = Some(false);
}

impl ValidPinMode for PinMode<input_modes::Filtered, output_modes::FixedLowAlternateDrive> {
    const MODE: u8 = 0b1101;
    const DOUT: Option<bool> = Some(false);
}

impl ValidPinMode for PinMode<input_modes::Enabled, output_modes::FixedLow> {
    const MODE: u8 = 0b1000;
    const DOUT: Option<bool> = Some(false);
}

impl ValidPinMode for PinMode<input_modes::Filtered, output_modes::FixedLow> {
    const MODE: u8 = 0b1001;
    const DOUT: Option<bool> = Some(false);
}

impl ValidPinMode for PinMode<input_modes::Enabled, output_modes::FixedHigh> {
    const MODE: u8 = 0b0110;
    const DOUT: Option<bool> = Some(true);
}

impl ValidPinMode for PinMode<input_modes::Enabled, output_modes::FixedHighAlternateDrive> {
    const MODE: u8 = 0b0101;
    const DOUT: Option<bool> = Some(true);
}

pub struct UnknownMode;
