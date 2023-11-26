#[doc = "Register `cfg` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `scale` reader - "]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `scale` writer - "]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sticky` reader - "]
pub type STICKY_R = crate::BitReader;
#[doc = "Field `sticky` writer - "]
pub type STICKY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `zerocmp` reader - "]
pub type ZEROCMP_R = crate::BitReader;
#[doc = "Field `zerocmp` writer - "]
pub type ZEROCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `deglitch` reader - "]
pub type DEGLITCH_R = crate::BitReader;
#[doc = "Field `deglitch` writer - "]
pub type DEGLITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enalways` reader - "]
pub type ENALWAYS_R = crate::BitReader;
#[doc = "Field `enalways` writer - "]
pub type ENALWAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enoneshot` reader - "]
pub type ENONESHOT_R = crate::BitReader;
#[doc = "Field `enoneshot` writer - "]
pub type ENONESHOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0center` reader - "]
pub type CMP0CENTER_R = crate::BitReader;
#[doc = "Field `cmp0center` writer - "]
pub type CMP0CENTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1center` reader - "]
pub type CMP1CENTER_R = crate::BitReader;
#[doc = "Field `cmp1center` writer - "]
pub type CMP1CENTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2center` reader - "]
pub type CMP2CENTER_R = crate::BitReader;
#[doc = "Field `cmp2center` writer - "]
pub type CMP2CENTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp3center` reader - "]
pub type CMP3CENTER_R = crate::BitReader;
#[doc = "Field `cmp3center` writer - "]
pub type CMP3CENTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0gang` reader - "]
pub type CMP0GANG_R = crate::BitReader;
#[doc = "Field `cmp0gang` writer - "]
pub type CMP0GANG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1gang` reader - "]
pub type CMP1GANG_R = crate::BitReader;
#[doc = "Field `cmp1gang` writer - "]
pub type CMP1GANG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2gang` reader - "]
pub type CMP2GANG_R = crate::FieldReader<u16>;
#[doc = "Field `cmp2gang` writer - "]
pub type CMP2GANG_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `cmp3gang` reader - "]
pub type CMP3GANG_R = crate::BitReader;
#[doc = "Field `cmp3gang` writer - "]
pub type CMP3GANG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp0ip` reader - "]
pub type CMP0IP_R = crate::BitReader;
#[doc = "Field `cmp0ip` writer - "]
pub type CMP0IP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp1ip` reader - "]
pub type CMP1IP_R = crate::BitReader;
#[doc = "Field `cmp1ip` writer - "]
pub type CMP1IP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp2ip` reader - "]
pub type CMP2IP_R = crate::BitReader;
#[doc = "Field `cmp2ip` writer - "]
pub type CMP2IP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmp3ip` reader - "]
pub type CMP3IP_R = crate::BitReader;
#[doc = "Field `cmp3ip` writer - "]
pub type CMP3IP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<CFG_SPEC> {
        SCALE_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sticky(&mut self) -> STICKY_W<CFG_SPEC> {
        STICKY_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn zerocmp(&mut self) -> ZEROCMP_W<CFG_SPEC> {
        ZEROCMP_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch(&mut self) -> DEGLITCH_W<CFG_SPEC> {
        DEGLITCH_W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn enalways(&mut self) -> ENALWAYS_W<CFG_SPEC> {
        ENALWAYS_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enoneshot(&mut self) -> ENONESHOT_W<CFG_SPEC> {
        ENONESHOT_W::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0center(&mut self) -> CMP0CENTER_W<CFG_SPEC> {
        CMP0CENTER_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1center(&mut self) -> CMP1CENTER_W<CFG_SPEC> {
        CMP1CENTER_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2center(&mut self) -> CMP2CENTER_W<CFG_SPEC> {
        CMP2CENTER_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3center(&mut self) -> CMP3CENTER_W<CFG_SPEC> {
        CMP3CENTER_W::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0gang(&mut self) -> CMP0GANG_W<CFG_SPEC> {
        CMP0GANG_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1gang(&mut self) -> CMP1GANG_W<CFG_SPEC> {
        CMP1GANG_W::new(self, 25)
    }
    #[doc = "Bits 26:36"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2gang(&mut self) -> CMP2GANG_W<CFG_SPEC> {
        CMP2GANG_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3gang(&mut self) -> CMP3GANG_W<CFG_SPEC> {
        CMP3GANG_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ip(&mut self) -> CMP0IP_W<CFG_SPEC> {
        CMP0IP_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ip(&mut self) -> CMP1IP_W<CFG_SPEC> {
        CMP1IP_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ip(&mut self) -> CMP2IP_W<CFG_SPEC> {
        CMP2IP_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ip(&mut self) -> CMP3IP_W<CFG_SPEC> {
        CMP3IP_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
