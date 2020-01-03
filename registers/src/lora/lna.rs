#[doc = "Reader of register LNA"]
pub type R = crate::R<u8, super::LNA>;
#[doc = "Writer for register LNA"]
pub type W = crate::W<u8, super::LNA>;
#[doc = "Register LNA `reset()`'s with value 0"]
impl crate::ResetValue for super::LNA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High frequency (RFI_HF) LNA current adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOST_HF_A {
    #[doc = "0: Default LNA current"]
    DEFAULT = 0,
    #[doc = "1: Boost on"]
    BOOST = 1,
}
impl From<BOOST_HF_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOST_HF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOOST_HF`"]
pub type BOOST_HF_R = crate::R<u8, BOOST_HF_A>;
impl BOOST_HF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOOST_HF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOOST_HF_A::DEFAULT),
            1 => Val(BOOST_HF_A::BOOST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == BOOST_HF_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `BOOST`"]
    #[inline(always)]
    pub fn is_boost(&self) -> bool {
        *self == BOOST_HF_A::BOOST
    }
}
#[doc = "Write proxy for field `BOOST_HF`"]
pub struct BOOST_HF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_HF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOST_HF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default LNA current"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(BOOST_HF_A::DEFAULT)
    }
    #[doc = "Boost on"]
    #[inline(always)]
    pub fn boost(self) -> &'a mut W {
        self.variant(BOOST_HF_A::BOOST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Low frequency (RFI_LF) LNA current adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOST_LF_A {
    #[doc = "0: Default LNA current"]
    DEFAULT = 0,
}
impl From<BOOST_LF_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOST_LF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOOST_LF`"]
pub type BOOST_LF_R = crate::R<u8, BOOST_LF_A>;
impl BOOST_LF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOOST_LF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOOST_LF_A::DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == BOOST_LF_A::DEFAULT
    }
}
#[doc = "Write proxy for field `BOOST_LF`"]
pub struct BOOST_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_LF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOST_LF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default LNA current"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(BOOST_LF_A::DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "LNA Gain setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LNA_GAIN_A {
    #[doc = "1: G1 / Maximum gain"]
    G1 = 1,
    #[doc = "2: G2"]
    G2 = 2,
    #[doc = "3: G3"]
    G3 = 3,
    #[doc = "4: G4"]
    G4 = 4,
    #[doc = "5: G5"]
    G5 = 5,
    #[doc = "6: G6"]
    G6 = 6,
}
impl From<LNA_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: LNA_GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LNA_GAIN`"]
pub type LNA_GAIN_R = crate::R<u8, LNA_GAIN_A>;
impl LNA_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LNA_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LNA_GAIN_A::G1),
            2 => Val(LNA_GAIN_A::G2),
            3 => Val(LNA_GAIN_A::G3),
            4 => Val(LNA_GAIN_A::G4),
            5 => Val(LNA_GAIN_A::G5),
            6 => Val(LNA_GAIN_A::G6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `G1`"]
    #[inline(always)]
    pub fn is_g1(&self) -> bool {
        *self == LNA_GAIN_A::G1
    }
    #[doc = "Checks if the value of the field is `G2`"]
    #[inline(always)]
    pub fn is_g2(&self) -> bool {
        *self == LNA_GAIN_A::G2
    }
    #[doc = "Checks if the value of the field is `G3`"]
    #[inline(always)]
    pub fn is_g3(&self) -> bool {
        *self == LNA_GAIN_A::G3
    }
    #[doc = "Checks if the value of the field is `G4`"]
    #[inline(always)]
    pub fn is_g4(&self) -> bool {
        *self == LNA_GAIN_A::G4
    }
    #[doc = "Checks if the value of the field is `G5`"]
    #[inline(always)]
    pub fn is_g5(&self) -> bool {
        *self == LNA_GAIN_A::G5
    }
    #[doc = "Checks if the value of the field is `G6`"]
    #[inline(always)]
    pub fn is_g6(&self) -> bool {
        *self == LNA_GAIN_A::G6
    }
}
#[doc = "Write proxy for field `LNA_GAIN`"]
pub struct LNA_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LNA_GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "G1 / Maximum gain"]
    #[inline(always)]
    pub fn g1(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G1)
    }
    #[doc = "G2"]
    #[inline(always)]
    pub fn g2(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G2)
    }
    #[doc = "G3"]
    #[inline(always)]
    pub fn g3(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G3)
    }
    #[doc = "G4"]
    #[inline(always)]
    pub fn g4(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G4)
    }
    #[doc = "G5"]
    #[inline(always)]
    pub fn g5(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G5)
    }
    #[doc = "G6"]
    #[inline(always)]
    pub fn g6(self) -> &'a mut W {
        self.variant(LNA_GAIN_A::G6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u8) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency (RFI_HF) LNA current adjustment"]
    #[inline(always)]
    pub fn boost_hf(&self) -> BOOST_HF_R {
        BOOST_HF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Low frequency (RFI_LF) LNA current adjustment"]
    #[inline(always)]
    pub fn boost_lf(&self) -> BOOST_LF_R {
        BOOST_LF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - LNA Gain setting"]
    #[inline(always)]
    pub fn lna_gain(&self) -> LNA_GAIN_R {
        LNA_GAIN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency (RFI_HF) LNA current adjustment"]
    #[inline(always)]
    pub fn boost_hf(&mut self) -> BOOST_HF_W {
        BOOST_HF_W { w: self }
    }
    #[doc = "Bits 4:5 - Low frequency (RFI_LF) LNA current adjustment"]
    #[inline(always)]
    pub fn boost_lf(&mut self) -> BOOST_LF_W {
        BOOST_LF_W { w: self }
    }
    #[doc = "Bits 5:6 - LNA Gain setting"]
    #[inline(always)]
    pub fn lna_gain(&mut self) -> LNA_GAIN_W {
        LNA_GAIN_W { w: self }
    }
}
