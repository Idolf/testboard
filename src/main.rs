#![feature(panic_implementation)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

#[macro_use]
extern crate static_assertions;

#[macro_use(interrupt)]
extern crate efm32hg309f64;

extern crate heapless;

extern crate typenum;

pub mod cmu;
pub mod consts;
pub mod device_information;
pub mod devices;
pub mod gpio;
pub mod leuart;
pub mod nvic;
pub mod panic;
pub mod usb;

use devices::StaticDevice;
// use gpio::*;
// use leuart::*;
// use embedded_hal::digital::OutputPin;

fn init_wdog(wdog: &efm32hg309f64::wdog::RegisterBlock) {
    // Disable the watchdog
    wdog.ctrl.reset();
}

fn init_cmu(
    cmu: cmu::InitialCmuState,
) -> &'static cmu::lfb::LfbClkLeuart0<'static, impl cmu::Clock, typenum::consts::U8> {
    // Initialize source clocks
    let lfrco = cmu.lfrco.enable_32768hz().finalize();
    let hfrco = cmu.hfrco.enable_21mhz().finalize();
    let ushfrco = cmu.ushfrco.enable_48mhz().finalize();

    // Initialize the main clocks
    let hfclk = cmu.hfclk.div1().hfrco(hfrco).finalize();
    let hfcoreclk = cmu.hfcoreclk.div1(hfclk).finalize();
    let hfcoreclklediv = cmu.hfcoreclklediv.enable_div4(hfcoreclk).finalize();
    let hfperclk = cmu.hfperclk.enable_div1(hfclk).finalize();

    // Initialize the three low-frequency clocks
    let lfa = cmu.lfaclk.enable_lfrco(lfrco).finalize();
    let lfb = cmu.lfbclk.enable_hfcoreclklediv(hfcoreclklediv).finalize();
    let lfc = cmu.lfcclk.enable_lfrco(lfrco).finalize();

    // Initialize the usb clocks
    let _usb = cmu.hfcoreclkusb.enable(hfcoreclk).finalize();
    let _usbc = cmu.hfcoreclkusbc.enable_ushfrco(ushfrco).finalize();
    let _usble = cmu.lfcclkusble.enable(lfc).finalize();

    // Initialize peripherals
    let _gpio = cmu.hfperclkgpio.enable(hfperclk).finalize();
    let _dma = cmu.hfcoreclkdma.enable(hfcoreclk).finalize();
    let _rtc = cmu.lfaclkrtc.enable_div1(lfa).finalize();
    let leuart = cmu.lfbclkleuart0.enable_div8(lfb).finalize();

    leuart
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

entry!(main);
#[inline]
fn main() -> ! {
    let ep = efm32hg309f64::Peripherals::take().unwrap();
    let cp = efm32hg309f64::CorePeripherals::take().unwrap();

    init_wdog(&ep.WDOG);

    let cmu = unsafe { cmu::InitialCmuState::get_initial_state(ep.CMU) };
    let lfb_leuart = init_cmu(cmu);
    init_rtc(80, &ep.RTC);

    let gpio = unsafe { gpio::InitialGpioState::get_initial_state(ep.GPIO) };

    let mut pb13 = gpio
        .pb13
        .mode(gpio::pin_modes::PinMode::new().push_pull().input_enable());

    let leuart = unsafe { leuart::Leuart::get_initial_state(ep.LEUART0) };
    let mut leuart = leuart
        .baudrate(&lfb_leuart, 115200.0)
        .location1()
        .enable_tx(&mut pb13);

    let mut rtc_handler = nvic::InterruptHandler::new(|| {
        let rtc = unsafe { &*efm32hg309f64::RTC::ptr() };

        rtc.ifc
            .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());
    });

    nvic::Nvic::new(cp.NVIC).with_handler(|handler| {
        handler.register(efm32hg309f64::Interrupt::RTC, &mut rtc_handler);
        loop {
            leuart.write_blocking(b"Hello!\n");
        }
    })
}

exception!(*, default_handler);
fn default_handler(_irqn: i16) {
    loop {
        cortex_m::asm::nop()
    }
}
