#![feature(asm, lang_items, pin, const_fn, panic_implementation, core_panic_info, used)]
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
pub mod gpio;

use core::panic::*;
use rt::ExceptionFrame;
use gpio::*;
use leuart::*;

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

fn init_clock(cmu: &efm32hg309f64::cmu::RegisterBlock) {
    // See section 11.3 in the EFM32HF-RM for an overview of these clocks

    // Enable rco's
    cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
    while cmu.status.read().lfrcordy().bit_is_clear() {}
    cmu.oscencmd.write(|w| w.hfrcoen().set_bit());
    while cmu.status.read().hfrcordy().bit_is_clear() {}
    cmu.oscencmd.write(|w| w.ushfrcoen().set_bit());
    while cmu.status.read().ushfrcordy().bit_is_clear() {}

    // Wait for rco's

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
    cmu.hfcoreclken0.write(|w| {
        w.le()
            .set_bit()
            .dma()
            .set_bit()
            .usb()
            .set_bit()
            .usbc()
            .set_bit()
    });

    // Set clock source for lfa to use the lfrco and lfb to use hfcoreclk_le. The div2 is actually a
    // lie, because we set it to div4 inside cmu.hfcoreclkdiv.
    cmu.lfclksel
        .modify(|_, w| w.lfa().lfrco().lfb().hfcoreclklediv2().lfc().lfrco());
    cmu.hfcoreclkdiv.write(|w| w.hfcoreclklediv().set_bit());

    // Prescale the leuart0 clock by a factor of 8.
    cmu.lfbpresc0.write(|w| w.leuart0().div8());

    // Enable the rtc to use the lfrco (though the lfa)
    cmu.lfaclken0.write(|w| w.rtc().set_bit());

    // Enable the rtc to use the leuart0 (though the lfb)
    cmu.lfbclken0.write(|w| w.leuart0().set_bit());

    cmu.lfcclken0.write(|w| w.usble().set_bit());

    cmu.cmd.write(|w| w.usbcclksel().ushfrco());
    while cmu.status.read().usbcushfrcosel().bit_is_clear() {}
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

fn init_usb(usb: &efm32hg309f64::usb::RegisterBlock) {
    usb.ctrl.write(|w| {
        w.lemoscctrl()
            .gate()
            .lemidleen()
            .set_bit()
            .lemphyctrl()
            .set_bit()
    });

    usb.route.write(|w| w.phypen().set_bit());

    usb.pcgcctl.modify(|_, w| {
        w.stoppclk()
            .clear_bit()
            .pwrclmp()
            .clear_bit()
            .rstpdwnmodule()
            .clear_bit()
    });

    usb.grstctl.modify(|_, w| w.csftrst().set_bit());
    while usb.grstctl.read().csftrst().bit_is_set() {}
    while usb.grstctl.read().ahbidle().bit_is_clear() {}

    usb.dcfg.modify(|_, w| {
        w.devspd()
            .fs()
            .nzstsouthshk()
            .set_bit()
            .perfrint()
            ._80pcnt()
    });

    usb.gahbcfg
        .modify(|_, w| w.hbstlen().single().dmaen().set_bit());

    usb.dctl.modify(|_, w| {
        w.cgoutnak()
            .clear_bit()
            .sgoutnak()
            .clear_bit()
            .cgnpinnak()
            .clear_bit()
            .sgnpinnak()
            .clear_bit()
            .ignrfrmnum()
            .set_bit()
    });

    const TOTAL_RX_FIFO_SIZE: u16 = 128;
    const EP_TX_FIFO_SIZE: u16 = 64;

    usb.grxfsiz
        .write(|w| unsafe { w.rxfdep().bits(TOTAL_RX_FIFO_SIZE) });

    usb.gnptxfsiz.write(|w| unsafe {
        w.nptxfstaddr()
            .bits(TOTAL_RX_FIFO_SIZE)
            .nptxfineptxf0dep()
            .bits(EP_TX_FIFO_SIZE)
    });

    usb.dctl.modify(|_, w| {
        w.cgoutnak()
            .clear_bit()
            .sgoutnak()
            .clear_bit()
            .cgnpinnak()
            .clear_bit()
            .sgnpinnak()
            .clear_bit()
            .sftdiscon()
            .clear_bit()
    });

    const DEVADDR0: u8 = 0;
    usb.dcfg
        .modify(|_, w| unsafe { w.devaddr().bits(DEVADDR0) });

    usb.gahbcfg.modify(|_, w| w.glblintrmsk().set_bit());
    usb.gintmsk.write(|w| {
        w.usbrstmsk()
            .set_bit()
            .enumdonemsk()
            .set_bit()
            .iepintmsk()
            .set_bit()
            .oepintmsk()
            .set_bit()
    });
    usb.daintmsk
        .write(|w| w.inepmsk0().set_bit().outepmsk0().set_bit());
    usb.doepmsk.write(|w| {
        w.setupmsk()
            .set_bit()
            .xfercomplmsk()
            .set_bit()
            .stsphsercvdmsk()
            .set_bit()
    });
    usb.diepmsk.write(|w| w.xfercomplmsk().set_bit());
    usb.doep0_ctl.write(|w| {
        w.setd0pidef()
            .set_bit()
            .usbactep()
            .set_bit()
            .snak()
            .set_bit()
            .eptype()
            .control()
    });
    usb.diep0_ctl.write(|w| {
        w.setd0pidef()
            .set_bit()
            .usbactep()
            .set_bit()
            .snak()
            .set_bit()
            .eptype()
            .control()
    });
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
    init_clock(&ep.CMU);
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
    }
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_ef: &ExceptionFrame) -> ! {
    loop {}
}

interrupt!(RTC, rtc_handler);
fn rtc_handler() {
    let rtc = unsafe { &*efm32hg309f64::RTC::ptr() };

    rtc.ifc
        .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());

//    gpio.pa_douttgl.write(|w| unsafe { w.douttgl().bits(1) });
}

static mut counter: u32 = 0;

enum ControlState {
    WaitSetup,
    InData,
    OutData,
    LastInData,
    WaitStatusIn,
    WaitStatusOut,
    Stalled,
}
static mut USB_STATE: ControlState = ControlState::WaitSetup;

interrupt!(USB, usb_handler);
fn usb_handler() {
    let gpio = unsafe { &*efm32hg309f64::GPIO::ptr() };
    let usb: &efm32hg309f64::usb::RegisterBlock = unsafe { &*efm32hg309f64::USB::ptr() };

    let intsts = usb.gintsts.read();

    if intsts.usbrst().bit_is_set() {
        usb.gintsts.write(|w| w.usbrst().set_bit());

        const DEVADDR0: u8 = 0;
        usb.dcfg
            .modify(|_, w| unsafe { w.devaddr().bits(DEVADDR0) });
    }

    if intsts.enumdone().bit_is_set() {
        usb.gintsts.write(|w| w.enumdone().set_bit());
        unsafe {
            USB_STATE = ControlState::WaitSetup;
        }
    }
    // rtc.ifc
    //     .write(|w| w.comp1().set_bit().comp0().set_bit().of().set_bit());

    unsafe {
        counter += 1;
        if counter > 0x100000 {
            counter = 0;
            gpio.pa_douttgl.write(|w| w.douttgl().bits(1) );
        }
    }
}

exception!(*, default_handler);
fn default_handler(_irqn: i16) {
    loop {}
}
