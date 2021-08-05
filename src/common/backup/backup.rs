#[doc = "Register `backup[%s]` reader"]
pub struct R(crate::R<BACKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `backup[%s]` writer"]
pub struct W(crate::W<BACKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_SPEC>;
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
impl From<crate::W<BACKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup](index.html) module"]
pub struct BACKUP_SPEC;
impl crate::RegisterSpec for BACKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup::R](R) reader structure"]
impl crate::Readable for BACKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup::W](W) writer structure"]
impl crate::Writable for BACKUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets backup[%s]
to value 0"]
impl crate::Resettable for BACKUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
