#[doc = "Reader of register lock"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register lock"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register lock `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
