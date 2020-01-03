#[doc = "Reader of register MODEM_CONFIG_1"]
pub type R = crate::R<u8, super::MODEM_CONFIG_1>;
#[doc = "Writer for register MODEM_CONFIG_1"]
pub type W = crate::W<u8, super::MODEM_CONFIG_1>;
#[doc = "Register MODEM_CONFIG_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MODEM_CONFIG_1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Implicit/explicit header mode on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPLICIT_HEADER_MODE_ON_A {
    #[doc = "0: Implicit header mode"]
    IMPLICIT_MODE = 0,
    #[doc = "1: Explicit header mode"]
    EXPLICIT_MODE = 1,
}
impl From<IMPLICIT_HEADER_MODE_ON_A> for bool {
    #[inline(always)]
    fn from(variant: IMPLICIT_HEADER_MODE_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMPLICIT_HEADER_MODE_ON`"]
pub type IMPLICIT_HEADER_MODE_ON_R = crate::R<bool, IMPLICIT_HEADER_MODE_ON_A>;
impl IMPLICIT_HEADER_MODE_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMPLICIT_HEADER_MODE_ON_A {
        match self.bits {
            false => IMPLICIT_HEADER_MODE_ON_A::IMPLICIT_MODE,
            true => IMPLICIT_HEADER_MODE_ON_A::EXPLICIT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `IMPLICIT_MODE`"]
    #[inline(always)]
    pub fn is_implicit_mode(&self) -> bool {
        *self == IMPLICIT_HEADER_MODE_ON_A::IMPLICIT_MODE
    }
    #[doc = "Checks if the value of the field is `EXPLICIT_MODE`"]
    #[inline(always)]
    pub fn is_explicit_mode(&self) -> bool {
        *self == IMPLICIT_HEADER_MODE_ON_A::EXPLICIT_MODE
    }
}
#[doc = "Write proxy for field `IMPLICIT_HEADER_MODE_ON`"]
pub struct IMPLICIT_HEADER_MODE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPLICIT_HEADER_MODE_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMPLICIT_HEADER_MODE_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Implicit header mode"]
    #[inline(always)]
    pub fn implicit_mode(self) -> &'a mut W {
        self.variant(IMPLICIT_HEADER_MODE_ON_A::IMPLICIT_MODE)
    }
    #[doc = "Explicit header mode"]
    #[inline(always)]
    pub fn explicit_mode(self) -> &'a mut W {
        self.variant(IMPLICIT_HEADER_MODE_ON_A::EXPLICIT_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Error coding rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CODING_RATE_A {
    #[doc = "1: 4/5"]
    _4_5 = 1,
    #[doc = "2: 4/6"]
    _4_6 = 2,
    #[doc = "3: 4/7"]
    _4_7 = 3,
    #[doc = "4: 4/8"]
    _4_8 = 4,
}
impl From<CODING_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: CODING_RATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CODING_RATE`"]
pub type CODING_RATE_R = crate::R<u8, CODING_RATE_A>;
impl CODING_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CODING_RATE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CODING_RATE_A::_4_5),
            2 => Val(CODING_RATE_A::_4_6),
            3 => Val(CODING_RATE_A::_4_7),
            4 => Val(CODING_RATE_A::_4_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_5`"]
    #[inline(always)]
    pub fn is_4_5(&self) -> bool {
        *self == CODING_RATE_A::_4_5
    }
    #[doc = "Checks if the value of the field is `_4_6`"]
    #[inline(always)]
    pub fn is_4_6(&self) -> bool {
        *self == CODING_RATE_A::_4_6
    }
    #[doc = "Checks if the value of the field is `_4_7`"]
    #[inline(always)]
    pub fn is_4_7(&self) -> bool {
        *self == CODING_RATE_A::_4_7
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline(always)]
    pub fn is_4_8(&self) -> bool {
        *self == CODING_RATE_A::_4_8
    }
}
#[doc = "Write proxy for field `CODING_RATE`"]
pub struct CODING_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_RATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4/5"]
    #[inline(always)]
    pub fn _4_5(self) -> &'a mut W {
        self.variant(CODING_RATE_A::_4_5)
    }
    #[doc = "4/6"]
    #[inline(always)]
    pub fn _4_6(self) -> &'a mut W {
        self.variant(CODING_RATE_A::_4_6)
    }
    #[doc = "4/7"]
    #[inline(always)]
    pub fn _4_7(self) -> &'a mut W {
        self.variant(CODING_RATE_A::_4_7)
    }
    #[doc = "4/8"]
    #[inline(always)]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(CODING_RATE_A::_4_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
#[doc = "Signal bandwith\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BW_A {
    #[doc = "0: 7.8 kHz"]
    _7_8KHZ = 0,
    #[doc = "1: 10.4 kHz"]
    _10_4KHZ = 1,
    #[doc = "2: 15.6 kHz"]
    _15_6KHZ = 2,
    #[doc = "3: 20.8 kHz"]
    _20_8KHZ = 3,
    #[doc = "4: 31.25 kHz"]
    _31_25KHZ = 4,
    #[doc = "5: 41.7 kHz"]
    _41_7KHZ = 5,
    #[doc = "6: 62.5 kHz"]
    _62_5KHZ = 6,
    #[doc = "7: 125 kHz"]
    _125KHZ = 7,
    #[doc = "8: 250 kHz"]
    _250KHZ = 8,
    #[doc = "9: 500 kHz"]
    _500KHZ = 9,
}
impl From<BW_A> for u8 {
    #[inline(always)]
    fn from(variant: BW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BW`"]
pub type BW_R = crate::R<u8, BW_A>;
impl BW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BW_A::_7_8KHZ),
            1 => Val(BW_A::_10_4KHZ),
            2 => Val(BW_A::_15_6KHZ),
            3 => Val(BW_A::_20_8KHZ),
            4 => Val(BW_A::_31_25KHZ),
            5 => Val(BW_A::_41_7KHZ),
            6 => Val(BW_A::_62_5KHZ),
            7 => Val(BW_A::_125KHZ),
            8 => Val(BW_A::_250KHZ),
            9 => Val(BW_A::_500KHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7_8KHZ`"]
    #[inline(always)]
    pub fn is_7_8khz(&self) -> bool {
        *self == BW_A::_7_8KHZ
    }
    #[doc = "Checks if the value of the field is `_10_4KHZ`"]
    #[inline(always)]
    pub fn is_10_4khz(&self) -> bool {
        *self == BW_A::_10_4KHZ
    }
    #[doc = "Checks if the value of the field is `_15_6KHZ`"]
    #[inline(always)]
    pub fn is_15_6khz(&self) -> bool {
        *self == BW_A::_15_6KHZ
    }
    #[doc = "Checks if the value of the field is `_20_8KHZ`"]
    #[inline(always)]
    pub fn is_20_8khz(&self) -> bool {
        *self == BW_A::_20_8KHZ
    }
    #[doc = "Checks if the value of the field is `_31_25KHZ`"]
    #[inline(always)]
    pub fn is_31_25khz(&self) -> bool {
        *self == BW_A::_31_25KHZ
    }
    #[doc = "Checks if the value of the field is `_41_7KHZ`"]
    #[inline(always)]
    pub fn is_41_7khz(&self) -> bool {
        *self == BW_A::_41_7KHZ
    }
    #[doc = "Checks if the value of the field is `_62_5KHZ`"]
    #[inline(always)]
    pub fn is_62_5khz(&self) -> bool {
        *self == BW_A::_62_5KHZ
    }
    #[doc = "Checks if the value of the field is `_125KHZ`"]
    #[inline(always)]
    pub fn is_125khz(&self) -> bool {
        *self == BW_A::_125KHZ
    }
    #[doc = "Checks if the value of the field is `_250KHZ`"]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        *self == BW_A::_250KHZ
    }
    #[doc = "Checks if the value of the field is `_500KHZ`"]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        *self == BW_A::_500KHZ
    }
}
#[doc = "Write proxy for field `BW`"]
pub struct BW_W<'a> {
    w: &'a mut W,
}
impl<'a> BW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "7.8 kHz"]
    #[inline(always)]
    pub fn _7_8khz(self) -> &'a mut W {
        self.variant(BW_A::_7_8KHZ)
    }
    #[doc = "10.4 kHz"]
    #[inline(always)]
    pub fn _10_4khz(self) -> &'a mut W {
        self.variant(BW_A::_10_4KHZ)
    }
    #[doc = "15.6 kHz"]
    #[inline(always)]
    pub fn _15_6khz(self) -> &'a mut W {
        self.variant(BW_A::_15_6KHZ)
    }
    #[doc = "20.8 kHz"]
    #[inline(always)]
    pub fn _20_8khz(self) -> &'a mut W {
        self.variant(BW_A::_20_8KHZ)
    }
    #[doc = "31.25 kHz"]
    #[inline(always)]
    pub fn _31_25khz(self) -> &'a mut W {
        self.variant(BW_A::_31_25KHZ)
    }
    #[doc = "41.7 kHz"]
    #[inline(always)]
    pub fn _41_7khz(self) -> &'a mut W {
        self.variant(BW_A::_41_7KHZ)
    }
    #[doc = "62.5 kHz"]
    #[inline(always)]
    pub fn _62_5khz(self) -> &'a mut W {
        self.variant(BW_A::_62_5KHZ)
    }
    #[doc = "125 kHz"]
    #[inline(always)]
    pub fn _125khz(self) -> &'a mut W {
        self.variant(BW_A::_125KHZ)
    }
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut W {
        self.variant(BW_A::_250KHZ)
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut W {
        self.variant(BW_A::_500KHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Implicit/explicit header mode on"]
    #[inline(always)]
    pub fn implicit_header_mode_on(&self) -> IMPLICIT_HEADER_MODE_ON_R {
        IMPLICIT_HEADER_MODE_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Error coding rate"]
    #[inline(always)]
    pub fn coding_rate(&self) -> CODING_RATE_R {
        CODING_RATE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Signal bandwith"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Implicit/explicit header mode on"]
    #[inline(always)]
    pub fn implicit_header_mode_on(&mut self) -> IMPLICIT_HEADER_MODE_ON_W {
        IMPLICIT_HEADER_MODE_ON_W { w: self }
    }
    #[doc = "Bits 1:3 - Error coding rate"]
    #[inline(always)]
    pub fn coding_rate(&mut self) -> CODING_RATE_W {
        CODING_RATE_W { w: self }
    }
    #[doc = "Bits 4:7 - Signal bandwith"]
    #[inline(always)]
    pub fn bw(&mut self) -> BW_W {
        BW_W { w: self }
    }
}
