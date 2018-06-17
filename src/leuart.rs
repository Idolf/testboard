use cortex_m;
use efm32hg309f64;

use heapless::consts::*;
use heapless::RingBuffer;
use gpio::*;

pub struct Leuart { }

static mut LEUART_RXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();
static mut LEUART_TXBUF: RingBuffer<u8, U256, u8> = RingBuffer::u8();

impl Leuart {
    pub fn location0(tx: Option<PB13>, rx: Option<PB14>) -> Leuart {
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };

        regs.route.write(|w| {
            // Use location 1 for the route
            // Location 1 TX==PB13, RX==PB14. See EFM32HF309 Datasheet section 4.2
            w.location().loc1()
        });

        let leuart = Leuart {};

        tx.map(|mut pin| { 
            pin.mode(PinMode::PushPull);
            regs.route.modify(|_, w| w.txpen().set_bit() );
            regs.cmd.write(|w| w.txen().set_bit() );
        });

        rx.map(|mut pin| { 
            pin.mode(PinMode::Input);
            regs.route.modify(|_, w| w.rxpen().set_bit() );
            regs.cmd.write(|w| w.rxen().set_bit() );
        });

        regs.cmd.write(|w| {
            // Clear rx and tx buffers
            w.clearrx().set_bit().cleartx().set_bit()
        });
        
        // two stopbits
        regs.ctrl.modify(|_, w| w.stopbits().set_bit() );

        // Enable interrupts on received data
        regs.ien.write(|w| w.rxdatav().set_bit());

        leuart
    }

    pub fn baud_rate(&self, baud_rate : f32) {
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

        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };

        regs.clkdiv.write(|w| unsafe {
            w.div().bits(scale_factor)
        });

    }

    pub fn write(&self, buf : &[u8]){
        let mut tx_producer = unsafe { LEUART_TXBUF.split().0 };
        let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };

        for byte in buf {
            while tx_producer.enqueue(*byte).is_err() {
                cortex_m::asm::wfe()
            }
        }
        cortex_m::interrupt::free(|_| {
            regs.ien.modify(|_, w| w.txbl().set_bit());
            regs.cmd.write(|w| w.txen().set_bit());
        });
    }

    pub fn read(&self, buf : &mut [u8]){
        let mut rx_consumer = unsafe { LEUART_RXBUF.split().1 };
        for ptr in buf.iter_mut() {
            loop {
                match rx_consumer.dequeue() {
                    Some(byte) => {*ptr = byte; break }
                    None => cortex_m::asm::wfi(),
                }
            }
        }
    }
}


interrupt!(LEUART0, leuart0_handler);
fn leuart0_handler() {
    let regs = unsafe { &*efm32hg309f64::LEUART0::ptr() };
    let leuart_if: efm32hg309f64::leuart0::if_::R = regs.if_.read();

    if leuart_if.txbl().bit_is_set() {
        let mut tx_consumer = unsafe { LEUART_TXBUF.split().1 };
        match tx_consumer.dequeue() {
            Some(byte) => regs.txdata.write(|w| unsafe { w.txdata().bits(byte) }),
            None => regs.ien.modify(|_, w| w.txbl().clear_bit()),
        }
    }

    if leuart_if.rxdatav().bit_is_set() {
        let mut rx_producer = unsafe { LEUART_RXBUF.split().0 };
        let byte = regs.rxdata.read().rxdata().bits();
        rx_producer.enqueue(byte).ok();
        cortex_m::asm::sev();
    }
}

