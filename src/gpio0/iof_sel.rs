#[doc = "Register `iof_sel` reader"]
pub type R = crate::R<IofSelSpec>;
#[doc = "Register `iof_sel` writer"]
pub type W = crate::W<IofSelSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm0_0 = 1,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin0` reader - "]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            false => Pin0::Iof0,
            true => Pin0::Pwm0_0,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin0::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == Pin0::Pwm0_0
    }
}
#[doc = "Field `pin0` writer - "]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Pwm0_0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm0_1 = 1,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin1` reader - "]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            false => Pin1::Iof0,
            true => Pin1::Pwm0_1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin1::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == Pin1::Pwm0_1
    }
}
#[doc = "Field `pin1` writer - "]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Pwm0_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "0: `0`"]
    Qspi1Ss0 = 0,
    #[doc = "1: `1`"]
    Pwm0_2 = 1,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin2` reader - "]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            false => Pin2::Qspi1Ss0,
            true => Pin2::Pwm0_2,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_ss0(&self) -> bool {
        *self == Pin2::Qspi1Ss0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm0_2(&self) -> bool {
        *self == Pin2::Pwm0_2
    }
}
#[doc = "Field `pin2` writer - "]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Qspi1Ss0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Pwm0_2)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "0: `0`"]
    Qspi1Sd0 = 0,
    #[doc = "1: `1`"]
    Pwm0_3 = 1,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin3` reader - "]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            false => Pin3::Qspi1Sd0,
            true => Pin3::Pwm0_3,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_sd0(&self) -> bool {
        *self == Pin3::Qspi1Sd0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm0_3(&self) -> bool {
        *self == Pin3::Pwm0_3
    }
}
#[doc = "Field `pin3` writer - "]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Qspi1Sd0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Pwm0_3)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "0: `0`"]
    Qspi1Sd1 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin4` reader - "]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            false => Pin4::Qspi1Sd1,
            true => Pin4::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_sd1(&self) -> bool {
        *self == Pin4::Qspi1Sd1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin4::Iof1
    }
}
#[doc = "Field `pin4` writer - "]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Qspi1Sd1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "0: `0`"]
    Qspi1Sck = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin5` reader - "]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            false => Pin5::Qspi1Sck,
            true => Pin5::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_sck(&self) -> bool {
        *self == Pin5::Qspi1Sck
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin5::Iof1
    }
}
#[doc = "Field `pin5` writer - "]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Qspi1Sck)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "0: `0`"]
    Qspi1Sd2 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin6` reader - "]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            false => Pin6::Qspi1Sd2,
            true => Pin6::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_sd2(&self) -> bool {
        *self == Pin6::Qspi1Sd2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin6::Iof1
    }
}
#[doc = "Field `pin6` writer - "]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Qspi1Sd2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "0: `0`"]
    Qspi1Sd3 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin7` reader - "]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            false => Pin7::Qspi1Sd3,
            true => Pin7::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_sd3(&self) -> bool {
        *self == Pin7::Qspi1Sd3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin7::Iof1
    }
}
#[doc = "Field `pin7` writer - "]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_sd3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Qspi1Sd3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "0: `0`"]
    Qspi1Ss1 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin8` reader - "]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            false => Pin8::Qspi1Ss1,
            true => Pin8::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_ss1(&self) -> bool {
        *self == Pin8::Qspi1Ss1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin8::Iof1
    }
}
#[doc = "Field `pin8` writer - "]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Qspi1Ss1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "0: `0`"]
    Qspi1Ss2 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin9` reader - "]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            false => Pin9::Qspi1Ss2,
            true => Pin9::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_ss2(&self) -> bool {
        *self == Pin9::Qspi1Ss2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin9::Iof1
    }
}
#[doc = "Field `pin9` writer - "]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Qspi1Ss2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "0: `0`"]
    Qspi1Ss3 = 0,
    #[doc = "1: `1`"]
    Pwm2_0 = 1,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin10` reader - "]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            false => Pin10::Qspi1Ss3,
            true => Pin10::Pwm2_0,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi1_ss3(&self) -> bool {
        *self == Pin10::Qspi1Ss3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm2_0(&self) -> bool {
        *self == Pin10::Pwm2_0
    }
}
#[doc = "Field `pin10` writer - "]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi1_ss3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Qspi1Ss3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Pwm2_0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm2_1 = 1,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin11` reader - "]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            false => Pin11::Iof0,
            true => Pin11::Pwm2_1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin11::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm2_1(&self) -> bool {
        *self == Pin11::Pwm2_1
    }
}
#[doc = "Field `pin11` writer - "]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Pwm2_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm2_2 = 1,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin12` reader - "]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            false => Pin12::Iof0,
            true => Pin12::Pwm2_2,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin12::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm2_2(&self) -> bool {
        *self == Pin12::Pwm2_2
    }
}
#[doc = "Field `pin12` writer - "]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Pwm2_2)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm2_3 = 1,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin13` reader - "]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            false => Pin13::Iof0,
            true => Pin13::Pwm2_3,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin13::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm2_3(&self) -> bool {
        *self == Pin13::Pwm2_3
    }
}
#[doc = "Field `pin13` writer - "]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Pwm2_3)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin14` reader - "]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            false => Pin14::Iof0,
            true => Pin14::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin14::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin14::Iof1
    }
}
#[doc = "Field `pin14` writer - "]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin15` reader - "]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            false => Pin15::Iof0,
            true => Pin15::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin15::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin15::Iof1
    }
}
#[doc = "Field `pin15` writer - "]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16 {
    #[doc = "0: `0`"]
    Uart0Rx = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin16> for bool {
    #[inline(always)]
    fn from(variant: Pin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin16` reader - "]
pub type Pin16R = crate::BitReader<Pin16>;
impl Pin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin16 {
        match self.bits {
            false => Pin16::Uart0Rx,
            true => Pin16::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == Pin16::Uart0Rx
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin16::Iof1
    }
}
#[doc = "Field `pin16` writer - "]
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG, Pin16>;
impl<'a, REG> Pin16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::Uart0Rx)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17 {
    #[doc = "0: `0`"]
    Uart0Tx = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin17> for bool {
    #[inline(always)]
    fn from(variant: Pin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin17` reader - "]
pub type Pin17R = crate::BitReader<Pin17>;
impl Pin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin17 {
        match self.bits {
            false => Pin17::Uart0Tx,
            true => Pin17::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == Pin17::Uart0Tx
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin17::Iof1
    }
}
#[doc = "Field `pin17` writer - "]
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG, Pin17>;
impl<'a, REG> Pin17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::Uart0Tx)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin18> for bool {
    #[inline(always)]
    fn from(variant: Pin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin18` reader - "]
pub type Pin18R = crate::BitReader<Pin18>;
impl Pin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin18 {
        match self.bits {
            false => Pin18::Iof0,
            true => Pin18::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin18::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin18::Iof1
    }
}
#[doc = "Field `pin18` writer - "]
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG, Pin18>;
impl<'a, REG> Pin18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm1_1 = 1,
}
impl From<Pin19> for bool {
    #[inline(always)]
    fn from(variant: Pin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin19` reader - "]
pub type Pin19R = crate::BitReader<Pin19>;
impl Pin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin19 {
        match self.bits {
            false => Pin19::Iof0,
            true => Pin19::Pwm1_1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin19::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm1_1(&self) -> bool {
        *self == Pin19::Pwm1_1
    }
}
#[doc = "Field `pin19` writer - "]
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG, Pin19>;
impl<'a, REG> Pin19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::Pwm1_1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm1_0 = 1,
}
impl From<Pin20> for bool {
    #[inline(always)]
    fn from(variant: Pin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin20` reader - "]
pub type Pin20R = crate::BitReader<Pin20>;
impl Pin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin20 {
        match self.bits {
            false => Pin20::Iof0,
            true => Pin20::Pwm1_0,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin20::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm1_0(&self) -> bool {
        *self == Pin20::Pwm1_0
    }
}
#[doc = "Field `pin20` writer - "]
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG, Pin20>;
impl<'a, REG> Pin20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::Pwm1_0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm1_2 = 1,
}
impl From<Pin21> for bool {
    #[inline(always)]
    fn from(variant: Pin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin21` reader - "]
pub type Pin21R = crate::BitReader<Pin21>;
impl Pin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin21 {
        match self.bits {
            false => Pin21::Iof0,
            true => Pin21::Pwm1_2,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin21::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm1_2(&self) -> bool {
        *self == Pin21::Pwm1_2
    }
}
#[doc = "Field `pin21` writer - "]
pub type Pin21W<'a, REG> = crate::BitWriter<'a, REG, Pin21>;
impl<'a, REG> Pin21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::Pwm1_2)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Pwm1_3 = 1,
}
impl From<Pin22> for bool {
    #[inline(always)]
    fn from(variant: Pin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin22` reader - "]
pub type Pin22R = crate::BitReader<Pin22>;
impl Pin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin22 {
        match self.bits {
            false => Pin22::Iof0,
            true => Pin22::Pwm1_3,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin22::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pwm1_3(&self) -> bool {
        *self == Pin22::Pwm1_3
    }
}
#[doc = "Field `pin22` writer - "]
pub type Pin22W<'a, REG> = crate::BitWriter<'a, REG, Pin22>;
impl<'a, REG> Pin22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::Pwm1_3)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23 {
    #[doc = "0: `0`"]
    Iof0 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin23> for bool {
    #[inline(always)]
    fn from(variant: Pin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin23` reader - "]
pub type Pin23R = crate::BitReader<Pin23>;
impl Pin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin23 {
        match self.bits {
            false => Pin23::Iof0,
            true => Pin23::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_iof0(&self) -> bool {
        *self == Pin23::Iof0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin23::Iof1
    }
}
#[doc = "Field `pin23` writer - "]
pub type Pin23W<'a, REG> = crate::BitWriter<'a, REG, Pin23>;
impl<'a, REG> Pin23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iof0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::Iof0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24 {
    #[doc = "0: `0`"]
    Uart1Rx = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin24> for bool {
    #[inline(always)]
    fn from(variant: Pin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin24` reader - "]
pub type Pin24R = crate::BitReader<Pin24>;
impl Pin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin24 {
        match self.bits {
            false => Pin24::Uart1Rx,
            true => Pin24::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == Pin24::Uart1Rx
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin24::Iof1
    }
}
#[doc = "Field `pin24` writer - "]
pub type Pin24W<'a, REG> = crate::BitWriter<'a, REG, Pin24>;
impl<'a, REG> Pin24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::Uart1Rx)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25 {
    #[doc = "0: `0`"]
    Uart1Tx = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin25> for bool {
    #[inline(always)]
    fn from(variant: Pin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin25` reader - "]
pub type Pin25R = crate::BitReader<Pin25>;
impl Pin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin25 {
        match self.bits {
            false => Pin25::Uart1Tx,
            true => Pin25::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == Pin25::Uart1Tx
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin25::Iof1
    }
}
#[doc = "Field `pin25` writer - "]
pub type Pin25W<'a, REG> = crate::BitWriter<'a, REG, Pin25>;
impl<'a, REG> Pin25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::Uart1Tx)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26 {
    #[doc = "0: `0`"]
    Qspi2Ss = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin26> for bool {
    #[inline(always)]
    fn from(variant: Pin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin26` reader - "]
pub type Pin26R = crate::BitReader<Pin26>;
impl Pin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin26 {
        match self.bits {
            false => Pin26::Qspi2Ss,
            true => Pin26::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_ss(&self) -> bool {
        *self == Pin26::Qspi2Ss
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin26::Iof1
    }
}
#[doc = "Field `pin26` writer - "]
pub type Pin26W<'a, REG> = crate::BitWriter<'a, REG, Pin26>;
impl<'a, REG> Pin26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::Qspi2Ss)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27 {
    #[doc = "0: `0`"]
    Qspi2Sd0 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin27> for bool {
    #[inline(always)]
    fn from(variant: Pin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin27` reader - "]
pub type Pin27R = crate::BitReader<Pin27>;
impl Pin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin27 {
        match self.bits {
            false => Pin27::Qspi2Sd0,
            true => Pin27::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_sd0(&self) -> bool {
        *self == Pin27::Qspi2Sd0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin27::Iof1
    }
}
#[doc = "Field `pin27` writer - "]
pub type Pin27W<'a, REG> = crate::BitWriter<'a, REG, Pin27>;
impl<'a, REG> Pin27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::Qspi2Sd0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28 {
    #[doc = "0: `0`"]
    Qspi2Sd1 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin28> for bool {
    #[inline(always)]
    fn from(variant: Pin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin28` reader - "]
pub type Pin28R = crate::BitReader<Pin28>;
impl Pin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin28 {
        match self.bits {
            false => Pin28::Qspi2Sd1,
            true => Pin28::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_sd1(&self) -> bool {
        *self == Pin28::Qspi2Sd1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin28::Iof1
    }
}
#[doc = "Field `pin28` writer - "]
pub type Pin28W<'a, REG> = crate::BitWriter<'a, REG, Pin28>;
impl<'a, REG> Pin28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::Qspi2Sd1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29 {
    #[doc = "0: `0`"]
    Qspi2Sck = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin29> for bool {
    #[inline(always)]
    fn from(variant: Pin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin29` reader - "]
pub type Pin29R = crate::BitReader<Pin29>;
impl Pin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin29 {
        match self.bits {
            false => Pin29::Qspi2Sck,
            true => Pin29::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_sck(&self) -> bool {
        *self == Pin29::Qspi2Sck
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin29::Iof1
    }
}
#[doc = "Field `pin29` writer - "]
pub type Pin29W<'a, REG> = crate::BitWriter<'a, REG, Pin29>;
impl<'a, REG> Pin29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::Qspi2Sck)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30 {
    #[doc = "0: `0`"]
    Qspi2Sd2 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin30> for bool {
    #[inline(always)]
    fn from(variant: Pin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin30` reader - "]
pub type Pin30R = crate::BitReader<Pin30>;
impl Pin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin30 {
        match self.bits {
            false => Pin30::Qspi2Sd2,
            true => Pin30::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_sd2(&self) -> bool {
        *self == Pin30::Qspi2Sd2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin30::Iof1
    }
}
#[doc = "Field `pin30` writer - "]
pub type Pin30W<'a, REG> = crate::BitWriter<'a, REG, Pin30>;
impl<'a, REG> Pin30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::Qspi2Sd2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::Iof1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31 {
    #[doc = "0: `0`"]
    Qspi2Sd3 = 0,
    #[doc = "1: `1`"]
    Iof1 = 1,
}
impl From<Pin31> for bool {
    #[inline(always)]
    fn from(variant: Pin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pin31` reader - "]
pub type Pin31R = crate::BitReader<Pin31>;
impl Pin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin31 {
        match self.bits {
            false => Pin31::Qspi2Sd3,
            true => Pin31::Iof1,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_qspi2_sd3(&self) -> bool {
        *self == Pin31::Qspi2Sd3
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iof1(&self) -> bool {
        *self == Pin31::Iof1
    }
}
#[doc = "Field `pin31` writer - "]
pub type Pin31W<'a, REG> = crate::BitWriter<'a, REG, Pin31>;
impl<'a, REG> Pin31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn qspi2_sd3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::Qspi2Sd3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iof1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::Iof1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pin21(&self) -> Pin21R {
        Pin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pin22(&self) -> Pin22R {
        Pin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pin23(&self) -> Pin23R {
        Pin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pin24(&self) -> Pin24R {
        Pin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pin25(&self) -> Pin25R {
        Pin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pin26(&self) -> Pin26R {
        Pin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin27(&self) -> Pin27R {
        Pin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pin28(&self) -> Pin28R {
        Pin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pin29(&self) -> Pin29R {
        Pin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pin30(&self) -> Pin30R {
        Pin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin31(&self) -> Pin31R {
        Pin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<IofSelSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<IofSelSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<IofSelSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<IofSelSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<IofSelSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<IofSelSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<IofSelSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<IofSelSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<IofSelSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<IofSelSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<IofSelSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<IofSelSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<IofSelSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<IofSelSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<IofSelSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<IofSelSpec> {
        Pin15W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pin16(&mut self) -> Pin16W<IofSelSpec> {
        Pin16W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pin17(&mut self) -> Pin17W<IofSelSpec> {
        Pin17W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pin18(&mut self) -> Pin18W<IofSelSpec> {
        Pin18W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pin19(&mut self) -> Pin19W<IofSelSpec> {
        Pin19W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pin20(&mut self) -> Pin20W<IofSelSpec> {
        Pin20W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pin21(&mut self) -> Pin21W<IofSelSpec> {
        Pin21W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pin22(&mut self) -> Pin22W<IofSelSpec> {
        Pin22W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pin23(&mut self) -> Pin23W<IofSelSpec> {
        Pin23W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pin24(&mut self) -> Pin24W<IofSelSpec> {
        Pin24W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pin25(&mut self) -> Pin25W<IofSelSpec> {
        Pin25W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pin26(&mut self) -> Pin26W<IofSelSpec> {
        Pin26W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pin27(&mut self) -> Pin27W<IofSelSpec> {
        Pin27W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pin28(&mut self) -> Pin28W<IofSelSpec> {
        Pin28W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pin29(&mut self) -> Pin29W<IofSelSpec> {
        Pin29W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pin30(&mut self) -> Pin30W<IofSelSpec> {
        Pin30W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pin31(&mut self) -> Pin31W<IofSelSpec> {
        Pin31W::new(self, 31)
    }
}
#[doc = "HW I/O Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iof_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iof_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IofSelSpec;
impl crate::RegisterSpec for IofSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iof_sel::R`](R) reader structure"]
impl crate::Readable for IofSelSpec {}
#[doc = "`write(|w| ..)` method takes [`iof_sel::W`](W) writer structure"]
impl crate::Writable for IofSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets iof_sel to value 0"]
impl crate::Resettable for IofSelSpec {
    const RESET_VALUE: u32 = 0;
}
