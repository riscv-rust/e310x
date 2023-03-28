#[doc = "Register `iof_sel` reader"]
pub struct R(crate::R<IOF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iof_sel` writer"]
pub struct W(crate::W<IOF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IOF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - "]
pub type PIN0_R = crate::BitReader<PIN0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN0_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM0_0 = 1,
}
impl From<PIN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN0_A {
        match self.bits {
            false => PIN0_A::IOF0,
            true => PIN0_A::PWM0_0,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN0_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM0_0`"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == PIN0_A::PWM0_0
    }
}
#[doc = "Field `pin0` writer - "]
pub type PIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN0_A, O>;
impl<'a, const O: u8> PIN0_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN0_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut W {
        self.variant(PIN0_A::PWM0_0)
    }
}
#[doc = "Field `pin1` reader - "]
pub type PIN1_R = crate::BitReader<PIN1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN1_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM0_1 = 1,
}
impl From<PIN1_A> for bool {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN1_A {
        match self.bits {
            false => PIN1_A::IOF0,
            true => PIN1_A::PWM0_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN1_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM0_1`"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == PIN1_A::PWM0_1
    }
}
#[doc = "Field `pin1` writer - "]
pub type PIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN1_A, O>;
impl<'a, const O: u8> PIN1_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN1_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut W {
        self.variant(PIN1_A::PWM0_1)
    }
}
#[doc = "Field `pin2` reader - "]
pub type PIN2_R = crate::BitReader<PIN2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN2_A {
    #[doc = "0: `0`"]
    QSPI1_SS0 = 0,
    #[doc = "1: `1`"]
    PWM0_2 = 1,
}
impl From<PIN2_A> for bool {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN2_A {
        match self.bits {
            false => PIN2_A::QSPI1_SS0,
            true => PIN2_A::PWM0_2,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS0`"]
    #[inline(always)]
    pub fn is_qspi1_ss0(&self) -> bool {
        *self == PIN2_A::QSPI1_SS0
    }
    #[doc = "Checks if the value of the field is `PWM0_2`"]
    #[inline(always)]
    pub fn is_pwm0_2(&self) -> bool {
        *self == PIN2_A::PWM0_2
    }
}
#[doc = "Field `pin2` writer - "]
pub type PIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN2_A, O>;
impl<'a, const O: u8> PIN2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss0(self) -> &'a mut W {
        self.variant(PIN2_A::QSPI1_SS0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_2(self) -> &'a mut W {
        self.variant(PIN2_A::PWM0_2)
    }
}
#[doc = "Field `pin3` reader - "]
pub type PIN3_R = crate::BitReader<PIN3_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN3_A {
    #[doc = "0: `0`"]
    QSPI1_SD0 = 0,
    #[doc = "1: `1`"]
    PWM0_3 = 1,
}
impl From<PIN3_A> for bool {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN3_A {
        match self.bits {
            false => PIN3_A::QSPI1_SD0,
            true => PIN3_A::PWM0_3,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD0`"]
    #[inline(always)]
    pub fn is_qspi1_sd0(&self) -> bool {
        *self == PIN3_A::QSPI1_SD0
    }
    #[doc = "Checks if the value of the field is `PWM0_3`"]
    #[inline(always)]
    pub fn is_pwm0_3(&self) -> bool {
        *self == PIN3_A::PWM0_3
    }
}
#[doc = "Field `pin3` writer - "]
pub type PIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN3_A, O>;
impl<'a, const O: u8> PIN3_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd0(self) -> &'a mut W {
        self.variant(PIN3_A::QSPI1_SD0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_3(self) -> &'a mut W {
        self.variant(PIN3_A::PWM0_3)
    }
}
#[doc = "Field `pin4` reader - "]
pub type PIN4_R = crate::BitReader<PIN4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN4_A {
    #[doc = "0: `0`"]
    QSPI1_SD1 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN4_A> for bool {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN4_A {
        match self.bits {
            false => PIN4_A::QSPI1_SD1,
            true => PIN4_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD1`"]
    #[inline(always)]
    pub fn is_qspi1_sd1(&self) -> bool {
        *self == PIN4_A::QSPI1_SD1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN4_A::IOF1
    }
}
#[doc = "Field `pin4` writer - "]
pub type PIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN4_A, O>;
impl<'a, const O: u8> PIN4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd1(self) -> &'a mut W {
        self.variant(PIN4_A::QSPI1_SD1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN4_A::IOF1)
    }
}
#[doc = "Field `pin5` reader - "]
pub type PIN5_R = crate::BitReader<PIN5_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN5_A {
    #[doc = "0: `0`"]
    QSPI1_SCK = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN5_A> for bool {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN5_A {
        match self.bits {
            false => PIN5_A::QSPI1_SCK,
            true => PIN5_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SCK`"]
    #[inline(always)]
    pub fn is_qspi1_sck(&self) -> bool {
        *self == PIN5_A::QSPI1_SCK
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN5_A::IOF1
    }
}
#[doc = "Field `pin5` writer - "]
pub type PIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN5_A, O>;
impl<'a, const O: u8> PIN5_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sck(self) -> &'a mut W {
        self.variant(PIN5_A::QSPI1_SCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN5_A::IOF1)
    }
}
#[doc = "Field `pin6` reader - "]
pub type PIN6_R = crate::BitReader<PIN6_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN6_A {
    #[doc = "0: `0`"]
    QSPI1_SD2 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN6_A> for bool {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN6_A {
        match self.bits {
            false => PIN6_A::QSPI1_SD2,
            true => PIN6_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD2`"]
    #[inline(always)]
    pub fn is_qspi1_sd2(&self) -> bool {
        *self == PIN6_A::QSPI1_SD2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN6_A::IOF1
    }
}
#[doc = "Field `pin6` writer - "]
pub type PIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN6_A, O>;
impl<'a, const O: u8> PIN6_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd2(self) -> &'a mut W {
        self.variant(PIN6_A::QSPI1_SD2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN6_A::IOF1)
    }
}
#[doc = "Field `pin7` reader - "]
pub type PIN7_R = crate::BitReader<PIN7_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN7_A {
    #[doc = "0: `0`"]
    QSPI1_SD3 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN7_A> for bool {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN7_A {
        match self.bits {
            false => PIN7_A::QSPI1_SD3,
            true => PIN7_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SD3`"]
    #[inline(always)]
    pub fn is_qspi1_sd3(&self) -> bool {
        *self == PIN7_A::QSPI1_SD3
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN7_A::IOF1
    }
}
#[doc = "Field `pin7` writer - "]
pub type PIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN7_A, O>;
impl<'a, const O: u8> PIN7_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd3(self) -> &'a mut W {
        self.variant(PIN7_A::QSPI1_SD3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN7_A::IOF1)
    }
}
#[doc = "Field `pin8` reader - "]
pub type PIN8_R = crate::BitReader<PIN8_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN8_A {
    #[doc = "0: `0`"]
    QSPI1_SS1 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN8_A> for bool {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN8_A {
        match self.bits {
            false => PIN8_A::QSPI1_SS1,
            true => PIN8_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS1`"]
    #[inline(always)]
    pub fn is_qspi1_ss1(&self) -> bool {
        *self == PIN8_A::QSPI1_SS1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN8_A::IOF1
    }
}
#[doc = "Field `pin8` writer - "]
pub type PIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN8_A, O>;
impl<'a, const O: u8> PIN8_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss1(self) -> &'a mut W {
        self.variant(PIN8_A::QSPI1_SS1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN8_A::IOF1)
    }
}
#[doc = "Field `pin9` reader - "]
pub type PIN9_R = crate::BitReader<PIN9_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN9_A {
    #[doc = "0: `0`"]
    QSPI1_SS2 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN9_A> for bool {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN9_A {
        match self.bits {
            false => PIN9_A::QSPI1_SS2,
            true => PIN9_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS2`"]
    #[inline(always)]
    pub fn is_qspi1_ss2(&self) -> bool {
        *self == PIN9_A::QSPI1_SS2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN9_A::IOF1
    }
}
#[doc = "Field `pin9` writer - "]
pub type PIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN9_A, O>;
impl<'a, const O: u8> PIN9_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss2(self) -> &'a mut W {
        self.variant(PIN9_A::QSPI1_SS2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN9_A::IOF1)
    }
}
#[doc = "Field `pin10` reader - "]
pub type PIN10_R = crate::BitReader<PIN10_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN10_A {
    #[doc = "0: `0`"]
    QSPI1_SS3 = 0,
    #[doc = "1: `1`"]
    PWM2_0 = 1,
}
impl From<PIN10_A> for bool {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN10_A {
        match self.bits {
            false => PIN10_A::QSPI1_SS3,
            true => PIN10_A::PWM2_0,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI1_SS3`"]
    #[inline(always)]
    pub fn is_qspi1_ss3(&self) -> bool {
        *self == PIN10_A::QSPI1_SS3
    }
    #[doc = "Checks if the value of the field is `PWM2_0`"]
    #[inline(always)]
    pub fn is_pwm2_0(&self) -> bool {
        *self == PIN10_A::PWM2_0
    }
}
#[doc = "Field `pin10` writer - "]
pub type PIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN10_A, O>;
impl<'a, const O: u8> PIN10_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss3(self) -> &'a mut W {
        self.variant(PIN10_A::QSPI1_SS3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_0(self) -> &'a mut W {
        self.variant(PIN10_A::PWM2_0)
    }
}
#[doc = "Field `pin11` reader - "]
pub type PIN11_R = crate::BitReader<PIN11_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN11_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM2_1 = 1,
}
impl From<PIN11_A> for bool {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN11_A {
        match self.bits {
            false => PIN11_A::IOF0,
            true => PIN11_A::PWM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN11_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_1`"]
    #[inline(always)]
    pub fn is_pwm2_1(&self) -> bool {
        *self == PIN11_A::PWM2_1
    }
}
#[doc = "Field `pin11` writer - "]
pub type PIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN11_A, O>;
impl<'a, const O: u8> PIN11_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN11_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_1(self) -> &'a mut W {
        self.variant(PIN11_A::PWM2_1)
    }
}
#[doc = "Field `pin12` reader - "]
pub type PIN12_R = crate::BitReader<PIN12_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN12_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM2_2 = 1,
}
impl From<PIN12_A> for bool {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN12_A {
        match self.bits {
            false => PIN12_A::IOF0,
            true => PIN12_A::PWM2_2,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN12_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_2`"]
    #[inline(always)]
    pub fn is_pwm2_2(&self) -> bool {
        *self == PIN12_A::PWM2_2
    }
}
#[doc = "Field `pin12` writer - "]
pub type PIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN12_A, O>;
impl<'a, const O: u8> PIN12_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN12_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_2(self) -> &'a mut W {
        self.variant(PIN12_A::PWM2_2)
    }
}
#[doc = "Field `pin13` reader - "]
pub type PIN13_R = crate::BitReader<PIN13_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN13_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM2_3 = 1,
}
impl From<PIN13_A> for bool {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN13_A {
        match self.bits {
            false => PIN13_A::IOF0,
            true => PIN13_A::PWM2_3,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN13_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM2_3`"]
    #[inline(always)]
    pub fn is_pwm2_3(&self) -> bool {
        *self == PIN13_A::PWM2_3
    }
}
#[doc = "Field `pin13` writer - "]
pub type PIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN13_A, O>;
impl<'a, const O: u8> PIN13_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN13_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_3(self) -> &'a mut W {
        self.variant(PIN13_A::PWM2_3)
    }
}
#[doc = "Field `pin14` reader - "]
pub type PIN14_R = crate::BitReader<PIN14_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN14_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN14_A> for bool {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN14_A {
        match self.bits {
            false => PIN14_A::IOF0,
            true => PIN14_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN14_A::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN14_A::IOF1
    }
}
#[doc = "Field `pin14` writer - "]
pub type PIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN14_A, O>;
impl<'a, const O: u8> PIN14_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN14_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN14_A::IOF1)
    }
}
#[doc = "Field `pin15` reader - "]
pub type PIN15_R = crate::BitReader<PIN15_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN15_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN15_A> for bool {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN15_A {
        match self.bits {
            false => PIN15_A::IOF0,
            true => PIN15_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN15_A::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN15_A::IOF1
    }
}
#[doc = "Field `pin15` writer - "]
pub type PIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN15_A, O>;
impl<'a, const O: u8> PIN15_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN15_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN15_A::IOF1)
    }
}
#[doc = "Field `pin16` reader - "]
pub type PIN16_R = crate::BitReader<PIN16_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN16_A {
    #[doc = "0: `0`"]
    UART0_RX = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN16_A> for bool {
    #[inline(always)]
    fn from(variant: PIN16_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN16_A {
        match self.bits {
            false => PIN16_A::UART0_RX,
            true => PIN16_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PIN16_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN16_A::IOF1
    }
}
#[doc = "Field `pin16` writer - "]
pub type PIN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN16_A, O>;
impl<'a, const O: u8> PIN16_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PIN16_A::UART0_RX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN16_A::IOF1)
    }
}
#[doc = "Field `pin17` reader - "]
pub type PIN17_R = crate::BitReader<PIN17_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN17_A {
    #[doc = "0: `0`"]
    UART0_TX = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN17_A> for bool {
    #[inline(always)]
    fn from(variant: PIN17_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN17_A {
        match self.bits {
            false => PIN17_A::UART0_TX,
            true => PIN17_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PIN17_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN17_A::IOF1
    }
}
#[doc = "Field `pin17` writer - "]
pub type PIN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN17_A, O>;
impl<'a, const O: u8> PIN17_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PIN17_A::UART0_TX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN17_A::IOF1)
    }
}
#[doc = "Field `pin18` reader - "]
pub type PIN18_R = crate::BitReader<PIN18_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN18_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN18_A> for bool {
    #[inline(always)]
    fn from(variant: PIN18_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN18_A {
        match self.bits {
            false => PIN18_A::IOF0,
            true => PIN18_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN18_A::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN18_A::IOF1
    }
}
#[doc = "Field `pin18` writer - "]
pub type PIN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN18_A, O>;
impl<'a, const O: u8> PIN18_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN18_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN18_A::IOF1)
    }
}
#[doc = "Field `pin19` reader - "]
pub type PIN19_R = crate::BitReader<PIN19_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN19_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM1_1 = 1,
}
impl From<PIN19_A> for bool {
    #[inline(always)]
    fn from(variant: PIN19_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN19_A {
        match self.bits {
            false => PIN19_A::IOF0,
            true => PIN19_A::PWM1_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN19_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_1`"]
    #[inline(always)]
    pub fn is_pwm1_1(&self) -> bool {
        *self == PIN19_A::PWM1_1
    }
}
#[doc = "Field `pin19` writer - "]
pub type PIN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN19_A, O>;
impl<'a, const O: u8> PIN19_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN19_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_1(self) -> &'a mut W {
        self.variant(PIN19_A::PWM1_1)
    }
}
#[doc = "Field `pin20` reader - "]
pub type PIN20_R = crate::BitReader<PIN20_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN20_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM1_0 = 1,
}
impl From<PIN20_A> for bool {
    #[inline(always)]
    fn from(variant: PIN20_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN20_A {
        match self.bits {
            false => PIN20_A::IOF0,
            true => PIN20_A::PWM1_0,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN20_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_0`"]
    #[inline(always)]
    pub fn is_pwm1_0(&self) -> bool {
        *self == PIN20_A::PWM1_0
    }
}
#[doc = "Field `pin20` writer - "]
pub type PIN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN20_A, O>;
impl<'a, const O: u8> PIN20_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN20_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_0(self) -> &'a mut W {
        self.variant(PIN20_A::PWM1_0)
    }
}
#[doc = "Field `pin21` reader - "]
pub type PIN21_R = crate::BitReader<PIN21_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN21_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM1_2 = 1,
}
impl From<PIN21_A> for bool {
    #[inline(always)]
    fn from(variant: PIN21_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN21_A {
        match self.bits {
            false => PIN21_A::IOF0,
            true => PIN21_A::PWM1_2,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN21_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_2`"]
    #[inline(always)]
    pub fn is_pwm1_2(&self) -> bool {
        *self == PIN21_A::PWM1_2
    }
}
#[doc = "Field `pin21` writer - "]
pub type PIN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN21_A, O>;
impl<'a, const O: u8> PIN21_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN21_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_2(self) -> &'a mut W {
        self.variant(PIN21_A::PWM1_2)
    }
}
#[doc = "Field `pin22` reader - "]
pub type PIN22_R = crate::BitReader<PIN22_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN22_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    PWM1_3 = 1,
}
impl From<PIN22_A> for bool {
    #[inline(always)]
    fn from(variant: PIN22_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN22_A {
        match self.bits {
            false => PIN22_A::IOF0,
            true => PIN22_A::PWM1_3,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN22_A::IOF0
    }
    #[doc = "Checks if the value of the field is `PWM1_3`"]
    #[inline(always)]
    pub fn is_pwm1_3(&self) -> bool {
        *self == PIN22_A::PWM1_3
    }
}
#[doc = "Field `pin22` writer - "]
pub type PIN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN22_A, O>;
impl<'a, const O: u8> PIN22_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN22_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_3(self) -> &'a mut W {
        self.variant(PIN22_A::PWM1_3)
    }
}
#[doc = "Field `pin23` reader - "]
pub type PIN23_R = crate::BitReader<PIN23_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN23_A {
    #[doc = "0: `0`"]
    IOF0 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN23_A> for bool {
    #[inline(always)]
    fn from(variant: PIN23_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN23_A {
        match self.bits {
            false => PIN23_A::IOF0,
            true => PIN23_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `IOF0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == PIN23_A::IOF0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN23_A::IOF1
    }
}
#[doc = "Field `pin23` writer - "]
pub type PIN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN23_A, O>;
impl<'a, const O: u8> PIN23_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut W {
        self.variant(PIN23_A::IOF0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN23_A::IOF1)
    }
}
#[doc = "Field `pin24` reader - "]
pub type PIN24_R = crate::BitReader<PIN24_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN24_A {
    #[doc = "0: `0`"]
    UART1_RX = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN24_A> for bool {
    #[inline(always)]
    fn from(variant: PIN24_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN24_A {
        match self.bits {
            false => PIN24_A::UART1_RX,
            true => PIN24_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PIN24_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN24_A::IOF1
    }
}
#[doc = "Field `pin24` writer - "]
pub type PIN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN24_A, O>;
impl<'a, const O: u8> PIN24_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PIN24_A::UART1_RX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN24_A::IOF1)
    }
}
#[doc = "Field `pin25` reader - "]
pub type PIN25_R = crate::BitReader<PIN25_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN25_A {
    #[doc = "0: `0`"]
    UART1_TX = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN25_A> for bool {
    #[inline(always)]
    fn from(variant: PIN25_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN25_A {
        match self.bits {
            false => PIN25_A::UART1_TX,
            true => PIN25_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PIN25_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN25_A::IOF1
    }
}
#[doc = "Field `pin25` writer - "]
pub type PIN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN25_A, O>;
impl<'a, const O: u8> PIN25_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PIN25_A::UART1_TX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN25_A::IOF1)
    }
}
#[doc = "Field `pin26` reader - "]
pub type PIN26_R = crate::BitReader<PIN26_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN26_A {
    #[doc = "0: `0`"]
    QSPI2_SS = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN26_A> for bool {
    #[inline(always)]
    fn from(variant: PIN26_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN26_A {
        match self.bits {
            false => PIN26_A::QSPI2_SS,
            true => PIN26_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SS`"]
    #[inline(always)]
    pub fn is_qspi2_ss(&self) -> bool {
        *self == PIN26_A::QSPI2_SS
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN26_A::IOF1
    }
}
#[doc = "Field `pin26` writer - "]
pub type PIN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN26_A, O>;
impl<'a, const O: u8> PIN26_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_ss(self) -> &'a mut W {
        self.variant(PIN26_A::QSPI2_SS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN26_A::IOF1)
    }
}
#[doc = "Field `pin27` reader - "]
pub type PIN27_R = crate::BitReader<PIN27_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN27_A {
    #[doc = "0: `0`"]
    QSPI2_SD0 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN27_A> for bool {
    #[inline(always)]
    fn from(variant: PIN27_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN27_A {
        match self.bits {
            false => PIN27_A::QSPI2_SD0,
            true => PIN27_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD0`"]
    #[inline(always)]
    pub fn is_qspi2_sd0(&self) -> bool {
        *self == PIN27_A::QSPI2_SD0
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN27_A::IOF1
    }
}
#[doc = "Field `pin27` writer - "]
pub type PIN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN27_A, O>;
impl<'a, const O: u8> PIN27_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd0(self) -> &'a mut W {
        self.variant(PIN27_A::QSPI2_SD0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN27_A::IOF1)
    }
}
#[doc = "Field `pin28` reader - "]
pub type PIN28_R = crate::BitReader<PIN28_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN28_A {
    #[doc = "0: `0`"]
    QSPI2_SD1 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN28_A> for bool {
    #[inline(always)]
    fn from(variant: PIN28_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN28_A {
        match self.bits {
            false => PIN28_A::QSPI2_SD1,
            true => PIN28_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD1`"]
    #[inline(always)]
    pub fn is_qspi2_sd1(&self) -> bool {
        *self == PIN28_A::QSPI2_SD1
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN28_A::IOF1
    }
}
#[doc = "Field `pin28` writer - "]
pub type PIN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN28_A, O>;
impl<'a, const O: u8> PIN28_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd1(self) -> &'a mut W {
        self.variant(PIN28_A::QSPI2_SD1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN28_A::IOF1)
    }
}
#[doc = "Field `pin29` reader - "]
pub type PIN29_R = crate::BitReader<PIN29_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN29_A {
    #[doc = "0: `0`"]
    QSPI2_SCK = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN29_A> for bool {
    #[inline(always)]
    fn from(variant: PIN29_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN29_A {
        match self.bits {
            false => PIN29_A::QSPI2_SCK,
            true => PIN29_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SCK`"]
    #[inline(always)]
    pub fn is_qspi2_sck(&self) -> bool {
        *self == PIN29_A::QSPI2_SCK
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN29_A::IOF1
    }
}
#[doc = "Field `pin29` writer - "]
pub type PIN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN29_A, O>;
impl<'a, const O: u8> PIN29_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sck(self) -> &'a mut W {
        self.variant(PIN29_A::QSPI2_SCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN29_A::IOF1)
    }
}
#[doc = "Field `pin30` reader - "]
pub type PIN30_R = crate::BitReader<PIN30_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN30_A {
    #[doc = "0: `0`"]
    QSPI2_SD2 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN30_A> for bool {
    #[inline(always)]
    fn from(variant: PIN30_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN30_A {
        match self.bits {
            false => PIN30_A::QSPI2_SD2,
            true => PIN30_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD2`"]
    #[inline(always)]
    pub fn is_qspi2_sd2(&self) -> bool {
        *self == PIN30_A::QSPI2_SD2
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN30_A::IOF1
    }
}
#[doc = "Field `pin30` writer - "]
pub type PIN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN30_A, O>;
impl<'a, const O: u8> PIN30_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd2(self) -> &'a mut W {
        self.variant(PIN30_A::QSPI2_SD2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN30_A::IOF1)
    }
}
#[doc = "Field `pin31` reader - "]
pub type PIN31_R = crate::BitReader<PIN31_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN31_A {
    #[doc = "0: `0`"]
    QSPI2_SD3 = 0,
    #[doc = "1: `1`"]
    IOF1 = 1,
}
impl From<PIN31_A> for bool {
    #[inline(always)]
    fn from(variant: PIN31_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN31_A {
        match self.bits {
            false => PIN31_A::QSPI2_SD3,
            true => PIN31_A::IOF1,
        }
    }
    #[doc = "Checks if the value of the field is `QSPI2_SD3`"]
    #[inline(always)]
    pub fn is_qspi2_sd3(&self) -> bool {
        *self == PIN31_A::QSPI2_SD3
    }
    #[doc = "Checks if the value of the field is `IOF1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == PIN31_A::IOF1
    }
}
#[doc = "Field `pin31` writer - "]
pub type PIN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOF_SEL_SPEC, PIN31_A, O>;
impl<'a, const O: u8> PIN31_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd3(self) -> &'a mut W {
        self.variant(PIN31_A::QSPI2_SD3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut W {
        self.variant(PIN31_A::IOF1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W<1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W<2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W<3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W<4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W<5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W<6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W<7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W<8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W<9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W<10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W<11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W<12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W<13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W<14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W<15> {
        PIN15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pin16(&mut self) -> PIN16_W<16> {
        PIN16_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pin17(&mut self) -> PIN17_W<17> {
        PIN17_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pin18(&mut self) -> PIN18_W<18> {
        PIN18_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pin19(&mut self) -> PIN19_W<19> {
        PIN19_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pin20(&mut self) -> PIN20_W<20> {
        PIN20_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pin21(&mut self) -> PIN21_W<21> {
        PIN21_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pin22(&mut self) -> PIN22_W<22> {
        PIN22_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pin23(&mut self) -> PIN23_W<23> {
        PIN23_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pin24(&mut self) -> PIN24_W<24> {
        PIN24_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pin25(&mut self) -> PIN25_W<25> {
        PIN25_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pin26(&mut self) -> PIN26_W<26> {
        PIN26_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin27(&mut self) -> PIN27_W<27> {
        PIN27_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pin28(&mut self) -> PIN28_W<28> {
        PIN28_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pin29(&mut self) -> PIN29_W<29> {
        PIN29_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pin30(&mut self) -> PIN30_W<30> {
        PIN30_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin31(&mut self) -> PIN31_W<31> {
        PIN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HW I/O Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iof_sel](index.html) module"]
pub struct IOF_SEL_SPEC;
impl crate::RegisterSpec for IOF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iof_sel::R](R) reader structure"]
impl crate::Readable for IOF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iof_sel::W](W) writer structure"]
impl crate::Writable for IOF_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets iof_sel to value 0"]
impl crate::Resettable for IOF_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
