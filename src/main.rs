#![feature(asm, lang_items, pin, const_fn)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate cortex_m_semihosting;

#[macro_use(interrupt)]
extern crate efm32hg309f64;

extern crate heapless;
extern crate panic_semihosting;

use heapless::consts::*;
use heapless::RingBuffer;

use rt::ExceptionFrame;

fn init_wdog(wdog: &efm32hg309f64::wdog::RegisterBlock) {
    // Disable the watchdog
    wdog.ctrl.reset();
}

fn init_clock(cmu: &efm32hg309f64::cmu::RegisterBlock) {
    // See section 11.3 in the EFM32HF-RM for an overview of these clocks

    // Enable rco's
    cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
    cmu.oscencmd.write(|w| w.hfrcoen().set_bit());

    // Wait for rco's
    while cmu.status.read().lfrcordy().bit_is_clear() {}
    while cmu.status.read().hfrcordy().bit_is_clear() {}

    // Choose 21MHz for the hfrco and calibrate it
    //
    // The lfrco does not need calibration; it's reset value is set to the correct calibration
    // automatically
    fn get_hfrco_calib_band_21() -> u8 {
        const HFRCO_CALIB_BAND_21: usize = 0x0fe081e0;
        unsafe { *(HFRCO_CALIB_BAND_21 as *const u8) }
    }
    cmu.hfrcoctrl
        .write(|w| unsafe { w.band()._21mhz().tuning().bits(get_hfrco_calib_band_21()) });

    // Enable the high frequency peripheral clock (hfperclk)
    cmu.hfperclkdiv.write(|w| w.hfperclken().set_bit());

    // Enable the gpio to use the hfperclk
    cmu.hfperclken0.write(|w| w.gpio().set_bit());

    // Enable the high frequency core clock for low energy peripherals
    cmu.hfcoreclken0.write(|w| w.le().set_bit());

    // Set clock source for lfa to use the lfrco and lfb to use hfcoreclk_le. The div2 is actually a
    // lie, because we set it to div4 inside cmu.hfcoreclkdiv.
    cmu.lfclksel
        .modify(|_, w| w.lfa().lfrco().lfb().hfcoreclklediv2());
    cmu.hfcoreclkdiv.write(|w| w.hfcoreclklediv().set_bit());

    // Prescale the leuart0 clock by a factor of 8.
    cmu.lfbpresc0.write(|w| w.leuart0().div8());

    // Enable the rtc to use the lfrco (though the lfa)
    cmu.lfaclken0.write(|w| w.rtc().set_bit());

    // Enable the rtc to use the leuart0 (though the lfb)
    cmu.lfbclken0.write(|w| w.leuart0().set_bit());
}

fn init_rtc(ms: u32, rtc: &efm32hg309f64::rtc::RegisterBlock) {
    // Set the rtc compare value
    let ticks_per_1000ms = 32768;
    let ticks_per_cycle = ticks_per_1000ms * ms / 1000;
    rtc.comp0
        .write(|w| unsafe { w.comp0().bits(ticks_per_cycle) });

    // Enable the rtc and set the rtc to use the compare value above
    rtc.ctrl.write(|w| w.en().set_bit().comp0top().set_bit());

    // Enable interrupts for the rtc
    rtc.ien.write(|w| w.comp0().set_bit());
}

fn init_leuart(
    enable_tx: bool,
    enable_rx: bool,
    baud_rate: f32,
    leuart: &efm32hg309f64::leuart0::RegisterBlock,
) {
    // The formulas in EFM32-HF-RM 17.3.3 says how to calculate the value of leuart.clkdiv from the
    // desired baud rate. What it does not say is that the 3 lower bits of leuart.clkdiv must be 0
    // (this is specified in 17.5.4).
    //
    // For instance they specify 616 as the value used to best achieve 9600 baud when using a 32768
    // Hz clock (such as the lfrco). Actually a better value would be 617 or 618.
    //
    // The fact that the lower bits must be zero is also noticeable in the svd file. For a while we
    // tried to set w.div().bits(616), which is equivalent to w.bits(616 << 3) -- which obviously
    // did not go well.
    //
    // Here we are using a derived fomula for finding the correct value of clkdiv from the baud rate
    // assuming that we are using the hfrco set up to 21MHz. Additionally use the w.div().bits(...)
    // instead of w.bits(...), so if you try to port this to something else, make sure to shift this
    // up accordingly.
    //
    // Using this setup we can support all the following baud ranges:
    // - At most 2.0% error: 4_989 - 669_642 baud
    // - At most 1.5% error: 5_014 - 626_866 baud
    // - At most 1.0% error: 5_039 - 424_242 baud
    // - At most 0.5% error: 5_064 - 211_055 baud
    let hfcoreclk_prescaler = 4.0;
    let leuart_prescaler = 8.0;
    let hf_frequency = 21_000_000.0;
    let source_clock_frequency = hf_frequency / hfcoreclk_prescaler / leuart_prescaler;
    let scale_factor: f32 = 32.0 * (source_clock_frequency / baud_rate - 1.0);
    let scale_factor = if scale_factor < 0.0 {
        0
    } else if scale_factor >= 0b111111111111 as f32 {
        0b111111111111
    } else {
        (scale_factor + 0.5) as u16
    };

    leuart
        .clkdiv
        .write(|w| unsafe { w.div().bits(scale_factor) });

    leuart.route.write(|w| {
        // Use location 1 for the route
        // Location 1 TX==PB13, RX==PB14. See EFM32HF309 Datasheet section 4.2
        w.location().loc1();

        // Enable the tx pin
        if enable_tx {
            w.txpen().set_bit();
        }
        // Enable the rx pin
        if enable_rx {
            w.rxpen().set_bit();
        }
        w
    });

    leuart.cmd.write(|w| {
        // Clear rx and tx buffers
        w.clearrx().set_bit().cleartx().set_bit();

        // Enable generation of tx signals
        if enable_tx {
            w.txen().set_bit();
        }

        // Enable reception of rx signals
        if enable_rx {
            w.rxen().set_bit();
        }
        w
    });

    // Enable interrupts on received data
    leuart.ien.write(|w| w.rxdatav().set_bit());
}

fn init_gpio(gpio: &efm32hg309f64::gpio::RegisterBlock) {
    // Set the mode for PA0 to pushpull
    gpio.pa_model.modify(|_, w| w.mode0().pushpull());

    // Set the mode for PB13 to pushpull and for PB14 to input.
    gpio.pb_modeh
        .modify(|_, w| w.mode13().pushpull().mode14().input());
}

fn init_nvic(nvic: &mut cortex_m::peripheral::NVIC) {
    nvic.enable(efm32hg309f64::Interrupt::RTC);
    nvic.enable(efm32hg309f64::Interrupt::LEUART0);
}

entry!(main);
#[inline]
fn main() -> ! {
    let ep = efm32hg309f64::Peripherals::take().unwrap();
    let mut cp = efm32hg309f64::CorePeripherals::take().unwrap();

    init_wdog(&ep.WDOG);
    init_clock(&ep.CMU);
    init_gpio(&ep.GPIO);
    init_rtc(80, &ep.RTC);
    init_leuart(true, true, 128_000.0, &ep.LEUART0);
    // init_leuart(true, true, 9600.0, &ep.LEUART0);
    init_nvic(&mut cp.NVIC);

    let mut rx_consumer = unsafe { LEUART_RXBUF.split().1 };
    let mut tx_producer = unsafe { LEUART_TXBUF.split().0 };

    loop {
        let byte = loop {
            match rx_consumer.dequeue() {
                Some(data) => break data,
                None => cortex_m::asm::wfi(),
            }
        };

        if byte.is_ascii_uppercase() {
            tx_producer.enqueue(byte | 0x20).ok();
        } else if byte.is_ascii_lowercase() {
            tx_producer.enqueue((byte & !0x20) + 1).ok();
        }
        ep.LEUART0.ien.modify(|_, w| w.txbl().set_bit());
    }
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_ef: &ExceptionFrame) -> ! {
    loop {}
}

interrupt!(RTC, rtc_handler);
fn rtc_handler() {
    let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
    let rtc = unsafe { &*efm32hg309f64::RTC::ptr() };

    rtc.ifc
        .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());

    gpio.pa_douttgl.write(|w| unsafe { w.douttgl().bits(1) });
}

static mut LEUART_RXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();
static mut LEUART_TXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();

interrupt!(LEUART0, leuart0_handler);
fn leuart0_handler() {
    let leuart = unsafe { &*efm32hg309f64::LEUART0::ptr() };

    let leuart_if: efm32hg309f64::leuart0::if_::R = leuart.if_.read();

    if leuart_if.txbl().bit_is_set() {
        let mut tx_consumer = unsafe { LEUART_TXBUF.split().1 };
        match tx_consumer.dequeue() {
            Some(byte) => leuart.txdata.write(|w| unsafe { w.txdata().bits(byte) }),
            None => leuart.ien.modify(|_, w| w.txbl().clear_bit()),
        }
    }

    if leuart_if.rxdatav().bit_is_set() {
        let mut rx_producer = unsafe { LEUART_RXBUF.split().0 };
        let byte = leuart.rxdata.read().rxdata().bits();
        rx_producer.enqueue(byte).ok();
    }
}

exception!(*, default_handler);
fn default_handler(_irqn: i16) {
    loop {}
}
