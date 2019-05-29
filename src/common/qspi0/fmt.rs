#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
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
pub struct LENGTHR {
    bits: u8,
}
impl LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `direction`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTIONR {
    #[doc = "\n                    For dual and quad protocols, the DQ pins are tri-stated. For\n                    the single protocol, the DQ0 pin is driven with the transmit\n                    data as normal.\n                  "]
    RX,
    #[doc = "The receive FIFO is not populated."]
    TX,
}
impl DIRECTIONR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIRECTIONR::RX => false,
            DIRECTIONR::TX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRECTIONR {
        match value {
            false => DIRECTIONR::RX,
            true => DIRECTIONR::TX,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline]
    pub fn is_rx(&self) -> bool {
        *self == DIRECTIONR::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline]
    pub fn is_tx(&self) -> bool {
        *self == DIRECTIONR::TX
    }
}
#[doc = "Possible values of the field `endian`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANR {
    #[doc = "Transmit MSB first."]
    BIG,
    #[doc = "Transmit LSB first."]
    LITTLE,
}
impl ENDIANR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENDIANR::BIG => false,
            ENDIANR::LITTLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIANR {
        match value {
            false => ENDIANR::BIG,
            true => ENDIANR::LITTLE,
        }
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline]
    pub fn is_big(&self) -> bool {
        *self == ENDIANR::BIG
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline]
    pub fn is_little(&self) -> bool {
        *self == ENDIANR::LITTLE
    }
}
#[doc = "Possible values of the field `protocol`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTOCOLR {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROTOCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROTOCOLR::SINGLE => 0,
            PROTOCOLR::DUAL => 1,
            PROTOCOLR::QUAD => 2,
            PROTOCOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROTOCOLR {
        match value {
            0 => PROTOCOLR::SINGLE,
            1 => PROTOCOLR::DUAL,
            2 => PROTOCOLR::QUAD,
            i => PROTOCOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == PROTOCOLR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == PROTOCOLR::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline]
    pub fn is_quad(&self) -> bool {
        *self == PROTOCOLR::QUAD
    }
}
#[doc = r" Proxy"]
pub struct _LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `direction`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTIONW {
    #[doc = "\n                    For dual and quad protocols, the DQ pins are tri-stated. For\n                    the single protocol, the DQ0 pin is driven with the transmit\n                    data as normal.\n                  "]
    RX,
    #[doc = "The receive FIFO is not populated."]
    TX,
}
impl DIRECTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRECTIONW::RX => false,
            DIRECTIONW::TX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRECTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "For dual and quad protocols, the DQ pins are tri-stated. For the single protocol, the DQ0 pin is driven with the transmit data as normal."]
    #[inline]
    pub fn rx(self) -> &'a mut W {
        self.variant(DIRECTIONW::RX)
    }
    #[doc = "The receive FIFO is not populated."]
    #[inline]
    pub fn tx(self) -> &'a mut W {
        self.variant(DIRECTIONW::TX)
    }
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `endian`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANW {
    #[doc = "Transmit MSB first."]
    BIG,
    #[doc = "Transmit LSB first."]
    LITTLE,
}
impl ENDIANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDIANW::BIG => false,
            ENDIANW::LITTLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDIANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDIANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit MSB first."]
    #[inline]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIANW::BIG)
    }
    #[doc = "Transmit LSB first."]
    #[inline]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIANW::LITTLE)
    }
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `protocol`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTOCOLW {
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    SINGLE,
    #[doc = "DQ0, DQ1"]
    DUAL,
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    QUAD,
}
impl PROTOCOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROTOCOLW::SINGLE => 0,
            PROTOCOLW::DUAL => 1,
            PROTOCOLW::QUAD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTOCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTOCOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTOCOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DQ0 (MOSI), DQ1 (MISO)"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(PROTOCOLW::SINGLE)
    }
    #[doc = "DQ0, DQ1"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(PROTOCOLW::DUAL)
    }
    #[doc = "DQ0, DQ1, DQ2, DQ3"]
    #[inline]
    pub fn quad(self) -> &'a mut W {
        self.variant(PROTOCOLW::QUAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LENGTHR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn direction(&self) -> DIRECTIONR {
        DIRECTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn endian(&self) -> ENDIANR {
        ENDIANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn protocol(&self) -> PROTOCOLR {
        PROTOCOLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn length(&mut self) -> _LENGTHW {
        _LENGTHW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn direction(&mut self) -> _DIRECTIONW {
        _DIRECTIONW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn endian(&mut self) -> _ENDIANW {
        _ENDIANW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn protocol(&mut self) -> _PROTOCOLW {
        _PROTOCOLW { w: self }
    }
}
