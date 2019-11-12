#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Prescale register lo-byte"]
    pub prer_lo: PRER_LO,
    #[doc = "0x04 - Clock Prescale register hi-byte"]
    pub prer_hi: PRER_HI,
    #[doc = "0x08 - Control register"]
    pub ctr: CTR,
    #[doc = "0x0c - Transmit register / Receive register"]
    pub txr_rxr: TXR_RXR,
    _reserved_4_cr: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub fn sr(&self) -> &SR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const SR) }
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub fn sr_mut(&self) -> &mut SR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut SR) }
    }
    #[doc = "0x10 - Command register"]
    #[inline(always)]
    pub fn cr(&self) -> &CR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const CR) }
    }
    #[doc = "0x10 - Command register"]
    #[inline(always)]
    pub fn cr_mut(&self) -> &mut CR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut CR) }
    }
    #[doc = "0x10 - Command register / Status register"]
    #[inline(always)]
    pub fn cr_sr(&self) -> &CR_SR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const CR_SR) }
    }
    #[doc = "0x10 - Command register / Status register"]
    #[inline(always)]
    pub fn cr_sr_mut(&self) -> &mut CR_SR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut CR_SR) }
    }
}
#[doc = "Clock Prescale register lo-byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prer_lo](prer_lo) module"]
pub type PRER_LO = crate::Reg<u32, _PRER_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER_LO;
#[doc = "`read()` method returns [prer_lo::R](prer_lo::R) reader structure"]
impl crate::Readable for PRER_LO {}
#[doc = "`write(|w| ..)` method takes [prer_lo::W](prer_lo::W) writer structure"]
impl crate::Writable for PRER_LO {}
#[doc = "Clock Prescale register lo-byte"]
pub mod prer_lo;
#[doc = "Clock Prescale register hi-byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prer_hi](prer_hi) module"]
pub type PRER_HI = crate::Reg<u32, _PRER_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER_HI;
#[doc = "`read()` method returns [prer_hi::R](prer_hi::R) reader structure"]
impl crate::Readable for PRER_HI {}
#[doc = "`write(|w| ..)` method takes [prer_hi::W](prer_hi::W) writer structure"]
impl crate::Writable for PRER_HI {}
#[doc = "Clock Prescale register hi-byte"]
pub mod prer_hi;
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "Control register"]
pub mod ctr;
#[doc = "Transmit register / Receive register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txr_rxr](txr_rxr) module"]
pub type TXR_RXR = crate::Reg<u32, _TXR_RXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXR_RXR;
#[doc = "`read()` method returns [txr_rxr::R](txr_rxr::R) reader structure"]
impl crate::Readable for TXR_RXR {}
#[doc = "`write(|w| ..)` method takes [txr_rxr::W](txr_rxr::W) writer structure"]
impl crate::Writable for TXR_RXR {}
#[doc = "Transmit register / Receive register"]
pub mod txr_rxr;
#[doc = "Command register / Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr_sr](cr_sr) module"]
pub type CR_SR = crate::Reg<u32, _CR_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_SR;
#[doc = "`read()` method returns [cr_sr::R](cr_sr::R) reader structure"]
impl crate::Readable for CR_SR {}
#[doc = "`write(|w| ..)` method takes [cr_sr::W](cr_sr::W) writer structure"]
impl crate::Writable for CR_SR {}
#[doc = "Command register / Status register"]
pub mod cr_sr;
#[doc = "Command register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Command register"]
pub mod cr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status register"]
pub mod sr;
