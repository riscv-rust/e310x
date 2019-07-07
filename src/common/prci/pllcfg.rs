#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct LOCKR {
    bits: bool,
}
impl LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BYPASSR {
    bits: bool,
}
impl BYPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct REFSELR {
    bits: bool,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SELR {
    bits: bool,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `pllq`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLQR {
    #[doc = "undocumented"]
    Q2,
    #[doc = "undocumented"]
    Q4,
    #[doc = "undocumented"]
    Q8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLLQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLQR::Q2 => 1,
            PLLQR::Q4 => 2,
            PLLQR::Q8 => 3,
            PLLQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLQR {
        match value {
            1 => PLLQR::Q2,
            2 => PLLQR::Q4,
            3 => PLLQR::Q8,
            i => PLLQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `Q2`"]
    #[inline]
    pub fn is_q2(&self) -> bool {
        *self == PLLQR::Q2
    }
    #[doc = "Checks if the value of the field is `Q4`"]
    #[inline]
    pub fn is_q4(&self) -> bool {
        *self == PLLQR::Q4
    }
    #[doc = "Checks if the value of the field is `Q8`"]
    #[inline]
    pub fn is_q8(&self) -> bool {
        *self == PLLQR::Q8
    }
}
#[doc = r" Value of the field"]
pub struct PLLFR {
    bits: u8,
}
impl PLLFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `pllr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRR {
    #[doc = "undocumented"]
    R1,
    #[doc = "undocumented"]
    R2,
    #[doc = "undocumented"]
    R3,
    #[doc = "undocumented"]
    R4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLLRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLRR::R1 => 0,
            PLLRR::R2 => 1,
            PLLRR::R3 => 2,
            PLLRR::R4 => 3,
            PLLRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLRR {
        match value {
            0 => PLLRR::R1,
            1 => PLLRR::R2,
            2 => PLLRR::R3,
            3 => PLLRR::R4,
            i => PLLRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `R1`"]
    #[inline]
    pub fn is_r1(&self) -> bool {
        *self == PLLRR::R1
    }
    #[doc = "Checks if the value of the field is `R2`"]
    #[inline]
    pub fn is_r2(&self) -> bool {
        *self == PLLRR::R2
    }
    #[doc = "Checks if the value of the field is `R3`"]
    #[inline]
    pub fn is_r3(&self) -> bool {
        *self == PLLRR::R3
    }
    #[doc = "Checks if the value of the field is `R4`"]
    #[inline]
    pub fn is_r4(&self) -> bool {
        *self == PLLRR::R4
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pllq`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLQW {
    #[doc = "`1`"]
    Q2,
    #[doc = "`10`"]
    Q4,
    #[doc = "`11`"]
    Q8,
}
impl PLLQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLQW::Q2 => 1,
            PLLQW::Q4 => 2,
            PLLQW::Q8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn q2(self) -> &'a mut W {
        self.variant(PLLQW::Q2)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn q4(self) -> &'a mut W {
        self.variant(PLLQW::Q4)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn q8(self) -> &'a mut W {
        self.variant(PLLQW::Q8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLFW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pllr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRW {
    #[doc = "`0`"]
    R1,
    #[doc = "`1`"]
    R2,
    #[doc = "`10`"]
    R3,
    #[doc = "`11`"]
    R4,
}
impl PLLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLRW::R1 => 0,
            PLLRW::R2 => 1,
            PLLRW::R3 => 2,
            PLLRW::R4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn r1(self) -> &'a mut W {
        self.variant(PLLRW::R1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn r2(self) -> &'a mut W {
        self.variant(PLLRW::R2)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn r3(self) -> &'a mut W {
        self.variant(PLLRW::R3)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn r4(self) -> &'a mut W {
        self.variant(PLLRW::R4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASSR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REFSELR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sel(&self) -> SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SELR { bits }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn pllq(&self) -> PLLQR {
        PLLQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:9"]
    #[inline]
    pub fn pllf(&self) -> PLLFR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLFR { bits }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn pllr(&self) -> PLLRR {
        PLLRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 198393 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn pllq(&mut self) -> _PLLQW {
        _PLLQW { w: self }
    }
    #[doc = "Bits 4:9"]
    #[inline]
    pub fn pllf(&mut self) -> _PLLFW {
        _PLLFW { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn pllr(&mut self) -> _PLLRW {
        _PLLRW { w: self }
    }
}
