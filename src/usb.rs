use efm32hg309f64;

pub fn init_usb(usb: &efm32hg309f64::usb::RegisterBlock) {
    usb.ctrl.write(|w| {
        w.lemoscctrl().gate()
         .lemidleen().set_bit()
         .lemphyctrl().set_bit()
    });

    usb.route.write(|w| w.phypen().set_bit());

    usb.pcgcctl.modify(|_, w| {
        w.stoppclk().clear_bit()
         .pwrclmp().clear_bit()
         .rstpdwnmodule().clear_bit()
    });

    usb.grstctl.modify(|_, w| w.csftrst().set_bit());
    while usb.grstctl.read().csftrst().bit_is_set() {}
    while usb.grstctl.read().ahbidle().bit_is_clear() {}

    usb.dcfg.modify(|_, w| {
        w.devspd().fs()
         .nzstsouthshk().set_bit()
         .perfrint()._80pcnt()
    });

    usb.gahbcfg
        .modify(|_, w| w.hbstlen().single().dmaen().set_bit());

    usb.dctl.modify(|_, w| {
        w.cgoutnak().clear_bit()
         .sgoutnak().clear_bit()
         .cgnpinnak().clear_bit()
         .sgnpinnak().clear_bit()
         .ignrfrmnum().set_bit()
    });

    const TOTAL_RX_FIFO_SIZE: u16 = 128;
    const EP_TX_FIFO_SIZE: u16 = 64;

    usb.grxfsiz.write(|w| unsafe { w.rxfdep().bits(TOTAL_RX_FIFO_SIZE) });

    usb.gnptxfsiz.write(|w| unsafe {
        w.nptxfstaddr().bits(TOTAL_RX_FIFO_SIZE)
         .nptxfineptxf0dep().bits(EP_TX_FIFO_SIZE)
    });

    usb.dctl.modify(|_, w| {
        w.cgoutnak().clear_bit()
         .sgoutnak().clear_bit()
         .cgnpinnak().clear_bit()
         .sgnpinnak().clear_bit()
         .sftdiscon().clear_bit()
    });

    usb.dcfg.modify(|_, w| unsafe { w.devaddr().bits(0) });

    usb.gahbcfg.modify(|_, w| w.glblintrmsk().set_bit());
    usb.gintmsk.write(|w| {
        w.usbrstmsk().set_bit()
         .enumdonemsk().set_bit()
         .iepintmsk().set_bit()
         .oepintmsk().set_bit()
    });
    usb.daintmsk.write(|w| w.inepmsk0().set_bit().outepmsk0().set_bit());
    usb.doepmsk.write(|w| {
        w.setupmsk().set_bit()
         .xfercomplmsk().set_bit()
         .stsphsercvdmsk().set_bit()
    });
    usb.diepmsk.write(|w| w.xfercomplmsk().set_bit());
    usb.doep0_ctl.write(|w| {
        w.setd0pidef().set_bit()
         .usbactep().set_bit()
         .snak().set_bit()
         .eptype().control()
    });
    usb.diep0_ctl.write(|w| {
        w.setd0pidef().set_bit()
         .usbactep().set_bit()
         .snak().set_bit()
         .eptype().control()
    });
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum ControlState {
    WaitSetup,
    InData,
    OutData,
    LastInData,
    WaitStatusIn,
    WaitStatusOut,
    Stalled,
}

#[allow(dead_code)]
static mut USB_STATE: ControlState = ControlState::WaitSetup;

static SETUP_PACKET : [u8; 20] = [0x41; 20];

fn prepare_ep0_setup(){
    ep0_prepare_out(&SETUP_PACKET, false);
}

fn ep0_prepare_out(buffer: &[u8], cnak : bool) {
    let usb: &efm32hg309f64::usb::RegisterBlock = unsafe { &*efm32hg309f64::USB::ptr() };
    usb.doep0dmaaddr.write(|w| unsafe{ w.bits(buffer.as_ptr() as u32) });
    usb.doep0tsiz.write(|w| unsafe {
        w.xfersize().bits(buffer.len() as u8).pktcnt().bit(true)
    });
    usb.doep0ctl.write(|w| {
        w.epena().set_bit();
        if cnak { w.cnak().set_bit(); }
        w
    });
}

fn ep0_out_stall() {}
fn ep0_in_stall() {}
fn handle_datastage_in0() {}


fn handle_in0(){
    match unsafe { USB_STATE } {
        ControlState::InData => {
            handle_datastage_in0();
        }
        ControlState::WaitStatusIn => {
            prepare_ep0_setup();
            unsafe{ USB_STATE = ControlState::WaitSetup; }
        }
        _ => {
            unsafe{ USB_STATE = ControlState::Stalled; }
            ep0_out_stall();
            ep0_in_stall();
            prepare_ep0_setup();
            unsafe{ USB_STATE = ControlState::WaitSetup; }
        }
    }
}

interrupt!(USB, usb_handler);
fn usb_handler() {
    let usb: &efm32hg309f64::usb::RegisterBlock = unsafe { &*efm32hg309f64::USB::ptr() };

    let intsts = usb.gintsts.read();

    if intsts.usbrst().bit_is_set() {
        usb.gintsts.write(|w| w.usbrst().set_bit());
        usb.dcfg.modify(|_, w| unsafe { w.devaddr().bits(0) });
    }

    if intsts.enumdone().bit_is_set() {
        usb.gintsts.write(|w| w.enumdone().set_bit());
        prepare_ep0_setup();
        unsafe { USB_STATE = ControlState::WaitSetup; }
    }

    if intsts.iepint().bit_is_set() {
        if usb.diep0int.read().xfercompl().bit_is_set() {
            usb.diep0int.write(|w| w.xfercompl().set_bit() );
            handle_in0();
        }
//        usb.gintsts.write(|w| w.iepint().set_bit());
    }
}
