#[doc = "Register `plloutdiv` reader"]
pub type R = crate::R<PLLOUTDIV_SPEC>;
#[doc = "Register `plloutdiv` writer"]
pub type W = crate::W<PLLOUTDIV_SPEC>;
#[doc = "Field `div` reader - "]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `div` writer - "]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `divby1` reader - "]
pub type DIVBY1_R = crate::BitReader;
#[doc = "Field `divby1` writer - "]
pub type DIVBY1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn divby1(&self) -> DIVBY1_R {
        DIVBY1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<PLLOUTDIV_SPEC> {
        DIV_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn divby1(&mut self) -> DIVBY1_W<PLLOUTDIV_SPEC> {
        DIVBY1_W::new(self, 8)
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
#[doc = "PLL Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plloutdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plloutdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLOUTDIV_SPEC;
impl crate::RegisterSpec for PLLOUTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plloutdiv::R`](R) reader structure"]
impl crate::Readable for PLLOUTDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plloutdiv::W`](W) writer structure"]
impl crate::Writable for PLLOUTDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets plloutdiv to value 0x0100"]
impl crate::Resettable for PLLOUTDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
