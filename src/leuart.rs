// use cmu::Clock;
use core::f64;
use core::fmt;
use core::marker::PhantomData;
use core::mem;
// use cortex_m;
use cmu;
use efm32hg309f64;

use gpio::*;
// use heapless::consts::*;
// use heapless::RingBuffer;
// use typenum;

// static mut LEUART_RXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();
// static mut LEUART_TXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();

#[allow(unused)]
#[cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
fn scaling_factor(frequency: f64, baudrate: f64) -> u16 {
    // The formulas in EFM32-HF-RM 17.3.3 says how to calculate the value of leuart.clkdiv from the
    // desired baud rate. What it does not say is that the 3 lower bits of leuart.clkdiv must be 0
    // (this is specified in 17.5.4).
    //
    // For instance they specify 616 as the value used to best achieve 9600 baud when using a 32768
    // Hz clock (such as the lfrco). Actually a better value would be 617 or 618, but these are disallowed.
    //
    // The fact that the lower bits must be zero is also noticeable in the svd file. At some point we
    // had a bug where we tried to set w.div().bits(616), which is equivalent to w.bits(616 << 3) -- which obviously
    // did not go well.
    //
    // Here we are using a derived fomula for finding the correct value of clkdiv from the baud rate.
    // The result from here is meant to be put into w.div().bits(...)
    // instead of w.bits(...), so if you try to port this to something else, make sure to shift this
    // up accordingly.
    let scaling_factor = 32.0 * (frequency / baudrate - 1.0);
    let scaling_factor = f64::max(0.0, scaling_factor);
    let scaling_factor = f64::min(0b1111_1111_1111 as f64, scaling_factor);
    (scaling_factor + 0.5) as u16
}

#[allow(unused)]
#[cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
fn recalculated_baudrate(frequency: f64, scaling_factor: u16) -> f64 {
    frequency / (1.0 + (scaling_factor as f64) / 32.0)
}

#[allow(unused)]
fn error(baudrate: f64, recalculated_baudrate: f64) -> f64 {
    let error = 1.0 - recalculated_baudrate / baudrate;
    if error >= 0.0 {
        error
    } else {
        -error
    }
}

pub trait LocationPins {
    type TxPin;
    type RxPin;
}

macro_rules! location {
    ($location_id:ident, $tx_pin:ident, $rx_pin:ident) => {
        pub struct $location_id;
        impl LocationPins for $location_id {
            type TxPin = $tx_pin<
                pin_modes::PinMode<
                    pin_modes::input_modes::Enabled,
                    pin_modes::output_modes::PushPull,
                >,
            >;
            type RxPin = $rx_pin<
                pin_modes::PinMode<
                    pin_modes::input_modes::Enabled,
                    pin_modes::output_modes::PushPull,
                >,
            >;
        }
    };
}

pub struct Location0;
// Location 1 TX==PB13, RX==PB14. See EFM32HF309 Datasheet section 4.2
location!(Location1, Pb13, Pb14);

pub struct TxOn;
pub struct TxOff;

pub struct RxOn;
pub struct RxOff;

pub struct ClockOff;
pub struct ClockOn;

pub struct Leuart<'devices, Location, TxMode, RxMode, ClockState> {
    location: PhantomData<Location>,
    tx_mode: PhantomData<TxMode>,
    rx_mode: PhantomData<RxMode>,
    clock_state: PhantomData<ClockState>,
    devices: PhantomData<&'devices ()>,
    non_send: PhantomData<*mut ()>,
}

impl<'devices, Location, TxMode, RxMode, ClockState>
    Leuart<'devices, Location, TxMode, RxMode, ClockState>
{
    #[inline]
    unsafe fn transmute_mode<'new_devices, NewLocation, NewTxMode, NewRxMode, NewClockState>(
        self,
    ) -> Leuart<'new_devices, NewLocation, NewTxMode, NewRxMode, NewClockState> {
        mem::forget(self);
        Leuart {
            location: PhantomData,
            tx_mode: PhantomData,
            rx_mode: PhantomData,
            clock_state: PhantomData,
            devices: PhantomData,
            non_send: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn claim_ownership() -> Self {
        Leuart {
            location: PhantomData,
            tx_mode: PhantomData,
            rx_mode: PhantomData,
            clock_state: PhantomData,
            devices: PhantomData,
            non_send: PhantomData,
        }
    }

    #[inline]
    pub fn baudrate<InnerSource, InnerDiv>(
        self,
        clk: &'devices cmu::lfb::LfbClkLeuart0<'devices, InnerSource, InnerDiv>,
        baudrate: f64,
    ) -> Leuart<'devices, Location, TxMode, RxMode, ClockOn>
    where
        cmu::lfb::LfbClkLeuart0<'devices, InnerSource, InnerDiv>: cmu::Clock,
    {
        use cmu::Clock;
        let _ = clk;
        let frequency = cmu::lfb::LfbClkLeuart0::<InnerSource, InnerDiv>::FREQUENCY;
        let scaling_factor = scaling_factor(frequency, baudrate);
        let recalculated_baudrate = recalculated_baudrate(frequency, scaling_factor);
        let error = error(baudrate, recalculated_baudrate);
        debug_assert!(error < 0.025);

        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        regs.clkdiv
            .write(|w| unsafe { w.div().bits(scaling_factor) });
        while regs.syncbusy.read().clkdiv().bit_is_set() {}
        regs.ctrl.modify(|_, w| w.stopbits().set_bit());
        while regs.syncbusy.read().ctrl().bit_is_set() {}
        unsafe { self.transmute_mode() }
    }

    #[inline]
    pub fn disable(self) -> Leuart<'static, Location, TxOff, RxOff, ClockOff> {
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        regs.cmd.write(|w| {
            w.txdis()
                .set_bit()
                .rxdis()
                .set_bit()
                .cleartx()
                .set_bit()
                .clearrx()
                .set_bit()
        });
        while regs.syncbusy.read().cmd().bit_is_set() {}

        unsafe { self.transmute_mode() }
    }
}

impl Leuart<'static, Location0, TxOff, RxOff, ClockOff> {
    #[inline]
    pub unsafe fn get_initial_state(
        leuart: efm32hg309f64::LEUART0,
    ) -> Leuart<'static, Location0, TxOff, RxOff, ClockOff> {
        let _ = leuart;
        Leuart::claim_ownership()
    }
}

impl<'devices, Location, TxMode, RxMode> Leuart<'devices, Location, TxMode, RxMode, ClockOn> {
    #[inline]
    pub fn location1(self) -> Leuart<'devices, Location1, TxMode, RxMode, ClockOn> {
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        regs.route.write(|w| w.location().loc1());

        unsafe { self.transmute_mode() }
    }
}

impl<'devices, Location: LocationPins, TxMode, RxMode>
    Leuart<'devices, Location, TxMode, RxMode, ClockOn>
{
    pub fn enable_tx(
        self,
        tx_pin: &'devices mut Location::TxPin,
    ) -> Leuart<'devices, Location, TxOn, RxMode, ClockOn> {
        let _ = tx_pin;
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        regs.route.modify(|_, w| w.txpen().set_bit());
        regs.cmd.write(|w| w.txen().set_bit().cleartx().set_bit());
        while regs.syncbusy.read().cmd().bit_is_set() {}

        unsafe { self.transmute_mode() }
    }

    pub fn enable_rx(
        self,
        rx_pin: &'devices mut Location::RxPin,
    ) -> Leuart<'devices, Location, TxMode, RxOn, ClockOn> {
        let _ = rx_pin;
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        regs.route.modify(|_, w| w.rxpen().set_bit());
        regs.cmd.write(|w| w.rxen().set_bit().clearrx().set_bit());
        while regs.syncbusy.read().cmd().bit_is_set() {}

        unsafe { self.transmute_mode() }
    }
}

impl<'devices, Location, RxMode> Leuart<'devices, Location, TxOn, RxMode, ClockOn> {
    pub fn write_blocking(&mut self, bytes: &[u8]) {
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
        for &b in bytes {
            regs.txdata.write(|w| unsafe { w.txdata().bits(b) });
            while regs.status.read().txc().bit_is_clear() {}
        }
    }
}

impl<'devices, Location, TxMode> Leuart<'devices, Location, TxMode, RxOn, ClockOn> {
    pub unsafe fn read_blocking(&mut self, buf: &mut [u8]) {
        let regs = &*efm32hg309f64::LEUART0::ptr();
        for b in buf.iter_mut() {
            while regs.status.read().rxdatav().bit_is_clear() {}
            *b = regs.rxdata.read().rxdata().bits();
        }
    }
}

impl<'devices, Location, RxMode> fmt::Write for Leuart<'devices, Location, TxOn, RxMode, ClockOn> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_blocking(s.as_bytes());
        Ok(())
    }
}

//     pub fn write(&self, buf: &[u8]) {
//         let tx_producer = unsafe { &mut LEUART_TXBUF };
//         let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };

//         for byte in buf {
//             while tx_producer.enqueue(*byte).is_err() {
//                 cortex_m::asm::wfe()
//             }
//         }
//         cortex_m::interrupt::free(|_| {
//             regs.ien.modify(|_, w| w.txbl().set_bit());
//             regs.cmd.write(|w| w.txen().set_bit());
//         });
//     }

//     pub fn read(&self, buf: &mut [u8]) {
//         let rx_consumer = unsafe { &mut LEUART_RXBUF };
//         for ptr in buf.iter_mut() {
//             loop {
//                 match rx_consumer.dequeue() {
//                     Some(byte) => {
//                         *ptr = byte;
//                         break;
//                     }
//                     None => cortex_m::asm::wfi(),
//                 }
//             }
//         }
//     }
// }

// interrupt!(LEUART0, leuart0_handler);
// fn leuart0_handler() {
//     let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
//     let leuart_if: efm32hg309f64::leuart0::if_::R = regs.if_.read();

//     if leuart_if.txbl().bit_is_set() {
//         let tx_consumer = unsafe { &mut LEUART_TXBUF };
//         match tx_consumer.dequeue() {
//             Some(byte) => regs.txdata.write(|w| unsafe { w.txdata().bits(byte) }),
//             None => regs.ien.modify(|_, w| w.txbl().clear_bit()),
//         }
//     }

//     if leuart_if.rxdatav().bit_is_set() {
//         let rx_producer = unsafe { &mut LEUART_RXBUF };
//         let byte = regs.rxdata.read().rxdata().bits();
//         rx_producer.enqueue(byte).ok();
//         cortex_m::asm::sev();
//     }
// }
