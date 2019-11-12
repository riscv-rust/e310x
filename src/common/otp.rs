#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Programmed-I/O lock register"]
    pub lock: LOCK,
    #[doc = "0x04 - OTP device clock signal"]
    pub clock: CLOCK,
    #[doc = "0x08 - OTP device output-enable signal"]
    pub output_en: OUTPUT_EN,
    #[doc = "0x0c - OTP device chip-select signal"]
    pub select: SELECT,
    #[doc = "0x10 - OTP device write-enable signal"]
    pub write_en: WRITE_EN,
    #[doc = "0x14 - OTP device mode register"]
    pub mode: MODE,
    #[doc = "0x18 - OTP read-voltage regulator control"]
    pub mrr: MRR,
    #[doc = "0x1c - OTP write-voltage charge pump control"]
    pub mpp: MPP,
    #[doc = "0x20 - OTP read-voltage enable"]
    pub vrren: VRREN,
    #[doc = "0x24 - OTP write-voltage enable"]
    pub vppen: VPPEN,
    #[doc = "0x28 - OTP device address"]
    pub addr: ADDR,
    #[doc = "0x2c - OTP device data input"]
    pub data_in: DATA_IN,
    #[doc = "0x30 - OTP device data output"]
    pub data_out: DATA_OUT,
    #[doc = "0x34 - OTP read sequencer control"]
    pub rsctrl: RSCTRL,
}
#[doc = "Programmed-I/O lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Programmed-I/O lock register"]
pub mod lock;
#[doc = "OTP device clock signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "OTP device clock signal"]
pub mod clock;
#[doc = "OTP device output-enable signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [output_en](output_en) module"]
pub type OUTPUT_EN = crate::Reg<u32, _OUTPUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT_EN;
#[doc = "`read()` method returns [output_en::R](output_en::R) reader structure"]
impl crate::Readable for OUTPUT_EN {}
#[doc = "`write(|w| ..)` method takes [output_en::W](output_en::W) writer structure"]
impl crate::Writable for OUTPUT_EN {}
#[doc = "OTP device output-enable signal"]
pub mod output_en;
#[doc = "OTP device chip-select signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [select](select) module"]
pub type SELECT = crate::Reg<u32, _SELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELECT;
#[doc = "`read()` method returns [select::R](select::R) reader structure"]
impl crate::Readable for SELECT {}
#[doc = "`write(|w| ..)` method takes [select::W](select::W) writer structure"]
impl crate::Writable for SELECT {}
#[doc = "OTP device chip-select signal"]
pub mod select;
#[doc = "OTP device write-enable signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [write_en](write_en) module"]
pub type WRITE_EN = crate::Reg<u32, _WRITE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITE_EN;
#[doc = "`read()` method returns [write_en::R](write_en::R) reader structure"]
impl crate::Readable for WRITE_EN {}
#[doc = "`write(|w| ..)` method takes [write_en::W](write_en::W) writer structure"]
impl crate::Writable for WRITE_EN {}
#[doc = "OTP device write-enable signal"]
pub mod write_en;
#[doc = "OTP device mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "OTP device mode register"]
pub mod mode;
#[doc = "OTP read-voltage regulator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrr](mrr) module"]
pub type MRR = crate::Reg<u32, _MRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRR;
#[doc = "`read()` method returns [mrr::R](mrr::R) reader structure"]
impl crate::Readable for MRR {}
#[doc = "`write(|w| ..)` method takes [mrr::W](mrr::W) writer structure"]
impl crate::Writable for MRR {}
#[doc = "OTP read-voltage regulator control"]
pub mod mrr;
#[doc = "OTP write-voltage charge pump control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpp](mpp) module"]
pub type MPP = crate::Reg<u32, _MPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPP;
#[doc = "`read()` method returns [mpp::R](mpp::R) reader structure"]
impl crate::Readable for MPP {}
#[doc = "`write(|w| ..)` method takes [mpp::W](mpp::W) writer structure"]
impl crate::Writable for MPP {}
#[doc = "OTP write-voltage charge pump control"]
pub mod mpp;
#[doc = "OTP read-voltage enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vrren](vrren) module"]
pub type VRREN = crate::Reg<u32, _VRREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VRREN;
#[doc = "`read()` method returns [vrren::R](vrren::R) reader structure"]
impl crate::Readable for VRREN {}
#[doc = "`write(|w| ..)` method takes [vrren::W](vrren::W) writer structure"]
impl crate::Writable for VRREN {}
#[doc = "OTP read-voltage enable"]
pub mod vrren;
#[doc = "OTP write-voltage enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vppen](vppen) module"]
pub type VPPEN = crate::Reg<u32, _VPPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VPPEN;
#[doc = "`read()` method returns [vppen::R](vppen::R) reader structure"]
impl crate::Readable for VPPEN {}
#[doc = "`write(|w| ..)` method takes [vppen::W](vppen::W) writer structure"]
impl crate::Writable for VPPEN {}
#[doc = "OTP write-voltage enable"]
pub mod vppen;
#[doc = "OTP device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "OTP device address"]
pub mod addr;
#[doc = "OTP device data input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_in](data_in) module"]
pub type DATA_IN = crate::Reg<u32, _DATA_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN;
#[doc = "`read()` method returns [data_in::R](data_in::R) reader structure"]
impl crate::Readable for DATA_IN {}
#[doc = "`write(|w| ..)` method takes [data_in::W](data_in::W) writer structure"]
impl crate::Writable for DATA_IN {}
#[doc = "OTP device data input"]
pub mod data_in;
#[doc = "OTP device data output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_out](data_out) module"]
pub type DATA_OUT = crate::Reg<u32, _DATA_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT;
#[doc = "`read()` method returns [data_out::R](data_out::R) reader structure"]
impl crate::Readable for DATA_OUT {}
#[doc = "`write(|w| ..)` method takes [data_out::W](data_out::W) writer structure"]
impl crate::Writable for DATA_OUT {}
#[doc = "OTP device data output"]
pub mod data_out;
#[doc = "OTP read sequencer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rsctrl](rsctrl) module"]
pub type RSCTRL = crate::Reg<u32, _RSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSCTRL;
#[doc = "`read()` method returns [rsctrl::R](rsctrl::R) reader structure"]
impl crate::Readable for RSCTRL {}
#[doc = "`write(|w| ..)` method takes [rsctrl::W](rsctrl::W) writer structure"]
impl crate::Writable for RSCTRL {}
#[doc = "OTP read sequencer control"]
pub mod rsctrl;
