#[doc = "Reader of register wdogcount"]
pub type R = crate::R<u32, super::WDOGCOUNT>;
#[doc = "Writer for register wdogcount"]
pub type W = crate::W<u32, super::WDOGCOUNT>;
#[doc = "Register wdogcount `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOGCOUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
