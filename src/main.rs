#![feature(asm, lang_items)]
#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

#[macro_use]
extern crate cortex_m_semihosting;

#[macro_use(interrupt)]
extern crate efm32hg309f64;

extern crate panic_semihosting;

use rt::ExceptionFrame;

fn get_21mhz_hfrco_calibration() -> u8 {
    const ADDR: usize = 0x0fe081e0;
    unsafe { *(ADDR as *const u8) }
}

fn init_clock(cmu: &efm32hg309f64::cmu::RegisterBlock) {
    cmu.hfperclkdiv.write(|w| w.hfperclken().set_bit());
    cmu.hfperclken0.write(|w| w.gpio().set_bit());
    cmu.lfclksel.modify(|_, w| w.lfc().lfrco());
    cmu.lfaclken0.write(|w| w.rtc().set_bit());

    cmu.hfrcoctrl.write(|w| unsafe {
        w.band()
            ._21mhz()
            .tuning()
            .bits(get_21mhz_hfrco_calibration())
    });
    cmu.oscencmd.write(|w| w.hfrcoen().set_bit());

    while cmu.status.read().hfrcordy().bit_is_clear() {}

    cmu.cmd.write(|w| w.hfclksel().hfrco());

    while cmu.status.read().hfrcosel().bit_is_clear() {}

    cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
}

fn init_rtc(rtc: &efm32hg309f64::rtc::RegisterBlock) {
    rtc.ifc
        .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());
    rtc.comp0
        .write(|w| unsafe { w.comp0().bits(0x8000 * 50 / 1000) });
    rtc.ien.write(|w| w.comp0().set_bit());

    rtc.ctrl
        .write(|w| w.comp0top().set_bit().debugrun().set_bit().en().set_bit());
}

fn sleep(n: u32) {
    for _ in 0..n {
        cortex_m::asm::nop();
    }
}

entry!(main);
#[inline]
fn main() -> ! {
    // const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
    // static MSG: &'static [u8] = b"main!\n";

    // // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
    // let r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };

    let peripherals: efm32hg309f64::Peripherals = efm32hg309f64::Peripherals::take().unwrap();
    let mut core_peripherals = unsafe { efm32hg309f64::CorePeripherals::steal() };
    core_peripherals.NVIC.enable(efm32hg309f64::Interrupt::RTC);

    peripherals.WDOG.ctrl.write(|w| unsafe { w.bits(0) });

    init_clock(&peripherals.CMU);
    let gpio = peripherals.GPIO;
    gpio.pa_model.modify(|_, w| w.mode0().wiredand());
    gpio.pa_doutset.write(|w| unsafe { w.doutset().bits(1) });

    init_rtc(&peripherals.RTC);

    loop {
        cortex_m::asm::wfi();
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

// define the hard fault handler
interrupt!(RTC, rtc);

fn rtc() {
    let peripherals = unsafe { efm32hg309f64::Peripherals::steal() };
    let gpio = peripherals.GPIO;
    let rtc = peripherals.RTC;

    rtc.ifc
        .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());

    gpio.pa_douttgl.write(|w| unsafe { w.douttgl().bits(1) });
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {}
}

// #[lang = "panic_fmt"]
// #[no_mangle]
// #[allow(private_no_mangle_fns)]
// extern "C" fn panic_fmt() -> ! {
//     loop {}
// }
