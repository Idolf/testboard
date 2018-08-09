use core::marker::PhantomData;
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use cortex_m;
use cortex_m::interrupt::Nr;
use efm32hg309f64;

#[repr(C)]
pub struct InterruptHandler<F: ?Sized> {
    function: fn(&mut F),
    data: F,
}

impl<F> InterruptHandler<F> {
    pub fn new(f: F) -> InterruptHandler<F>
    where
        F: FnMut() + Send + Sync,
    {
        fn call<F: FnMut()>(f: &mut F) {
            f()
        }
        InterruptHandler {
            function: call::<F>,
            data: f,
        }
    }

    fn call(&mut self) {
        (self.function)(&mut self.data)
    }

    fn unify<'a>(&'a mut self) -> *mut InterruptHandler<()> {
        self as *mut InterruptHandler<F> as *mut InterruptHandler<()>
    }
}

pub struct Nvic {
    non_send_sync: PhantomData<*mut ()>,
}

pub struct NvicHandle<'a> {
    invariant_lifetime: PhantomData<&'a fn(&'a ()) -> &'a ()>,
    non_send_sync: PhantomData<*mut ()>,
}

unsafe fn enable_interrupt(interrupt: efm32hg309f64::Interrupt) {
    let mut nvic = mem::transmute::<(), efm32hg309f64::NVIC>(());
    nvic.enable(interrupt);
}

unsafe fn disable_interrupt(interrupt: efm32hg309f64::Interrupt) {
    let mut nvic = mem::transmute::<(), efm32hg309f64::NVIC>(());
    nvic.disable(interrupt);
}

const INTERRUPTS: [efm32hg309f64::Interrupt; 21] = [
    efm32hg309f64::Interrupt::DMA,
    efm32hg309f64::Interrupt::GPIO_EVEN,
    efm32hg309f64::Interrupt::TIMER0,
    efm32hg309f64::Interrupt::ACMP0,
    efm32hg309f64::Interrupt::ADC0,
    efm32hg309f64::Interrupt::I2C0,
    efm32hg309f64::Interrupt::GPIO_ODD,
    efm32hg309f64::Interrupt::TIMER1,
    efm32hg309f64::Interrupt::USART1_RX,
    efm32hg309f64::Interrupt::USART1_TX,
    efm32hg309f64::Interrupt::LEUART0,
    efm32hg309f64::Interrupt::PCNT0,
    efm32hg309f64::Interrupt::RTC,
    efm32hg309f64::Interrupt::CMU,
    efm32hg309f64::Interrupt::VCMP,
    efm32hg309f64::Interrupt::MSC,
    efm32hg309f64::Interrupt::AES,
    efm32hg309f64::Interrupt::USART0_RX,
    efm32hg309f64::Interrupt::USART0_TX,
    efm32hg309f64::Interrupt::USB,
    efm32hg309f64::Interrupt::TIMER2,
];

static INTERRUPT_HANDLERS: [AtomicPtr<InterruptHandler<()>>; 21] = [
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
    AtomicPtr::new(ptr::null_mut()),
];

impl<'a> NvicHandle<'a> {
    pub fn register<F>(&self, interrupt: efm32hg309f64::Interrupt, f: &'a mut InterruptHandler<F>)
    where
        F: FnMut() + Send + Sync + 'a,
    {
        assert_eq_size_val!(f, [0u8; 4], 0usize);

        let f: *mut InterruptHandler<()> = f.unify();

        let nr = interrupt.nr() as usize;
        assert!(nr <= 20);

        INTERRUPT_HANDLERS[nr].store(f, Ordering::Release);
        unsafe {
            enable_interrupt(interrupt);
        }
    }
}

impl Nvic {
    pub fn new(nvic: efm32hg309f64::NVIC) -> Nvic {
        let _ = nvic;
        Nvic {
            non_send_sync: PhantomData,
        }
    }

    pub fn with_handler<'nvic, F, R>(&'nvic mut self, f: F) -> R
    where
        R: 'static,
        F: 'nvic + FnOnce(NvicHandle<'nvic>) -> R,
    {
        let mut saved_handles: [*mut InterruptHandler<()>; 21] = [ptr::null_mut(); 21];

        for (saved, current) in saved_handles.iter_mut().zip(INTERRUPT_HANDLERS.iter()) {
            *saved = current.load(Ordering::Relaxed);
        }

        let _ = Cleanup { saved_handles };

        struct Cleanup {
            saved_handles: [*mut InterruptHandler<()>; 21],
        }

        impl Drop for Cleanup {
            fn drop(&mut self) {
                // We could in principle remove the interrupt handlers from the
                // table while they are still running, as long as we make sure
                // not to invalidate them (which can happen after we return from
                // this function). However it seems much safer to simply not do
                // that, especially since I do not want to consider panic
                // semantics.
                cortex_m::interrupt::free(|_| {
                    for ((&saved, current), &interrupt) in self
                        .saved_handles
                        .iter()
                        .zip(INTERRUPT_HANDLERS.iter())
                        .zip(INTERRUPTS.iter())
                    {
                        if saved.is_null() {
                            unsafe {
                                disable_interrupt(interrupt);
                            }
                        }
                        current.store(saved, Ordering::Release);
                    }
                })
            }
        }

        f(NvicHandle {
            invariant_lifetime: PhantomData,
            non_send_sync: PhantomData,
        })
    }
}

#[inline]
fn call_interrupt(interrupt: efm32hg309f64::Interrupt) {
    let nr = interrupt.nr() as usize;
    assert!(nr <= 21);
    let handler = INTERRUPT_HANDLERS[nr].load(Ordering::Acquire);
    if let Some(handler) = unsafe { handler.as_mut() } {
        handler.call();
    }
}

macro_rules! make_interrupt (
    ($interrupt:ident, $handler:ident) => {
        interrupt!($interrupt, $handler);
        fn $handler() {
            call_interrupt(efm32hg309f64::Interrupt::$interrupt);
        }
    }
);

make_interrupt!(DMA, handle_dma);
make_interrupt!(GPIO_EVEN, handle_gpio_even);
make_interrupt!(TIMER0, handle_timer0);
make_interrupt!(ACMP0, handle_acmp0);
make_interrupt!(ADC0, handle_adc0);
make_interrupt!(I2C0, handle_i2c0);
make_interrupt!(GPIO_ODD, handle_gpio_odd);
make_interrupt!(TIMER1, handle_timer1);
make_interrupt!(USART1_RX, handle_usart1_rx);
make_interrupt!(USART1_TX, handle_usart1_tx);
make_interrupt!(LEUART0, handle_leuart0);
make_interrupt!(PCNT0, handle_pcnt0);
make_interrupt!(RTC, handle_rtc);
make_interrupt!(CMU, handle_cmu);
make_interrupt!(VCMP, handle_vcmp);
make_interrupt!(MSC, handle_msc);
make_interrupt!(AES, handle_aes);
make_interrupt!(USART0_RX, handle_usart0_rx);
make_interrupt!(USART0_TX, handle_usart0_tx);
make_interrupt!(USB, handle_usb);
make_interrupt!(TIMER2, handle_timer2);
