#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FFMT {
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
pub struct PAD_CODER {
    bits: u8,
}
impl PAD_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMD_CODER {
    bits: u8,
}
impl CMD_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `data_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_PROTOR {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA_PROTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_PROTOR::SINGLE => 0,
            DATA_PROTOR::DUAL => 1,
            DATA_PROTOR::QUAD => 2,
            DATA_PROTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_PROTOR {
        match value {
            0 => DATA_PROTOR::SINGLE,
            1 => DATA_PROTOR::DUAL,
            2 => DATA_PROTOR::QUAD,
            i => DATA_PROTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == DATA_PROTOR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == DATA_PROTOR::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline]
    pub fn is_quad(&self) -> bool {
        *self == DATA_PROTOR::QUAD
    }
}
#[doc = "Possible values of the field `addr_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_PROTOR {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADDR_PROTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDR_PROTOR::SINGLE => 0,
            ADDR_PROTOR::DUAL => 1,
            ADDR_PROTOR::QUAD => 2,
            ADDR_PROTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDR_PROTOR {
        match value {
            0 => ADDR_PROTOR::SINGLE,
            1 => ADDR_PROTOR::DUAL,
            2 => ADDR_PROTOR::QUAD,
            i => ADDR_PROTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == ADDR_PROTOR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == ADDR_PROTOR::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline]
    pub fn is_quad(&self) -> bool {
        *self == ADDR_PROTOR::QUAD
    }
}
#[doc = "Possible values of the field `cmd_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_PROTOR {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMD_PROTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_PROTOR::SINGLE => 0,
            CMD_PROTOR::DUAL => 1,
            CMD_PROTOR::QUAD => 2,
            CMD_PROTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_PROTOR {
        match value {
            0 => CMD_PROTOR::SINGLE,
            1 => CMD_PROTOR::DUAL,
            2 => CMD_PROTOR::QUAD,
            i => CMD_PROTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == CMD_PROTOR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == CMD_PROTOR::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline]
    pub fn is_quad(&self) -> bool {
        *self == CMD_PROTOR::QUAD
    }
}
#[doc = r" Value of the field"]
pub struct PAD_CNTR {
    bits: bool,
}
impl PAD_CNTR {
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
pub struct ADDR_LENR {
    bits: u8,
}
impl ADDR_LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMD_ENR {
    bits: bool,
}
impl CMD_ENR {
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
#[doc = r" Proxy"]
pub struct _PAD_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_CODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMD_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `data_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_PROTOW {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
}
impl DATA_PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATA_PROTOW::SINGLE => 0,
            DATA_PROTOW::DUAL => 1,
            DATA_PROTOW::QUAD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_PROTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_PROTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(DATA_PROTOW::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(DATA_PROTOW::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline]
    pub fn quad(self) -> &'a mut W {
        self.variant(DATA_PROTOW::QUAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `addr_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_PROTOW {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
}
impl ADDR_PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDR_PROTOW::SINGLE => 0,
            ADDR_PROTOW::DUAL => 1,
            ADDR_PROTOW::QUAD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_PROTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDR_PROTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(ADDR_PROTOW::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(ADDR_PROTOW::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline]
    pub fn quad(self) -> &'a mut W {
        self.variant(ADDR_PROTOW::QUAD)
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
#[doc = "Values that can be written to the field `cmd_proto`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_PROTOW {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
}
impl CMD_PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_PROTOW::SINGLE => 0,
            CMD_PROTOW::DUAL => 1,
            CMD_PROTOW::QUAD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_PROTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_PROTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(CMD_PROTOW::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(CMD_PROTOW::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline]
    pub fn quad(self) -> &'a mut W {
        self.variant(CMD_PROTOW::QUAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAD_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_CNTW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_LENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_ENW<'a> {
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
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline]
    pub fn pad_code(&self) -> PAD_CODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAD_CODER { bits }
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline]
    pub fn cmd_code(&self) -> CMD_CODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMD_CODER { bits }
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline]
    pub fn data_proto(&self) -> DATA_PROTOR {
        DATA_PROTOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline]
    pub fn addr_proto(&self) -> ADDR_PROTOR {
        ADDR_PROTOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline]
    pub fn cmd_proto(&self) -> CMD_PROTOR {
        CMD_PROTOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - Number of dummy cycles"]
    #[inline]
    pub fn pad_cnt(&self) -> PAD_CNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAD_CNTR { bits }
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline]
    pub fn addr_len(&self) -> ADDR_LENR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR_LENR { bits }
    }
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline]
    pub fn cmd_en(&self) -> CMD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMD_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - First 8 bits to transmit during dummy cycles"]
    #[inline]
    pub fn pad_code(&mut self) -> _PAD_CODEW {
        _PAD_CODEW { w: self }
    }
    #[doc = "Bits 16:23 - Value of command byte"]
    #[inline]
    pub fn cmd_code(&mut self) -> _CMD_CODEW {
        _CMD_CODEW { w: self }
    }
    #[doc = "Bits 12:13 - Protocol for receiving data bytes"]
    #[inline]
    pub fn data_proto(&mut self) -> _DATA_PROTOW {
        _DATA_PROTOW { w: self }
    }
    #[doc = "Bits 10:11 - Protocol for transmitting address and padding"]
    #[inline]
    pub fn addr_proto(&mut self) -> _ADDR_PROTOW {
        _ADDR_PROTOW { w: self }
    }
    #[doc = "Bits 8:9 - Protocol for transmitting command"]
    #[inline]
    pub fn cmd_proto(&mut self) -> _CMD_PROTOW {
        _CMD_PROTOW { w: self }
    }
    #[doc = "Bit 0 - Number of dummy cycles"]
    #[inline]
    pub fn pad_cnt(&mut self) -> _PAD_CNTW {
        _PAD_CNTW { w: self }
    }
    #[doc = "Bits 1:3 - Number of address bytes (0 to 4)"]
    #[inline]
    pub fn addr_len(&mut self) -> _ADDR_LENW {
        _ADDR_LENW { w: self }
    }
    #[doc = "Bit 0 - Enable sending of command"]
    #[inline]
    pub fn cmd_en(&mut self) -> _CMD_ENW {
        _CMD_ENW { w: self }
    }
}
