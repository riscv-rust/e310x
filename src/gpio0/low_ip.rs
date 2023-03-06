#[doc = "Register `low_ip` reader"]
pub struct R(crate::R<LOW_IP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_IP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_IP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_IP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `low_ip` writer"]
pub struct W(crate::W<LOW_IP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOW_IP_SPEC>;
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
impl From<crate::W<LOW_IP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOW_IP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - "]
pub type PIN0_R = crate::BitReader<bool>;
#[doc = "Field `pin0` writer - "]
pub type PIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin1` reader - "]
pub type PIN1_R = crate::BitReader<bool>;
#[doc = "Field `pin1` writer - "]
pub type PIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin2` reader - "]
pub type PIN2_R = crate::BitReader<bool>;
#[doc = "Field `pin2` writer - "]
pub type PIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin3` reader - "]
pub type PIN3_R = crate::BitReader<bool>;
#[doc = "Field `pin3` writer - "]
pub type PIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin4` reader - "]
pub type PIN4_R = crate::BitReader<bool>;
#[doc = "Field `pin4` writer - "]
pub type PIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin5` reader - "]
pub type PIN5_R = crate::BitReader<bool>;
#[doc = "Field `pin5` writer - "]
pub type PIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin6` reader - "]
pub type PIN6_R = crate::BitReader<bool>;
#[doc = "Field `pin6` writer - "]
pub type PIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin7` reader - "]
pub type PIN7_R = crate::BitReader<bool>;
#[doc = "Field `pin7` writer - "]
pub type PIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin8` reader - "]
pub type PIN8_R = crate::BitReader<bool>;
#[doc = "Field `pin8` writer - "]
pub type PIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin9` reader - "]
pub type PIN9_R = crate::BitReader<bool>;
#[doc = "Field `pin9` writer - "]
pub type PIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin10` reader - "]
pub type PIN10_R = crate::BitReader<bool>;
#[doc = "Field `pin10` writer - "]
pub type PIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin11` reader - "]
pub type PIN11_R = crate::BitReader<bool>;
#[doc = "Field `pin11` writer - "]
pub type PIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin12` reader - "]
pub type PIN12_R = crate::BitReader<bool>;
#[doc = "Field `pin12` writer - "]
pub type PIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin13` reader - "]
pub type PIN13_R = crate::BitReader<bool>;
#[doc = "Field `pin13` writer - "]
pub type PIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin14` reader - "]
pub type PIN14_R = crate::BitReader<bool>;
#[doc = "Field `pin14` writer - "]
pub type PIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin15` reader - "]
pub type PIN15_R = crate::BitReader<bool>;
#[doc = "Field `pin15` writer - "]
pub type PIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin16` reader - "]
pub type PIN16_R = crate::BitReader<bool>;
#[doc = "Field `pin16` writer - "]
pub type PIN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin17` reader - "]
pub type PIN17_R = crate::BitReader<bool>;
#[doc = "Field `pin17` writer - "]
pub type PIN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin18` reader - "]
pub type PIN18_R = crate::BitReader<bool>;
#[doc = "Field `pin18` writer - "]
pub type PIN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin19` reader - "]
pub type PIN19_R = crate::BitReader<bool>;
#[doc = "Field `pin19` writer - "]
pub type PIN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin20` reader - "]
pub type PIN20_R = crate::BitReader<bool>;
#[doc = "Field `pin20` writer - "]
pub type PIN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin21` reader - "]
pub type PIN21_R = crate::BitReader<bool>;
#[doc = "Field `pin21` writer - "]
pub type PIN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin22` reader - "]
pub type PIN22_R = crate::BitReader<bool>;
#[doc = "Field `pin22` writer - "]
pub type PIN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin23` reader - "]
pub type PIN23_R = crate::BitReader<bool>;
#[doc = "Field `pin23` writer - "]
pub type PIN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin24` reader - "]
pub type PIN24_R = crate::BitReader<bool>;
#[doc = "Field `pin24` writer - "]
pub type PIN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin25` reader - "]
pub type PIN25_R = crate::BitReader<bool>;
#[doc = "Field `pin25` writer - "]
pub type PIN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin26` reader - "]
pub type PIN26_R = crate::BitReader<bool>;
#[doc = "Field `pin26` writer - "]
pub type PIN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin27` reader - "]
pub type PIN27_R = crate::BitReader<bool>;
#[doc = "Field `pin27` writer - "]
pub type PIN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin28` reader - "]
pub type PIN28_R = crate::BitReader<bool>;
#[doc = "Field `pin28` writer - "]
pub type PIN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin29` reader - "]
pub type PIN29_R = crate::BitReader<bool>;
#[doc = "Field `pin29` writer - "]
pub type PIN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin30` reader - "]
pub type PIN30_R = crate::BitReader<bool>;
#[doc = "Field `pin30` writer - "]
pub type PIN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
#[doc = "Field `pin31` reader - "]
pub type PIN31_R = crate::BitReader<bool>;
#[doc = "Field `pin31` writer - "]
pub type PIN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOW_IP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W<1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W<2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W<3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W<4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W<5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W<6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W<7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W<8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W<9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W<10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W<11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W<12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W<13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W<14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W<15> {
        PIN15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pin16(&mut self) -> PIN16_W<16> {
        PIN16_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pin17(&mut self) -> PIN17_W<17> {
        PIN17_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pin18(&mut self) -> PIN18_W<18> {
        PIN18_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pin19(&mut self) -> PIN19_W<19> {
        PIN19_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pin20(&mut self) -> PIN20_W<20> {
        PIN20_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pin21(&mut self) -> PIN21_W<21> {
        PIN21_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pin22(&mut self) -> PIN22_W<22> {
        PIN22_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pin23(&mut self) -> PIN23_W<23> {
        PIN23_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pin24(&mut self) -> PIN24_W<24> {
        PIN24_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pin25(&mut self) -> PIN25_W<25> {
        PIN25_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pin26(&mut self) -> PIN26_W<26> {
        PIN26_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin27(&mut self) -> PIN27_W<27> {
        PIN27_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pin28(&mut self) -> PIN28_W<28> {
        PIN28_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pin29(&mut self) -> PIN29_W<29> {
        PIN29_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pin30(&mut self) -> PIN30_W<30> {
        PIN30_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin31(&mut self) -> PIN31_W<31> {
        PIN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_ip](index.html) module"]
pub struct LOW_IP_SPEC;
impl crate::RegisterSpec for LOW_IP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_ip::R](R) reader structure"]
impl crate::Readable for LOW_IP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [low_ip::W](W) writer structure"]
impl crate::Writable for LOW_IP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets low_ip to value 0"]
impl crate::Resettable for LOW_IP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
