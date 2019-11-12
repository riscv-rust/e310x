#[doc = "Reader of register csdef"]
pub type R = crate::R<u32, super::CSDEF>;
#[doc = "Writer for register csdef"]
pub type W = crate::W<u32, super::CSDEF>;
#[doc = "Register csdef `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CSDEF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
impl R {}
impl W {}
