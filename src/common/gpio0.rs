#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin value."]
    pub input_val: INPUT_VAL,
    #[doc = "0x04 - Pin Input Enable Register"]
    pub input_en: INPUT_EN,
    #[doc = "0x08 - Pin Output Enable Register"]
    pub output_en: OUTPUT_EN,
    #[doc = "0x0c - Output Port Value Register"]
    pub output_val: OUTPUT_VAL,
    #[doc = "0x10 - Internal Pull-Up Enable Register"]
    pub pullup: PULLUP,
    #[doc = "0x14 - Drive Strength Register"]
    pub drive: DRIVE,
    #[doc = "0x18 - Rise Interrupt Enable Register"]
    pub rise_ie: RISE_IE,
    #[doc = "0x1c - Rise Interrupt Pending Register"]
    pub rise_ip: RISE_IP,
    #[doc = "0x20 - Fall Interrupt Enable Register"]
    pub fall_ie: FALL_IE,
    #[doc = "0x24 - Fall Interrupt Pending Register"]
    pub fall_ip: FALL_IP,
    #[doc = "0x28 - High Interrupt Enable Register"]
    pub high_ie: HIGH_IE,
    #[doc = "0x2c - High Interrupt Pending Register"]
    pub high_ip: HIGH_IP,
    #[doc = "0x30 - Low Interrupt Enable Register"]
    pub low_ie: LOW_IE,
    #[doc = "0x34 - Low Interrupt Pending Register"]
    pub low_ip: LOW_IP,
    #[doc = "0x38 - HW I/O Function Enable Register"]
    pub iof_en: IOF_EN,
    #[doc = "0x3c - HW I/O Function Select Register"]
    pub iof_sel: IOF_SEL,
    #[doc = "0x40 - Output XOR (invert) Register"]
    pub out_xor: OUT_XOR,
}
#[doc = "Pin value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [input_val](input_val) module"]
pub type INPUT_VAL = crate::Reg<u32, _INPUT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT_VAL;
#[doc = "`read()` method returns [input_val::R](input_val::R) reader structure"]
impl crate::Readable for INPUT_VAL {}
#[doc = "`write(|w| ..)` method takes [input_val::W](input_val::W) writer structure"]
impl crate::Writable for INPUT_VAL {}
#[doc = "Pin value."]
pub mod input_val;
#[doc = "Pin Input Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [input_en](input_en) module"]
pub type INPUT_EN = crate::Reg<u32, _INPUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT_EN;
#[doc = "`read()` method returns [input_en::R](input_en::R) reader structure"]
impl crate::Readable for INPUT_EN {}
#[doc = "`write(|w| ..)` method takes [input_en::W](input_en::W) writer structure"]
impl crate::Writable for INPUT_EN {}
#[doc = "Pin Input Enable Register"]
pub mod input_en;
#[doc = "Pin Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [output_en](output_en) module"]
pub type OUTPUT_EN = crate::Reg<u32, _OUTPUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT_EN;
#[doc = "`read()` method returns [output_en::R](output_en::R) reader structure"]
impl crate::Readable for OUTPUT_EN {}
#[doc = "`write(|w| ..)` method takes [output_en::W](output_en::W) writer structure"]
impl crate::Writable for OUTPUT_EN {}
#[doc = "Pin Output Enable Register"]
pub mod output_en;
#[doc = "Output Port Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [output_val](output_val) module"]
pub type OUTPUT_VAL = crate::Reg<u32, _OUTPUT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT_VAL;
#[doc = "`read()` method returns [output_val::R](output_val::R) reader structure"]
impl crate::Readable for OUTPUT_VAL {}
#[doc = "`write(|w| ..)` method takes [output_val::W](output_val::W) writer structure"]
impl crate::Writable for OUTPUT_VAL {}
#[doc = "Output Port Value Register"]
pub mod output_val;
#[doc = "Internal Pull-Up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pullup](pullup) module"]
pub type PULLUP = crate::Reg<u32, _PULLUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULLUP;
#[doc = "`read()` method returns [pullup::R](pullup::R) reader structure"]
impl crate::Readable for PULLUP {}
#[doc = "`write(|w| ..)` method takes [pullup::W](pullup::W) writer structure"]
impl crate::Writable for PULLUP {}
#[doc = "Internal Pull-Up Enable Register"]
pub mod pullup;
#[doc = "Drive Strength Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [drive](drive) module"]
pub type DRIVE = crate::Reg<u32, _DRIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRIVE;
#[doc = "`read()` method returns [drive::R](drive::R) reader structure"]
impl crate::Readable for DRIVE {}
#[doc = "`write(|w| ..)` method takes [drive::W](drive::W) writer structure"]
impl crate::Writable for DRIVE {}
#[doc = "Drive Strength Register"]
pub mod drive;
#[doc = "Rise Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rise_ie](rise_ie) module"]
pub type RISE_IE = crate::Reg<u32, _RISE_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISE_IE;
#[doc = "`read()` method returns [rise_ie::R](rise_ie::R) reader structure"]
impl crate::Readable for RISE_IE {}
#[doc = "`write(|w| ..)` method takes [rise_ie::W](rise_ie::W) writer structure"]
impl crate::Writable for RISE_IE {}
#[doc = "Rise Interrupt Enable Register"]
pub mod rise_ie;
#[doc = "Rise Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rise_ip](rise_ip) module"]
pub type RISE_IP = crate::Reg<u32, _RISE_IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISE_IP;
#[doc = "`read()` method returns [rise_ip::R](rise_ip::R) reader structure"]
impl crate::Readable for RISE_IP {}
#[doc = "`write(|w| ..)` method takes [rise_ip::W](rise_ip::W) writer structure"]
impl crate::Writable for RISE_IP {}
#[doc = "Rise Interrupt Pending Register"]
pub mod rise_ip;
#[doc = "Fall Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fall_ie](fall_ie) module"]
pub type FALL_IE = crate::Reg<u32, _FALL_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FALL_IE;
#[doc = "`read()` method returns [fall_ie::R](fall_ie::R) reader structure"]
impl crate::Readable for FALL_IE {}
#[doc = "`write(|w| ..)` method takes [fall_ie::W](fall_ie::W) writer structure"]
impl crate::Writable for FALL_IE {}
#[doc = "Fall Interrupt Enable Register"]
pub mod fall_ie;
#[doc = "Fall Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fall_ip](fall_ip) module"]
pub type FALL_IP = crate::Reg<u32, _FALL_IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FALL_IP;
#[doc = "`read()` method returns [fall_ip::R](fall_ip::R) reader structure"]
impl crate::Readable for FALL_IP {}
#[doc = "`write(|w| ..)` method takes [fall_ip::W](fall_ip::W) writer structure"]
impl crate::Writable for FALL_IP {}
#[doc = "Fall Interrupt Pending Register"]
pub mod fall_ip;
#[doc = "High Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [high_ie](high_ie) module"]
pub type HIGH_IE = crate::Reg<u32, _HIGH_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIGH_IE;
#[doc = "`read()` method returns [high_ie::R](high_ie::R) reader structure"]
impl crate::Readable for HIGH_IE {}
#[doc = "`write(|w| ..)` method takes [high_ie::W](high_ie::W) writer structure"]
impl crate::Writable for HIGH_IE {}
#[doc = "High Interrupt Enable Register"]
pub mod high_ie;
#[doc = "High Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [high_ip](high_ip) module"]
pub type HIGH_IP = crate::Reg<u32, _HIGH_IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIGH_IP;
#[doc = "`read()` method returns [high_ip::R](high_ip::R) reader structure"]
impl crate::Readable for HIGH_IP {}
#[doc = "`write(|w| ..)` method takes [high_ip::W](high_ip::W) writer structure"]
impl crate::Writable for HIGH_IP {}
#[doc = "High Interrupt Pending Register"]
pub mod high_ip;
#[doc = "Low Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [low_ie](low_ie) module"]
pub type LOW_IE = crate::Reg<u32, _LOW_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOW_IE;
#[doc = "`read()` method returns [low_ie::R](low_ie::R) reader structure"]
impl crate::Readable for LOW_IE {}
#[doc = "`write(|w| ..)` method takes [low_ie::W](low_ie::W) writer structure"]
impl crate::Writable for LOW_IE {}
#[doc = "Low Interrupt Enable Register"]
pub mod low_ie;
#[doc = "Low Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [low_ip](low_ip) module"]
pub type LOW_IP = crate::Reg<u32, _LOW_IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOW_IP;
#[doc = "`read()` method returns [low_ip::R](low_ip::R) reader structure"]
impl crate::Readable for LOW_IP {}
#[doc = "`write(|w| ..)` method takes [low_ip::W](low_ip::W) writer structure"]
impl crate::Writable for LOW_IP {}
#[doc = "Low Interrupt Pending Register"]
pub mod low_ip;
#[doc = "HW I/O Function Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iof_en](iof_en) module"]
pub type IOF_EN = crate::Reg<u32, _IOF_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOF_EN;
#[doc = "`read()` method returns [iof_en::R](iof_en::R) reader structure"]
impl crate::Readable for IOF_EN {}
#[doc = "`write(|w| ..)` method takes [iof_en::W](iof_en::W) writer structure"]
impl crate::Writable for IOF_EN {}
#[doc = "HW I/O Function Enable Register"]
pub mod iof_en;
#[doc = "HW I/O Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iof_sel](iof_sel) module"]
pub type IOF_SEL = crate::Reg<u32, _IOF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOF_SEL;
#[doc = "`read()` method returns [iof_sel::R](iof_sel::R) reader structure"]
impl crate::Readable for IOF_SEL {}
#[doc = "`write(|w| ..)` method takes [iof_sel::W](iof_sel::W) writer structure"]
impl crate::Writable for IOF_SEL {}
#[doc = "HW I/O Function Select Register"]
pub mod iof_sel;
#[doc = "Output XOR (invert) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_xor](out_xor) module"]
pub type OUT_XOR = crate::Reg<u32, _OUT_XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_XOR;
#[doc = "`read()` method returns [out_xor::R](out_xor::R) reader structure"]
impl crate::Readable for OUT_XOR {}
#[doc = "`write(|w| ..)` method takes [out_xor::W](out_xor::W) writer structure"]
impl crate::Writable for OUT_XOR {}
#[doc = "Output XOR (invert) Register"]
pub mod out_xor;
