use efm32hg309f64;

pub enum Port {
    A, B, C, D, E, F
}

pub enum PinMode {
    Disabled = 0,
    Input = 1,
    InputPull = 2,
    InputPullFilter = 3,
    PushPull = 4,
    PushPullDrive = 5,
    Wiredor = 6,
    WiredOrPullDown = 7,
    WiredAnd = 8,
    WiredAndFilter = 9,
    WiredAndPullUp = 10,
    WiredAndPullUpFilter = 11,
    WiredAndDrive = 12,
    WiredAndDriveFilter = 13,
    WiredAndDrivePullUp = 14,
    WiredAndDrivePullUpFilter = 15,
}

pub struct Gpio {}

pub trait Pin : Sized {
//    const PORT : Port;
//    const PIN : u8;

    fn pin(&self) -> u8;
    fn port(&self) -> Port;
    
    fn mode(&mut self, mode : PinMode){
        Gpio{}.pin_mode(self.port(), self.pin(), mode)
    }
    fn set(&mut self){
        Gpio{}.pin_set(self.port(), self.pin())
    }
    fn clr(&mut self){
        Gpio{}.pin_clr(self.port(), self.pin())
    }
    fn tgl(&mut self){
        Gpio{}.pin_tgl(self.port(), self.pin())
    }
    fn din(&mut self) -> bool {
        Gpio{}.pin_din(self.port(), self.pin())
    }
    fn dout(&mut self, set:bool){
        if set { self.set() } else { self.clr() }
    }
}


pub struct PA0 { }
impl Pin for PA0 {
    fn port(&self) -> Port { Port::A }
    fn pin(&self)-> u8 { 0 }
}

pub struct PB7 { }
impl Pin for PB7 {
    fn port(&self) -> Port { Port::B }
    fn pin(&self)-> u8 { 7 }
}

pub struct PB8 { }
impl Pin for PB8 {
    fn port(&self) -> Port { Port::B }
    fn pin(&self)-> u8 { 8 }
}
pub struct PB11 { }
impl Pin for PB11 {
    fn port(&self) -> Port { Port::B }
    fn pin(&self)-> u8 { 11 }
}

pub struct PB13 { }
impl Pin for PB13 {
    fn port(&self) -> Port { Port::B }
    fn pin(&self)-> u8 { 13 }
}

pub struct PB14 { }
impl Pin for PB14 {
    fn port(&self) -> Port { Port::B }
    fn pin(&self)-> u8 { 14 }
}

pub struct PC0 {}
impl Pin for PC0 {
    fn port(&self) -> Port { Port::C }
    fn pin(&self)-> u8 { 0 }
}

pub struct PC1 {}
impl Pin for PC1 {
    fn port(&self) -> Port { Port::C }
    fn pin(&self)-> u8 { 1 }
}

pub struct PE12 {}
impl Pin for PE12 {
    fn port(&self) -> Port { Port::E }
    fn pin(&self)-> u8 { 12 }
}

pub struct PE13 {}
impl Pin for PE13 {
    fn port(&self) -> Port { Port::E }
    fn pin(&self)-> u8 { 13 }
}

pub struct PF0 {}
impl Pin for PF0 {
    fn port(&self) -> Port { Port::F }
    fn pin(&self)-> u8 { 0 }
}

pub struct PF1 {}
impl Pin for PF1 {
    fn port(&self) -> Port { Port::F }
    fn pin(&self)-> u8 { 1 }
}

pub struct PF2 {}
impl Pin for PF2 {
    fn port(&self) -> Port { Port::F }
    fn pin(&self)-> u8 { 2 }
}

pub struct Pins {
    pub pa0 : PA0,
    pub pb7 : PB7,
    pub pb8 : PB8,
    pub pb11 : PB11,
    pub pb13 : PB13,
    pub pb14 : PB14,
    pub pc0 : PC0,
    pub pc1 : PC1,
    pub pe12 : PE12,
    pub pe13 : PE13,
    pub pf0 : PF0,
    pub pf1 : PF1,
    pub pf2 : PF2,
}

impl Gpio {
    pub fn init_gpio() -> Gpio {
        Gpio { }
    }

    pub fn pins(&self) -> Pins {
        Pins {
            pa0: PA0 { },
            pb7: PB7 { },
            pb8: PB8 { },
            pb11: PB11 { },
            pb13: PB13 { },
            pb14: PB14 { },
            pc0: PC0 { },
            pc1: PC1 { },
            pe12: PE12 {},
            pe13: PE13 {},
            pf0: PF0 { },
            pf1: PF1 { },
            pf2: PF2 { },
        }
    }

    fn pin_mode(&self, port : Port, pin : u8, mode : PinMode){
        use self::Port::*;
        let regs = unsafe { &*efm32hg309f64::GPIO::ptr() };
        if pin < 8 {
            let mask = !(0xf << (pin * 4));
            let mode = (mode as u32) << (pin * 4);
            match port {
                A => regs.pa_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                B => regs.pb_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                C => regs.pc_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                D => regs.pd_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                E => regs.pe_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                F => regs.pf_model.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
            }
        } else {
            let pin = pin - 8;
            let mask = !(0xf << (pin * 4));
            let mode = (mode as u32) << (pin * 4);
            match port {
                A => regs.pa_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                B => regs.pb_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                C => regs.pc_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                D => regs.pd_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                E => regs.pe_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
                F => regs.pf_modeh.modify(|r, w| unsafe {
                    w.bits((r.bits() & mask) | mode)
                }),
            }
        }
    }

    fn pin_set(&self, port : Port, pin : u8){
        let regs = unsafe { &*efm32hg309f64::GPIO::ptr() };
        let pin = 1 << pin;
        use self::Port::*;
        match port {
            A => regs.pa_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
            B => regs.pb_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
            C => regs.pc_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
            D => regs.pd_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
            E => regs.pe_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
            F => regs.pf_doutset.write(|w| unsafe {w.doutset().bits(pin)}),
        }
    }

    fn pin_clr(&self, port : Port, pin : u8){
        let regs = unsafe { &*efm32hg309f64::GPIO::ptr() };
        let pin = 1 << pin;
        use self::Port::*;
        match port {
            A => regs.pa_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
            B => regs.pb_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
            C => regs.pc_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
            D => regs.pd_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
            E => regs.pe_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
            F => regs.pf_doutclr.write(|w| unsafe {w.doutclr().bits(pin)}),
        }
    }
    fn pin_tgl(&self, port : Port, pin : u8){
        let regs = unsafe { &*efm32hg309f64::GPIO::ptr() };
        let pin = 1 << pin;
        use self::Port::*;
        match port {
            A => regs.pa_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
            B => regs.pb_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
            C => regs.pc_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
            D => regs.pd_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
            E => regs.pe_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
            F => regs.pf_douttgl.write(|w| unsafe {w.douttgl().bits(pin)}),
        }
    }
    fn pin_din(&self, port : Port, pin : u8) -> bool {
        let regs = unsafe { &*efm32hg309f64::GPIO::ptr() };
        use self::Port::*;
        match port {
            A => (regs.pa_din.read().bits() >> pin) == 1,
            B => (regs.pb_din.read().bits() >> pin) == 1,
            C => (regs.pc_din.read().bits() >> pin) == 1,
            D => (regs.pd_din.read().bits() >> pin) == 1,
            E => (regs.pe_din.read().bits() >> pin) == 1,
            F => (regs.pf_din.read().bits() >> pin) == 1
        }
    }
} 
