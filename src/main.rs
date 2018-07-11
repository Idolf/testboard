#![feature(asm, lang_items, pin, const_fn, panic_implementation, core_panic_info, used)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

#[macro_use]
extern crate static_assertions;

#[macro_use(interrupt)]
extern crate efm32hg309f64;

extern crate heapless;

extern crate typenum;

pub mod cmu;
pub mod device_information;
pub mod devices;
pub mod frequencies;
pub mod gpio;
pub mod usb;
pub mod leuart;

use core::panic::*;
use devices::FinalizeDevice;
use gpio::*;
use usb::*;
use leuart::*;
use rt::ExceptionFrame;

#[panic_implementation]
#[no_mangle]
pub fn panic_impl(_pi: &PanicInfo) -> ! {
    //cortex_m::interrupt::disable();
    //cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfi()
    }
}

fn init_wdog(wdog: &efm32hg309f64::wdog::RegisterBlock) {
    // Disable the watchdog
    wdog.ctrl.reset();
}

fn init_cmu(cmu: cmu::InitialCmuState) {
    // See section 11.3 in the EFM32HF-RM for an overview of these clocks

    // Initialize source clocks
    let lfrco = cmu.lfrco.enable_32768hz().finalize();
    let hfrco = cmu.hfrco.enable_21mhz().finalize();
    let ushfrco = cmu.ushfrco.enable_48mhz().finalize();

    // Initialize the main clocks
    let hfclk = cmu.hfclk.div1().hfrco(&hfrco).finalize();
    let hfcoreclk = cmu.hfcoreclk.div1(&hfclk).finalize();
    let hfcoreclklediv = cmu.hfcoreclklediv.enable_div4(&hfcoreclk).finalize();
    let hfperclk = cmu.hfperclk.enable_div1(&hfclk).finalize();

    // Initialize the three low-frequency clocks
    let lfa = cmu.lfaclk.enable_lfrco(&lfrco).finalize();
    let lfb = cmu.lfbclk.enable_hfcoreclklediv(&hfcoreclklediv).finalize();
    let lfc = cmu.lfcclk.enable_lfrco(&lfrco).finalize();

    // Initialize the usb clocks
    let _usb = cmu.hfcoreclkusb.enable(&hfcoreclk).finalize();
    let _usbc = cmu.hfcoreclkusbc.enable_ushfrco(&ushfrco).finalize();
    let _usble = cmu.lfcclkusble.enable(&lfc).finalize();

    // Initialize peripherals
    let _gpio = cmu.hfperclkgpio.enable(&hfperclk).finalize();
    let _dma = cmu.hfcoreclkdma.enable(&hfcoreclk).finalize();
    let _rtc = cmu.lfaclkrtc.enable_div1(&lfa).finalize();
    let _leuart = cmu.lfbclkleuart0.enable_div8(&lfb).finalize();
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

fn init_nvic(nvic: &mut cortex_m::peripheral::NVIC) {
    nvic.enable(efm32hg309f64::Interrupt::RTC);
    nvic.enable(efm32hg309f64::Interrupt::LEUART0);
    nvic.enable(efm32hg309f64::Interrupt::USB);
}

entry!(main);
#[inline]
fn main() -> ! {
    let ep = efm32hg309f64::Peripherals::take().unwrap();
    let mut cp = efm32hg309f64::CorePeripherals::take().unwrap();

    init_wdog(&ep.WDOG);
    let cmu = unsafe { cmu::InitialCmuState::get_initial_state(ep.CMU) };
    init_cmu(cmu);
    init_rtc(80, &ep.RTC);

    let gpio = gpio::Gpio::init_gpio();
    let mut pins = gpio.pins();

    pins.pa0.mode(gpio::PinMode::PushPull);
    pins.pa0.set();

    let leuart = Leuart::location0(Some(pins.pb13), Some(pins.pb14));
    leuart.baud_rate(9600.0);

//    init_usb(&ep.USB);
    init_nvic(&mut cp.NVIC);

    loop {
        leuart.write(b"Hello!\n");
        pins.pa0.tgl();
    }
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_ef: &ExceptionFrame) -> ! {
    loop {}
}

exception!(*, default_handler);
fn default_handler(_irqn: i16) {
    loop {}
}
