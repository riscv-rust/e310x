#[doc = "Reader of register pending[%s]"]
pub type R = crate::R<u32, super::PENDING>;
#[doc = "Writer for register pending[%s]"]
pub type W = crate::W<u32, super::PENDING>;
#[doc = "Register pending[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PENDING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
