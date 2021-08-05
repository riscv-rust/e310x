#[doc = "Register `cfg` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cfg` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmp3ip` reader - "]
pub struct CMP3IP_R(crate::FieldReader<bool, bool>);
impl CMP3IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp3ip` writer - "]
pub struct CMP3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `cmp2ip` reader - "]
pub struct CMP2IP_R(crate::FieldReader<bool, bool>);
impl CMP2IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp2ip` writer - "]
pub struct CMP2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `cmp1ip` reader - "]
pub struct CMP1IP_R(crate::FieldReader<bool, bool>);
impl CMP1IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp1ip` writer - "]
pub struct CMP1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `cmp0ip` reader - "]
pub struct CMP0IP_R(crate::FieldReader<bool, bool>);
impl CMP0IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp0ip` writer - "]
pub struct CMP0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0IP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `cmp3gang` reader - "]
pub struct CMP3GANG_R(crate::FieldReader<bool, bool>);
impl CMP3GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp3gang` writer - "]
pub struct CMP3GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3GANG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `cmp2gang` reader - "]
pub struct CMP2GANG_R(crate::FieldReader<u16, u16>);
impl CMP2GANG_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMP2GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2GANG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp2gang` writer - "]
pub struct CMP2GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2GANG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 26)) | ((value as u32 & 0x07ff) << 26);
        self.w
    }
}
#[doc = "Field `cmp1gang` reader - "]
pub struct CMP1GANG_R(crate::FieldReader<bool, bool>);
impl CMP1GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp1gang` writer - "]
pub struct CMP1GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1GANG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `cmp0gang` reader - "]
pub struct CMP0GANG_R(crate::FieldReader<bool, bool>);
impl CMP0GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp0gang` writer - "]
pub struct CMP0GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0GANG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `cmp3center` reader - "]
pub struct CMP3CENTER_R(crate::FieldReader<bool, bool>);
impl CMP3CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp3center` writer - "]
pub struct CMP3CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3CENTER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `cmp2center` reader - "]
pub struct CMP2CENTER_R(crate::FieldReader<bool, bool>);
impl CMP2CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp2center` writer - "]
pub struct CMP2CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2CENTER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cmp1center` reader - "]
pub struct CMP1CENTER_R(crate::FieldReader<bool, bool>);
impl CMP1CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp1center` writer - "]
pub struct CMP1CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1CENTER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `cmp0center` reader - "]
pub struct CMP0CENTER_R(crate::FieldReader<bool, bool>);
impl CMP0CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmp0center` writer - "]
pub struct CMP0CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0CENTER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `enoneshot` reader - "]
pub struct ENONESHOT_R(crate::FieldReader<bool, bool>);
impl ENONESHOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENONESHOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENONESHOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enoneshot` writer - "]
pub struct ENONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENONESHOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `enalways` reader - "]
pub struct ENALWAYS_R(crate::FieldReader<bool, bool>);
impl ENALWAYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENALWAYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENALWAYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enalways` writer - "]
pub struct ENALWAYS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENALWAYS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `deglitch` reader - "]
pub struct DEGLITCH_R(crate::FieldReader<bool, bool>);
impl DEGLITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEGLITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEGLITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deglitch` writer - "]
pub struct DEGLITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `zerocmp` reader - "]
pub struct ZEROCMP_R(crate::FieldReader<bool, bool>);
impl ZEROCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZEROCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZEROCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `zerocmp` writer - "]
pub struct ZEROCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROCMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `sticky` reader - "]
pub struct STICKY_R(crate::FieldReader<bool, bool>);
impl STICKY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STICKY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STICKY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sticky` writer - "]
pub struct STICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> STICKY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `scale` reader - "]
pub struct SCALE_R(crate::FieldReader<u8, u8>);
impl SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scale` writer - "]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> CMP3IP_R {
        CMP3IP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> CMP2IP_R {
        CMP2IP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> CMP1IP_R {
        CMP1IP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&self) -> CMP0IP_R {
        CMP0IP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&self) -> CMP3GANG_R {
        CMP3GANG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&self) -> CMP2GANG_R {
        CMP2GANG_R::new(((self.bits >> 26) & 0x07ff) as u16)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&self) -> CMP1GANG_R {
        CMP1GANG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&self) -> CMP0GANG_R {
        CMP0GANG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&self) -> CMP3CENTER_R {
        CMP3CENTER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&self) -> CMP2CENTER_R {
        CMP2CENTER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&self) -> CMP1CENTER_R {
        CMP1CENTER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&self) -> CMP0CENTER_R {
        CMP0CENTER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&self) -> ENONESHOT_R {
        ENONESHOT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&self) -> DEGLITCH_R {
        DEGLITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&self) -> STICKY_R {
        STICKY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&mut self) -> CMP3IP_W {
        CMP3IP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&mut self) -> CMP2IP_W {
        CMP2IP_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&mut self) -> CMP1IP_W {
        CMP1IP_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&mut self) -> CMP0IP_W {
        CMP0IP_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&mut self) -> CMP3GANG_W {
        CMP3GANG_W { w: self }
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&mut self) -> CMP2GANG_W {
        CMP2GANG_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&mut self) -> CMP1GANG_W {
        CMP1GANG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&mut self) -> CMP0GANG_W {
        CMP0GANG_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&mut self) -> CMP3CENTER_W {
        CMP3CENTER_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&mut self) -> CMP2CENTER_W {
        CMP2CENTER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&mut self) -> CMP1CENTER_W {
        CMP1CENTER_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&mut self) -> CMP0CENTER_W {
        CMP0CENTER_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&mut self) -> ENONESHOT_W {
        ENONESHOT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W {
        ENALWAYS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&mut self) -> DEGLITCH_W {
        DEGLITCH_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&mut self) -> ZEROCMP_W {
        ZEROCMP_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&mut self) -> STICKY_W {
        STICKY_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
