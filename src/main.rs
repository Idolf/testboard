#![feature(asm, lang_items, pin, const_fn, panic_implementation,core_panic_info, used)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

//extern crate cortex_m_semihosting;

#[macro_use(interrupt)]
extern crate efm32hg309f64;

extern crate heapless;
//extern crate panic_semihosting;

pub mod leuart;

use rt::ExceptionFrame;
use core::panic::*;

#[panic_implementation]
#[no_mangle]
pub fn panic_impl(_pi:&PanicInfo) -> ! {
    //cortex_m::interrupt::disable();
    //cortex_m::asm::bkpt();
    loop { cortex_m::asm::wfi() }
}

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
    leuart::init_leuart(true, true, 9600.0, &ep.LEUART0);
    // init_leuart(true, true, 9600.0, &ep.LEUART0);
    init_nvic(&mut cp.NVIC);

    loop {
//        let mut byte = [0];
//        leuart::leuart_read(&mut byte);
        leuart::write(b"Hello!\n");
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


exception!(*, default_handler);
fn default_handler(_irqn: i16) {
    loop {}
}
