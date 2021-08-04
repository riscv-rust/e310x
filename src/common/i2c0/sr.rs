#[doc = "Register `sr` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rx_ack` reader - Received acknowledge from slave. This flag represents acknowledge from the addressed slave. '1' = No acknowledge received '0' = Acknowledge received"]
pub struct RX_ACK_R(crate::FieldReader<bool, bool>);
impl RX_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `busy` reader - I2C bus busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `al` reader - Arbitration lost"]
pub struct AL_R(crate::FieldReader<bool, bool>);
impl AL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tip` reader - Transfer in progress"]
pub struct TIP_R(crate::FieldReader<bool, bool>);
impl TIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `if` reader - Interrupt Flag. This bit is set when an interrupt is pending, which will cause a processor interrupt request if the IEN bit is set."]
pub struct IF_R(crate::FieldReader<bool, bool>);
impl IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Received acknowledge from slave. This flag represents acknowledge from the addressed slave. '1' = No acknowledge received '0' = Acknowledge received"]
    #[inline(always)]
    pub fn rx_ack(&self) -> RX_ACK_R {
        RX_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Arbitration lost"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer in progress"]
    #[inline(always)]
    pub fn tip(&self) -> TIP_R {
        TIP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Interrupt Flag. This bit is set when an interrupt is pending, which will cause a processor interrupt request if the IEN bit is set."]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sr to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
