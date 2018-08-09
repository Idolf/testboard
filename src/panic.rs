use cmu;
use core::panic::PanicInfo;
use cortex_m;
use gpio;
use leuart;
use rt::ExceptionFrame;

#[panic_implementation]
#[no_mangle]
pub fn panic_impl(panic_info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    pub struct UnknownState;
    unsafe {
        let hfrco = cmu::hfrco::HfRco::<UnknownState>::claim_ownership().enable_21mhz();
        let hfclk = cmu::hfclk::HfClk::<UnknownState, UnknownState>::claim_ownership()
            .div1()
            .hfrco(&hfrco);
        let hfcoreclk =
            cmu::hfcoreclk::HfCoreClk::<UnknownState, UnknownState>::claim_ownership().div1(&hfclk);
        let hfcoreclklediv =
            cmu::hfcoreclkle::HfCoreClkLeDiv::<UnknownState, UnknownState>::claim_ownership()
                .enable_div4(&hfcoreclk);
        let lfb = cmu::lfb::LfbClk::<UnknownState>::claim_ownership()
            .enable_hfcoreclklediv(&hfcoreclklediv);
        let lfb_leuart = cmu::lfb::LfbClkLeuart0::<UnknownState, UnknownState>::claim_ownership()
            .enable_div8(&lfb);

        let mut pa0 = gpio::Pa0::<UnknownState>::claim_ownership()
            .mode(gpio::pin_modes::PinMode::new().push_pull().input_enable());

        let mut pb13 = gpio::Pb13::<UnknownState>::claim_ownership()
            .mode(gpio::pin_modes::PinMode::new().push_pull().input_enable());

        let mut leuart = leuart::Leuart::<UnknownState, UnknownState, UnknownState, UnknownState>::claim_ownership()
            .baudrate(&lfb_leuart, 115200.0)
            .location1()
            .enable_tx(&mut pb13);

        loop {
            use core::fmt::Write;
            use embedded_hal::digital::OutputPin;

            for _ in 0..4 {
                pa0.set_low();
                cortex_m::asm::delay(21_000_000 / 16);
                pa0.set_high();
                cortex_m::asm::delay(21_000_000 / 16);
            }
            write!(leuart, "\n===== PANIC! =====\n{}\n\n", panic_info);
            cortex_m::asm::delay(21_000_000 / 2);
        }
    }
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(ef: &ExceptionFrame) -> ! {
    panic!("\nHard fault!\n    r0:   0x{:08x}\n    r1:   0x{:08x}\n    r2:   0x{:08x}\n    r3:   0x{:08x}\n    r12:  0x{:08x}\n    lr:   0x{:08x}\n    pc:   0x{:08x}\n    xpsr: 0x{:08x}\n", ef.r0, ef.r1, ef.r2, ef.r3, ef.r12, ef.lr, ef.pc, ef.xpsr);
}
