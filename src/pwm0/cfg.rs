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
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `sticky` reader - "]
pub type STICKY_R = crate::BitReader<bool>;
#[doc = "Field `sticky` writer - "]
pub type STICKY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `zerocmp` reader - "]
pub type ZEROCMP_R = crate::BitReader<bool>;
#[doc = "Field `zerocmp` writer - "]
pub type ZEROCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `deglitch` reader - "]
pub type DEGLITCH_R = crate::BitReader<bool>;
#[doc = "Field `deglitch` writer - "]
pub type DEGLITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader<bool>;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `enoneshot` reader - "]
pub type ENONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `enoneshot` writer - "]
pub type ENONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp0center` reader - "]
pub type CMP0CENTER_R = crate::BitReader<bool>;
#[doc = "Field `cmp0center` writer - "]
pub type CMP0CENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp1center` reader - "]
pub type CMP1CENTER_R = crate::BitReader<bool>;
#[doc = "Field `cmp1center` writer - "]
pub type CMP1CENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp2center` reader - "]
pub type CMP2CENTER_R = crate::BitReader<bool>;
#[doc = "Field `cmp2center` writer - "]
pub type CMP2CENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp3center` reader - "]
pub type CMP3CENTER_R = crate::BitReader<bool>;
#[doc = "Field `cmp3center` writer - "]
pub type CMP3CENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp0gang` reader - "]
pub type CMP0GANG_R = crate::BitReader<bool>;
#[doc = "Field `cmp0gang` writer - "]
pub type CMP0GANG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp1gang` reader - "]
pub type CMP1GANG_R = crate::BitReader<bool>;
#[doc = "Field `cmp1gang` writer - "]
pub type CMP1GANG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp2gang` reader - "]
pub type CMP2GANG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cmp2gang` writer - "]
pub type CMP2GANG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u16, u16, 11, O>;
#[doc = "Field `cmp3gang` reader - "]
pub type CMP3GANG_R = crate::BitReader<bool>;
#[doc = "Field `cmp3gang` writer - "]
pub type CMP3GANG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp0ip` reader - "]
pub type CMP0IP_R = crate::BitReader<bool>;
#[doc = "Field `cmp0ip` writer - "]
pub type CMP0IP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp1ip` reader - "]
pub type CMP1IP_R = crate::BitReader<bool>;
#[doc = "Field `cmp1ip` writer - "]
pub type CMP1IP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp2ip` reader - "]
pub type CMP2IP_R = crate::BitReader<bool>;
#[doc = "Field `cmp2ip` writer - "]
pub type CMP2IP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `cmp3ip` reader - "]
pub type CMP3IP_R = crate::BitReader<bool>;
#[doc = "Field `cmp3ip` writer - "]
pub type CMP3IP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&self) -> STICKY_R {
        STICKY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&self) -> ZEROCMP_R {
        ZEROCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&self) -> DEGLITCH_R {
        DEGLITCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&self) -> ENALWAYS_R {
        ENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&self) -> ENONESHOT_R {
        ENONESHOT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&self) -> CMP0CENTER_R {
        CMP0CENTER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&self) -> CMP1CENTER_R {
        CMP1CENTER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&self) -> CMP2CENTER_R {
        CMP2CENTER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&self) -> CMP3CENTER_R {
        CMP3CENTER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&self) -> CMP0GANG_R {
        CMP0GANG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&self) -> CMP1GANG_R {
        CMP1GANG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&self) -> CMP2GANG_R {
        CMP2GANG_R::new(((self.bits >> 26) & 0x07ff) as u16)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&self) -> CMP3GANG_R {
        CMP3GANG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&self) -> CMP0IP_R {
        CMP0IP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> CMP1IP_R {
        CMP1IP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> CMP2IP_R {
        CMP2IP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> CMP3IP_R {
        CMP3IP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<0> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sticky(&mut self) -> STICKY_W<8> {
        STICKY_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn zerocmp(&mut self) -> ZEROCMP_W<9> {
        ZEROCMP_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn deglitch(&mut self) -> DEGLITCH_W<10> {
        DEGLITCH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn enalways(&mut self) -> ENALWAYS_W<12> {
        ENALWAYS_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enoneshot(&mut self) -> ENONESHOT_W<13> {
        ENONESHOT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cmp0center(&mut self) -> CMP0CENTER_W<16> {
        CMP0CENTER_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cmp1center(&mut self) -> CMP1CENTER_W<17> {
        CMP1CENTER_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cmp2center(&mut self) -> CMP2CENTER_W<18> {
        CMP2CENTER_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cmp3center(&mut self) -> CMP3CENTER_W<19> {
        CMP3CENTER_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cmp0gang(&mut self) -> CMP0GANG_W<24> {
        CMP0GANG_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmp1gang(&mut self) -> CMP1GANG_W<25> {
        CMP1GANG_W::new(self)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    pub fn cmp2gang(&mut self) -> CMP2GANG_W<26> {
        CMP2GANG_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cmp3gang(&mut self) -> CMP3GANG_W<27> {
        CMP3GANG_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cmp0ip(&mut self) -> CMP0IP_W<28> {
        CMP0IP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cmp1ip(&mut self) -> CMP1IP_W<29> {
        CMP1IP_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cmp2ip(&mut self) -> CMP2IP_W<30> {
        CMP2IP_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cmp3ip(&mut self) -> CMP3IP_W<31> {
        CMP3IP_W::new(self)
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
