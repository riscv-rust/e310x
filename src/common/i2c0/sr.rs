#[doc = "Reader of register sr"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `rx_ack`"]
pub type RX_ACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `busy`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `al`"]
pub type AL_R = crate::R<bool, bool>;
#[doc = "Reader of field `tip`"]
pub type TIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `if`"]
pub type IF_R = crate::R<bool, bool>;
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
