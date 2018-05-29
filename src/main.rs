#![feature(asm, lang_items)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate efm32hg309f64;

use rt::ExceptionFrame;

fn init_clock(cmu: &efm32hg309f64::cmu::RegisterBlock) {
    cmu.hfperclkdiv.write(|w| w.hfperclken().set_bit());
    cmu.hfperclken0.write(|w| w.gpio().set_bit());
    cmu.lfclksel.modify(|_, w| w.lfc().lfrco());
    cmu.lfaclken0.write(|w| w.rtc().set_bit());
    cmu.oscencmd.write(|w| w.hfrcoen().set_bit());

    while cmu.status.read().hfrcordy().bit_is_clear() {}

    cmu.cmd.write(|w| w.hfclksel().hfrco());

    while cmu.status.read().hfrcosel().bit_is_clear() {}

    cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
}

fn sleep(n: u32) {
    for _ in 0..n {
        unsafe {
            asm!("nop");
        }
    }
}

entry!(main);
#[inline]
fn main() -> ! {
    let peripherals = unsafe { efm32hg309f64::Peripherals::steal() };
    peripherals.WDOG.ctrl.write(|w| unsafe { w.bits(0) });

    init_clock(&peripherals.CMU);
    let gpio = peripherals.GPIO;
    gpio.pa_model.modify(|_, w| w.mode0().wiredand());
    gpio.pa_doutset.write(|w| unsafe { w.doutset().bits(1) });

    loop {
        sleep(100_000);
        gpio.pa_douttgl.write(|w| unsafe { w.douttgl().bits(1) });
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    loop {}
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
extern "C" fn panic_fmt() -> ! {
    loop {}
}
