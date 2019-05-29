#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOF_SEL {
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
#[doc = "Possible values of the field `pin0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM0_0,
}
impl PIN0R {
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
            PIN0R::IOF0 => false,
            PIN0R::PWM0_0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN0R {
        match value {
            false => PIN0R::IOF0,
            true => PIN0R::PWM0_0,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN0R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM0_0`"]
    #[inline]
    pub fn is_pwm0_0(&self) -> bool {
        *self == PIN0R::PWM0_0
    }
}
#[doc = "Possible values of the field `pin1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM0_1,
}
impl PIN1R {
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
            PIN1R::IOF0 => false,
            PIN1R::PWM0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN1R {
        match value {
            false => PIN1R::IOF0,
            true => PIN1R::PWM0_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN1R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM0_1`"]
    #[inline]
    pub fn is_pwm0_1(&self) -> bool {
        *self == PIN1R::PWM0_1
    }
}
#[doc = "Possible values of the field `pin2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = "undocumented"]
    QSPI1_SS0,
    #[doc = "undocumented"]
    PWM0_2,
}
impl PIN2R {
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
            PIN2R::QSPI1_SS0 => false,
            PIN2R::PWM0_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN2R {
        match value {
            false => PIN2R::QSPI1_SS0,
            true => PIN2R::PWM0_2,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS0`"]
    #[inline]
    pub fn is_qspi1_ss0(&self) -> bool {
        *self == PIN2R::QSPI1_SS0
    }
    #[doc = "Checks if the value of the field is `PWM0_2`"]
    #[inline]
    pub fn is_pwm0_2(&self) -> bool {
        *self == PIN2R::PWM0_2
    }
}
#[doc = "Possible values of the field `pin3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = "undocumented"]
    QSPI1_SD0,
    #[doc = "undocumented"]
    PWM0_3,
}
impl PIN3R {
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
            PIN3R::QSPI1_SD0 => false,
            PIN3R::PWM0_3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN3R {
        match value {
            false => PIN3R::QSPI1_SD0,
            true => PIN3R::PWM0_3,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD0`"]
    #[inline]
    pub fn is_qspi1_sd0(&self) -> bool {
        *self == PIN3R::QSPI1_SD0
    }
    #[doc = "Checks if the value of the field is `PWM0_3`"]
    #[inline]
    pub fn is_pwm0_3(&self) -> bool {
        *self == PIN3R::PWM0_3
    }
}
#[doc = "Possible values of the field `pin4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = "undocumented"]
    QSPI1_SD1,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN4R {
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
            PIN4R::QSPI1_SD1 => false,
            PIN4R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN4R {
        match value {
            false => PIN4R::QSPI1_SD1,
            true => PIN4R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD1`"]
    #[inline]
    pub fn is_qspi1_sd1(&self) -> bool {
        *self == PIN4R::QSPI1_SD1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN4R::IOF1
    }
}
#[doc = "Possible values of the field `pin5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = "undocumented"]
    QSPI1_SCK,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN5R {
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
            PIN5R::QSPI1_SCK => false,
            PIN5R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN5R {
        match value {
            false => PIN5R::QSPI1_SCK,
            true => PIN5R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SCK`"]
    #[inline]
    pub fn is_qspi1_sck(&self) -> bool {
        *self == PIN5R::QSPI1_SCK
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN5R::IOF1
    }
}
#[doc = "Possible values of the field `pin6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = "undocumented"]
    QSPI1_SD2,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN6R {
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
            PIN6R::QSPI1_SD2 => false,
            PIN6R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN6R {
        match value {
            false => PIN6R::QSPI1_SD2,
            true => PIN6R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD2`"]
    #[inline]
    pub fn is_qspi1_sd2(&self) -> bool {
        *self == PIN6R::QSPI1_SD2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN6R::IOF1
    }
}
#[doc = "Possible values of the field `pin7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = "undocumented"]
    QSPI1_SD3,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN7R {
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
            PIN7R::QSPI1_SD3 => false,
            PIN7R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN7R {
        match value {
            false => PIN7R::QSPI1_SD3,
            true => PIN7R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD3`"]
    #[inline]
    pub fn is_qspi1_sd3(&self) -> bool {
        *self == PIN7R::QSPI1_SD3
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN7R::IOF1
    }
}
#[doc = "Possible values of the field `pin8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8R {
    #[doc = "undocumented"]
    QSPI1_SS1,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN8R {
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
            PIN8R::QSPI1_SS1 => false,
            PIN8R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN8R {
        match value {
            false => PIN8R::QSPI1_SS1,
            true => PIN8R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS1`"]
    #[inline]
    pub fn is_qspi1_ss1(&self) -> bool {
        *self == PIN8R::QSPI1_SS1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN8R::IOF1
    }
}
#[doc = "Possible values of the field `pin9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9R {
    #[doc = "undocumented"]
    QSPI1_SS2,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN9R {
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
            PIN9R::QSPI1_SS2 => false,
            PIN9R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN9R {
        match value {
            false => PIN9R::QSPI1_SS2,
            true => PIN9R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS2`"]
    #[inline]
    pub fn is_qspi1_ss2(&self) -> bool {
        *self == PIN9R::QSPI1_SS2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN9R::IOF1
    }
}
#[doc = "Possible values of the field `pin10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10R {
    #[doc = "undocumented"]
    QSPI1_SS3,
    #[doc = "undocumented"]
    PWM2_0,
}
impl PIN10R {
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
            PIN10R::QSPI1_SS3 => false,
            PIN10R::PWM2_0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN10R {
        match value {
            false => PIN10R::QSPI1_SS3,
            true => PIN10R::PWM2_0,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS3`"]
    #[inline]
    pub fn is_qspi1_ss3(&self) -> bool {
        *self == PIN10R::QSPI1_SS3
    }
    #[doc = "Checks if the value of the field is `PWM2_0`"]
    #[inline]
    pub fn is_pwm2_0(&self) -> bool {
        *self == PIN10R::PWM2_0
    }
}
#[doc = "Possible values of the field `pin11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM2_1,
}
impl PIN11R {
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
            PIN11R::IOF0 => false,
            PIN11R::PWM2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN11R {
        match value {
            false => PIN11R::IOF0,
            true => PIN11R::PWM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN11R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_1`"]
    #[inline]
    pub fn is_pwm2_1(&self) -> bool {
        *self == PIN11R::PWM2_1
    }
}
#[doc = "Possible values of the field `pin12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM2_2,
}
impl PIN12R {
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
            PIN12R::IOF0 => false,
            PIN12R::PWM2_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN12R {
        match value {
            false => PIN12R::IOF0,
            true => PIN12R::PWM2_2,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN12R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_2`"]
    #[inline]
    pub fn is_pwm2_2(&self) -> bool {
        *self == PIN12R::PWM2_2
    }
}
#[doc = "Possible values of the field `pin13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM2_3,
}
impl PIN13R {
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
            PIN13R::IOF0 => false,
            PIN13R::PWM2_3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN13R {
        match value {
            false => PIN13R::IOF0,
            true => PIN13R::PWM2_3,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN13R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_3`"]
    #[inline]
    pub fn is_pwm2_3(&self) -> bool {
        *self == PIN13R::PWM2_3
    }
}
#[doc = "Possible values of the field `pin14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN14R {
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
            PIN14R::IOF0 => false,
            PIN14R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN14R {
        match value {
            false => PIN14R::IOF0,
            true => PIN14R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN14R::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN14R::IOF1
    }
}
#[doc = "Possible values of the field `pin15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN15R {
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
            PIN15R::IOF0 => false,
            PIN15R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN15R {
        match value {
            false => PIN15R::IOF0,
            true => PIN15R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN15R::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN15R::IOF1
    }
}
#[doc = "Possible values of the field `pin16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16R {
    #[doc = "undocumented"]
    UART0_RX,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN16R {
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
            PIN16R::UART0_RX => false,
            PIN16R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN16R {
        match value {
            false => PIN16R::UART0_RX,
            true => PIN16R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PIN16R::UART0_RX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN16R::IOF1
    }
}
#[doc = "Possible values of the field `pin17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17R {
    #[doc = "undocumented"]
    UART0_TX,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN17R {
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
            PIN17R::UART0_TX => false,
            PIN17R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN17R {
        match value {
            false => PIN17R::UART0_TX,
            true => PIN17R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PIN17R::UART0_TX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN17R::IOF1
    }
}
#[doc = "Possible values of the field `pin18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN18R {
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
            PIN18R::IOF0 => false,
            PIN18R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN18R {
        match value {
            false => PIN18R::IOF0,
            true => PIN18R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN18R::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN18R::IOF1
    }
}
#[doc = "Possible values of the field `pin19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM1_1,
}
impl PIN19R {
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
            PIN19R::IOF0 => false,
            PIN19R::PWM1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN19R {
        match value {
            false => PIN19R::IOF0,
            true => PIN19R::PWM1_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN19R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_1`"]
    #[inline]
    pub fn is_pwm1_1(&self) -> bool {
        *self == PIN19R::PWM1_1
    }
}
#[doc = "Possible values of the field `pin20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM1_0,
}
impl PIN20R {
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
            PIN20R::IOF0 => false,
            PIN20R::PWM1_0 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN20R {
        match value {
            false => PIN20R::IOF0,
            true => PIN20R::PWM1_0,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN20R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_0`"]
    #[inline]
    pub fn is_pwm1_0(&self) -> bool {
        *self == PIN20R::PWM1_0
    }
}
#[doc = "Possible values of the field `pin21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM1_2,
}
impl PIN21R {
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
            PIN21R::IOF0 => false,
            PIN21R::PWM1_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN21R {
        match value {
            false => PIN21R::IOF0,
            true => PIN21R::PWM1_2,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN21R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_2`"]
    #[inline]
    pub fn is_pwm1_2(&self) -> bool {
        *self == PIN21R::PWM1_2
    }
}
#[doc = "Possible values of the field `pin22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    PWM1_3,
}
impl PIN22R {
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
            PIN22R::IOF0 => false,
            PIN22R::PWM1_3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN22R {
        match value {
            false => PIN22R::IOF0,
            true => PIN22R::PWM1_3,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN22R::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_3`"]
    #[inline]
    pub fn is_pwm1_3(&self) -> bool {
        *self == PIN22R::PWM1_3
    }
}
#[doc = "Possible values of the field `pin23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23R {
    #[doc = "undocumented"]
    IOF0,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN23R {
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
            PIN23R::IOF0 => false,
            PIN23R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN23R {
        match value {
            false => PIN23R::IOF0,
            true => PIN23R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline]
    pub fn is_iof0(&self) -> bool {
        *self == PIN23R::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN23R::IOF1
    }
}
#[doc = "Possible values of the field `pin24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24R {
    #[doc = "undocumented"]
    UART1_RX,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN24R {
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
            PIN24R::UART1_RX => false,
            PIN24R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN24R {
        match value {
            false => PIN24R::UART1_RX,
            true => PIN24R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PIN24R::UART1_RX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN24R::IOF1
    }
}
#[doc = "Possible values of the field `pin25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25R {
    #[doc = "undocumented"]
    UART1_TX,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN25R {
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
            PIN25R::UART1_TX => false,
            PIN25R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN25R {
        match value {
            false => PIN25R::UART1_TX,
            true => PIN25R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PIN25R::UART1_TX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN25R::IOF1
    }
}
#[doc = "Possible values of the field `pin26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26R {
    #[doc = "undocumented"]
    QSPI2_SS,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN26R {
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
            PIN26R::QSPI2_SS => false,
            PIN26R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN26R {
        match value {
            false => PIN26R::QSPI2_SS,
            true => PIN26R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SS`"]
    #[inline]
    pub fn is_qspi2_ss(&self) -> bool {
        *self == PIN26R::QSPI2_SS
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN26R::IOF1
    }
}
#[doc = "Possible values of the field `pin27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27R {
    #[doc = "undocumented"]
    QSPI2_SD0,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN27R {
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
            PIN27R::QSPI2_SD0 => false,
            PIN27R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN27R {
        match value {
            false => PIN27R::QSPI2_SD0,
            true => PIN27R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD0`"]
    #[inline]
    pub fn is_qspi2_sd0(&self) -> bool {
        *self == PIN27R::QSPI2_SD0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN27R::IOF1
    }
}
#[doc = "Possible values of the field `pin28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28R {
    #[doc = "undocumented"]
    QSPI2_SD1,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN28R {
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
            PIN28R::QSPI2_SD1 => false,
            PIN28R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN28R {
        match value {
            false => PIN28R::QSPI2_SD1,
            true => PIN28R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD1`"]
    #[inline]
    pub fn is_qspi2_sd1(&self) -> bool {
        *self == PIN28R::QSPI2_SD1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN28R::IOF1
    }
}
#[doc = "Possible values of the field `pin29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29R {
    #[doc = "undocumented"]
    QSPI2_SCK,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN29R {
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
            PIN29R::QSPI2_SCK => false,
            PIN29R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN29R {
        match value {
            false => PIN29R::QSPI2_SCK,
            true => PIN29R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SCK`"]
    #[inline]
    pub fn is_qspi2_sck(&self) -> bool {
        *self == PIN29R::QSPI2_SCK
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN29R::IOF1
    }
}
#[doc = "Possible values of the field `pin30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30R {
    #[doc = "undocumented"]
    QSPI2_SD2,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN30R {
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
            PIN30R::QSPI2_SD2 => false,
            PIN30R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN30R {
        match value {
            false => PIN30R::QSPI2_SD2,
            true => PIN30R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD2`"]
    #[inline]
    pub fn is_qspi2_sd2(&self) -> bool {
        *self == PIN30R::QSPI2_SD2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN30R::IOF1
    }
}
#[doc = "Possible values of the field `pin31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31R {
    #[doc = "undocumented"]
    QSPI2_SD3,
    #[doc = "undocumented"]
    IOF1,
}
impl PIN31R {
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
            PIN31R::QSPI2_SD3 => false,
            PIN31R::IOF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN31R {
        match value {
            false => PIN31R::QSPI2_SD3,
            true => PIN31R::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD3`"]
    #[inline]
    pub fn is_qspi2_sd3(&self) -> bool {
        *self == PIN31R::QSPI2_SD3
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline]
    pub fn is_iof1(&self) -> bool {
        *self == PIN31R::IOF1
    }
}
#[doc = "Values that can be written to the field `pin0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM0_0,
}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN0W::IOF0 => false,
            PIN0W::PWM0_0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN0W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm0_0(self) -> &'a mut W {
        self.variant(PIN0W::PWM0_0)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM0_1,
}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN1W::IOF0 => false,
            PIN1W::PWM0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN1W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm0_1(self) -> &'a mut W {
        self.variant(PIN1W::PWM0_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2W {
    #[doc = "`0`"]
    QSPI1_SS0,
    #[doc = "`1`"]
    PWM0_2,
}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN2W::QSPI1_SS0 => false,
            PIN2W::PWM0_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_ss0(self) -> &'a mut W {
        self.variant(PIN2W::QSPI1_SS0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm0_2(self) -> &'a mut W {
        self.variant(PIN2W::PWM0_2)
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
#[doc = "Values that can be written to the field `pin3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3W {
    #[doc = "`0`"]
    QSPI1_SD0,
    #[doc = "`1`"]
    PWM0_3,
}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN3W::QSPI1_SD0 => false,
            PIN3W::PWM0_3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_sd0(self) -> &'a mut W {
        self.variant(PIN3W::QSPI1_SD0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm0_3(self) -> &'a mut W {
        self.variant(PIN3W::PWM0_3)
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
#[doc = "Values that can be written to the field `pin4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4W {
    #[doc = "`0`"]
    QSPI1_SD1,
    #[doc = "`1`"]
    IOF1,
}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN4W::QSPI1_SD1 => false,
            PIN4W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_sd1(self) -> &'a mut W {
        self.variant(PIN4W::QSPI1_SD1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN4W::IOF1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5W {
    #[doc = "`0`"]
    QSPI1_SCK,
    #[doc = "`1`"]
    IOF1,
}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN5W::QSPI1_SCK => false,
            PIN5W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_sck(self) -> &'a mut W {
        self.variant(PIN5W::QSPI1_SCK)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN5W::IOF1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6W {
    #[doc = "`0`"]
    QSPI1_SD2,
    #[doc = "`1`"]
    IOF1,
}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN6W::QSPI1_SD2 => false,
            PIN6W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_sd2(self) -> &'a mut W {
        self.variant(PIN6W::QSPI1_SD2)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN6W::IOF1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7W {
    #[doc = "`0`"]
    QSPI1_SD3,
    #[doc = "`1`"]
    IOF1,
}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN7W::QSPI1_SD3 => false,
            PIN7W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_sd3(self) -> &'a mut W {
        self.variant(PIN7W::QSPI1_SD3)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN7W::IOF1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8W {
    #[doc = "`0`"]
    QSPI1_SS1,
    #[doc = "`1`"]
    IOF1,
}
impl PIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN8W::QSPI1_SS1 => false,
            PIN8W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN8W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_ss1(self) -> &'a mut W {
        self.variant(PIN8W::QSPI1_SS1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN8W::IOF1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9W {
    #[doc = "`0`"]
    QSPI1_SS2,
    #[doc = "`1`"]
    IOF1,
}
impl PIN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN9W::QSPI1_SS2 => false,
            PIN9W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN9W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_ss2(self) -> &'a mut W {
        self.variant(PIN9W::QSPI1_SS2)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN9W::IOF1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10W {
    #[doc = "`0`"]
    QSPI1_SS3,
    #[doc = "`1`"]
    PWM2_0,
}
impl PIN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN10W::QSPI1_SS3 => false,
            PIN10W::PWM2_0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN10W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi1_ss3(self) -> &'a mut W {
        self.variant(PIN10W::QSPI1_SS3)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm2_0(self) -> &'a mut W {
        self.variant(PIN10W::PWM2_0)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM2_1,
}
impl PIN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN11W::IOF0 => false,
            PIN11W::PWM2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN11W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN11W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm2_1(self) -> &'a mut W {
        self.variant(PIN11W::PWM2_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM2_2,
}
impl PIN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN12W::IOF0 => false,
            PIN12W::PWM2_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN12W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN12W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm2_2(self) -> &'a mut W {
        self.variant(PIN12W::PWM2_2)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM2_3,
}
impl PIN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN13W::IOF0 => false,
            PIN13W::PWM2_3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN13W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN13W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm2_3(self) -> &'a mut W {
        self.variant(PIN13W::PWM2_3)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    IOF1,
}
impl PIN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN14W::IOF0 => false,
            PIN14W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN14W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN14W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN14W::IOF1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    IOF1,
}
impl PIN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN15W::IOF0 => false,
            PIN15W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN15W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN15W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN15W::IOF1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16W {
    #[doc = "`0`"]
    UART0_RX,
    #[doc = "`1`"]
    IOF1,
}
impl PIN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN16W::UART0_RX => false,
            PIN16W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN16W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PIN16W::UART0_RX)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN16W::IOF1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17W {
    #[doc = "`0`"]
    UART0_TX,
    #[doc = "`1`"]
    IOF1,
}
impl PIN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN17W::UART0_TX => false,
            PIN17W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN17W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PIN17W::UART0_TX)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN17W::IOF1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    IOF1,
}
impl PIN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN18W::IOF0 => false,
            PIN18W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN18W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN18W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN18W::IOF1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM1_1,
}
impl PIN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN19W::IOF0 => false,
            PIN19W::PWM1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN19W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN19W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm1_1(self) -> &'a mut W {
        self.variant(PIN19W::PWM1_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM1_0,
}
impl PIN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN20W::IOF0 => false,
            PIN20W::PWM1_0 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN20W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN20W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm1_0(self) -> &'a mut W {
        self.variant(PIN20W::PWM1_0)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM1_2,
}
impl PIN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN21W::IOF0 => false,
            PIN21W::PWM1_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN21W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN21W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm1_2(self) -> &'a mut W {
        self.variant(PIN21W::PWM1_2)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    PWM1_3,
}
impl PIN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN22W::IOF0 => false,
            PIN22W::PWM1_3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN22W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN22W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn pwm1_3(self) -> &'a mut W {
        self.variant(PIN22W::PWM1_3)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23W {
    #[doc = "`0`"]
    IOF0,
    #[doc = "`1`"]
    IOF1,
}
impl PIN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN23W::IOF0 => false,
            PIN23W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN23W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN23W::IOF0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN23W::IOF1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24W {
    #[doc = "`0`"]
    UART1_RX,
    #[doc = "`1`"]
    IOF1,
}
impl PIN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN24W::UART1_RX => false,
            PIN24W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN24W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PIN24W::UART1_RX)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN24W::IOF1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25W {
    #[doc = "`0`"]
    UART1_TX,
    #[doc = "`1`"]
    IOF1,
}
impl PIN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN25W::UART1_TX => false,
            PIN25W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN25W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PIN25W::UART1_TX)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN25W::IOF1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26W {
    #[doc = "`0`"]
    QSPI2_SS,
    #[doc = "`1`"]
    IOF1,
}
impl PIN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN26W::QSPI2_SS => false,
            PIN26W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN26W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_ss(self) -> &'a mut W {
        self.variant(PIN26W::QSPI2_SS)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN26W::IOF1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27W {
    #[doc = "`0`"]
    QSPI2_SD0,
    #[doc = "`1`"]
    IOF1,
}
impl PIN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN27W::QSPI2_SD0 => false,
            PIN27W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN27W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_sd0(self) -> &'a mut W {
        self.variant(PIN27W::QSPI2_SD0)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN27W::IOF1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28W {
    #[doc = "`0`"]
    QSPI2_SD1,
    #[doc = "`1`"]
    IOF1,
}
impl PIN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN28W::QSPI2_SD1 => false,
            PIN28W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN28W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_sd1(self) -> &'a mut W {
        self.variant(PIN28W::QSPI2_SD1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN28W::IOF1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29W {
    #[doc = "`0`"]
    QSPI2_SCK,
    #[doc = "`1`"]
    IOF1,
}
impl PIN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN29W::QSPI2_SCK => false,
            PIN29W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN29W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_sck(self) -> &'a mut W {
        self.variant(PIN29W::QSPI2_SCK)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN29W::IOF1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30W {
    #[doc = "`0`"]
    QSPI2_SD2,
    #[doc = "`1`"]
    IOF1,
}
impl PIN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN30W::QSPI2_SD2 => false,
            PIN30W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN30W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_sd2(self) -> &'a mut W {
        self.variant(PIN30W::QSPI2_SD2)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN30W::IOF1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31W {
    #[doc = "`0`"]
    QSPI2_SD3,
    #[doc = "`1`"]
    IOF1,
}
impl PIN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN31W::QSPI2_SD3 => false,
            PIN31W::IOF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN31W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn qspi2_sd3(self) -> &'a mut W {
        self.variant(PIN31W::QSPI2_SD3)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN31W::IOF1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0"]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pin8(&self) -> PIN8R {
        PIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn pin9(&self) -> PIN9R {
        PIN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn pin10(&self) -> PIN10R {
        PIN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn pin11(&self) -> PIN11R {
        PIN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pin12(&self) -> PIN12R {
        PIN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn pin13(&self) -> PIN13R {
        PIN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn pin14(&self) -> PIN14R {
        PIN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn pin15(&self) -> PIN15R {
        PIN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn pin16(&self) -> PIN16R {
        PIN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn pin17(&self) -> PIN17R {
        PIN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn pin18(&self) -> PIN18R {
        PIN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn pin19(&self) -> PIN19R {
        PIN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn pin20(&self) -> PIN20R {
        PIN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn pin21(&self) -> PIN21R {
        PIN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn pin22(&self) -> PIN22R {
        PIN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn pin23(&self) -> PIN23R {
        PIN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn pin24(&self) -> PIN24R {
        PIN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn pin25(&self) -> PIN25R {
        PIN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn pin26(&self) -> PIN26R {
        PIN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn pin27(&self) -> PIN27R {
        PIN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn pin28(&self) -> PIN28R {
        PIN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn pin29(&self) -> PIN29R {
        PIN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30"]
    #[inline]
    pub fn pin30(&self) -> PIN30R {
        PIN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn pin31(&self) -> PIN31R {
        PIN31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0"]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pin8(&mut self) -> _PIN8W {
        _PIN8W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn pin9(&mut self) -> _PIN9W {
        _PIN9W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn pin10(&mut self) -> _PIN10W {
        _PIN10W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn pin11(&mut self) -> _PIN11W {
        _PIN11W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pin12(&mut self) -> _PIN12W {
        _PIN12W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn pin13(&mut self) -> _PIN13W {
        _PIN13W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn pin14(&mut self) -> _PIN14W {
        _PIN14W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn pin15(&mut self) -> _PIN15W {
        _PIN15W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn pin16(&mut self) -> _PIN16W {
        _PIN16W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn pin17(&mut self) -> _PIN17W {
        _PIN17W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn pin18(&mut self) -> _PIN18W {
        _PIN18W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn pin19(&mut self) -> _PIN19W {
        _PIN19W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn pin20(&mut self) -> _PIN20W {
        _PIN20W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn pin21(&mut self) -> _PIN21W {
        _PIN21W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn pin22(&mut self) -> _PIN22W {
        _PIN22W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline]
    pub fn pin23(&mut self) -> _PIN23W {
        _PIN23W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn pin24(&mut self) -> _PIN24W {
        _PIN24W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn pin25(&mut self) -> _PIN25W {
        _PIN25W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn pin26(&mut self) -> _PIN26W {
        _PIN26W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn pin27(&mut self) -> _PIN27W {
        _PIN27W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn pin28(&mut self) -> _PIN28W {
        _PIN28W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn pin29(&mut self) -> _PIN29W {
        _PIN29W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline]
    pub fn pin30(&mut self) -> _PIN30W {
        _PIN30W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn pin31(&mut self) -> _PIN31W {
        _PIN31W { w: self }
    }
}
