const USHFRCO_COARSECAL_BAND_24: *const u8 = 0x0fe0_81cc as *const u8;
const USHFRCO_FINECAL_BAND_24: *const u8 = 0x0fe0_81cd as *const u8;
const USHFRCO_COARSECAL_BAND_48: *const u8 = 0x0fe0_81ce as *const u8;
const USHFRCO_FINECAL_BAND_48: *const u8 = 0x0fe0_81cf as *const u8;

const AUXHFRCO_CALIB_BAND_1: *const u8 = 0x0fe0_81d4 as *const u8;
const AUXHFRCO_CALIB_BAND_7: *const u8 = 0x0fe0_81d5 as *const u8;
const AUXHFRCO_CALIB_BAND_11: *const u8 = 0x0fe0_81d6 as *const u8;
const AUXHFRCO_CALIB_BAND_14: *const u8 = 0x0fe0_81d7 as *const u8;
const AUXHFRCO_CALIB_BAND_21: *const u8 = 0x0fe0_81d8 as *const u8;

const HFRCO_CALIB_BAND_1: *const u8 = 0x0fe0_81dc as *const u8;
const HFRCO_CALIB_BAND_7: *const u8 = 0x0fe0_81dd as *const u8;
const HFRCO_CALIB_BAND_11: *const u8 = 0x0fe0_81de as *const u8;
const HFRCO_CALIB_BAND_14: *const u8 = 0x0fe0_81df as *const u8;
const HFRCO_CALIB_BAND_21: *const u8 = 0x0fe0_81e0 as *const u8;

#[inline]
pub fn get_hfrco_calib_band_1() -> u8 {
    unsafe { *HFRCO_CALIB_BAND_1 }
}

#[inline]
pub fn get_hfrco_calib_band_7() -> u8 {
    unsafe { *HFRCO_CALIB_BAND_7 }
}

#[inline]
pub fn get_hfrco_calib_band_11() -> u8 {
    unsafe { *HFRCO_CALIB_BAND_11 }
}

#[inline]
pub fn get_hfrco_calib_band_14() -> u8 {
    unsafe { *HFRCO_CALIB_BAND_14 }
}

#[inline]
pub fn get_hfrco_calib_band_21() -> u8 {
    unsafe { *HFRCO_CALIB_BAND_21 }
}

#[inline]
pub fn get_auxhfrco_calib_band_1() -> u8 {
    unsafe { *AUXHFRCO_CALIB_BAND_1 }
}

#[inline]
pub fn get_auxhfrco_calib_band_7() -> u8 {
    unsafe { *AUXHFRCO_CALIB_BAND_7 }
}

#[inline]
pub fn get_auxhfrco_calib_band_11() -> u8 {
    unsafe { *AUXHFRCO_CALIB_BAND_11 }
}

#[inline]
pub fn get_auxhfrco_calib_band_14() -> u8 {
    unsafe { *AUXHFRCO_CALIB_BAND_14 }
}

#[inline]
pub fn get_auxhfrco_calib_band_21() -> u8 {
    unsafe { *AUXHFRCO_CALIB_BAND_21 }
}

#[inline]
pub fn get_ushfrco_calib_band_24() -> (u8, u8) {
    unsafe {
        (
            (*USHFRCO_COARSECAL_BAND_24) & 0x7f,
            (*USHFRCO_FINECAL_BAND_24) & 0x3f,
        )
    }
}

#[inline]
pub fn get_ushfrco_calib_band_48() -> (u8, u8) {
    unsafe {
        (
            (*USHFRCO_COARSECAL_BAND_48) & 0x7f,
            (*USHFRCO_FINECAL_BAND_48) & 0x3f,
        )
    }
}
