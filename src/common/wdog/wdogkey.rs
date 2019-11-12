#[doc = "Writer for register wdogkey"]
pub type W = crate::W<u32, super::WDOGKEY>;
#[doc = "Register wdogkey `reset()`'s with value 0x0051_f15e"]
impl crate::ResetValue for super::WDOGKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0051_f15e
    }
}
impl W {}
