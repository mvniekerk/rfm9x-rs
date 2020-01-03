#[doc = "Reader of register PA"]
pub type R = crate::R<u8, super::PA>;
#[doc = "Writer for register PA"]
pub type W = crate::W<u8, super::PA>;
#[doc = "Register PA `reset()`'s with value 0"]
impl crate::ResetValue for super::PA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTPUT_POWER`"]
pub type OUTPUT_POWER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTPUT_POWER`"]
pub struct OUTPUT_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_POWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MAX_POWER`"]
pub type MAX_POWER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_POWER`"]
pub struct MAX_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_POWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
#[doc = "Select PA output pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_SELECT_A {
    #[doc = "0: RFO pin. Output limited to 14dBm"]
    RFO = 0,
    #[doc = "1: PA boost pin. Output power limited to 20dBm"]
    PA_BOOST = 1,
}
impl From<PA_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PA_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_SELECT`"]
pub type PA_SELECT_R = crate::R<bool, PA_SELECT_A>;
impl PA_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PA_SELECT_A {
        match self.bits {
            false => PA_SELECT_A::RFO,
            true => PA_SELECT_A::PA_BOOST,
        }
    }
    #[doc = "Checks if the value of the field is `RFO`"]
    #[inline(always)]
    pub fn is_rfo(&self) -> bool {
        *self == PA_SELECT_A::RFO
    }
    #[doc = "Checks if the value of the field is `PA_BOOST`"]
    #[inline(always)]
    pub fn is_pa_boost(&self) -> bool {
        *self == PA_SELECT_A::PA_BOOST
    }
}
#[doc = "Write proxy for field `PA_SELECT`"]
pub struct PA_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RFO pin. Output limited to 14dBm"]
    #[inline(always)]
    pub fn rfo(self) -> &'a mut W {
        self.variant(PA_SELECT_A::RFO)
    }
    #[doc = "PA boost pin. Output power limited to 20dBm"]
    #[inline(always)]
    pub fn pa_boost(self) -> &'a mut W {
        self.variant(PA_SELECT_A::PA_BOOST)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Output power"]
    #[inline(always)]
    pub fn output_power(&self) -> OUTPUT_POWER_R {
        OUTPUT_POWER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Pmax=10.8 + 0.6 * MaxPower (dBm)"]
    #[inline(always)]
    pub fn max_power(&self) -> MAX_POWER_R {
        MAX_POWER_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Select PA output pin"]
    #[inline(always)]
    pub fn pa_select(&self) -> PA_SELECT_R {
        PA_SELECT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output power"]
    #[inline(always)]
    pub fn output_power(&mut self) -> OUTPUT_POWER_W {
        OUTPUT_POWER_W { w: self }
    }
    #[doc = "Bits 4:6 - Pmax=10.8 + 0.6 * MaxPower (dBm)"]
    #[inline(always)]
    pub fn max_power(&mut self) -> MAX_POWER_W {
        MAX_POWER_W { w: self }
    }
    #[doc = "Bit 7 - Select PA output pin"]
    #[inline(always)]
    pub fn pa_select(&mut self) -> PA_SELECT_W {
        PA_SELECT_W { w: self }
    }
}
