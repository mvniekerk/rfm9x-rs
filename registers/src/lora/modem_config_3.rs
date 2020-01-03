#[doc = "Reader of register MODEM_CONFIG_3"]
pub type R = crate::R<u8, super::MODEM_CONFIG_3>;
#[doc = "Writer for register MODEM_CONFIG_3"]
pub type W = crate::W<u8, super::MODEM_CONFIG_3>;
#[doc = "Register MODEM_CONFIG_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MODEM_CONFIG_3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LNA gain set by regiter or internal AGC loop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LNA_GAIN_SETTER_A {
    #[doc = "0: LNA gain set by register LnaGain"]
    REGISTER_LNA_GAIN = 0,
    #[doc = "1: LNA gain set by the internal AGC loop"]
    AGC_LOOP = 1,
}
impl From<LNA_GAIN_SETTER_A> for u8 {
    #[inline(always)]
    fn from(variant: LNA_GAIN_SETTER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LNA_GAIN_SETTER`"]
pub type LNA_GAIN_SETTER_R = crate::R<u8, LNA_GAIN_SETTER_A>;
impl LNA_GAIN_SETTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LNA_GAIN_SETTER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LNA_GAIN_SETTER_A::REGISTER_LNA_GAIN),
            1 => Val(LNA_GAIN_SETTER_A::AGC_LOOP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REGISTER_LNA_GAIN`"]
    #[inline(always)]
    pub fn is_register_lna_gain(&self) -> bool {
        *self == LNA_GAIN_SETTER_A::REGISTER_LNA_GAIN
    }
    #[doc = "Checks if the value of the field is `AGC_LOOP`"]
    #[inline(always)]
    pub fn is_agc_loop(&self) -> bool {
        *self == LNA_GAIN_SETTER_A::AGC_LOOP
    }
}
#[doc = "Write proxy for field `LNA_GAIN_SETTER`"]
pub struct LNA_GAIN_SETTER_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_GAIN_SETTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LNA_GAIN_SETTER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LNA gain set by register LnaGain"]
    #[inline(always)]
    pub fn register_lna_gain(self) -> &'a mut W {
        self.variant(LNA_GAIN_SETTER_A::REGISTER_LNA_GAIN)
    }
    #[doc = "LNA gain set by the internal AGC loop"]
    #[inline(always)]
    pub fn agc_loop(self) -> &'a mut W {
        self.variant(LNA_GAIN_SETTER_A::AGC_LOOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Mobile node static or mobile\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOBILE_NODE_A {
    #[doc = "0: Static node"]
    STATIC_NODE = 0,
    #[doc = "1: Mobile node"]
    MOBILE_NODE = 1,
}
impl From<MOBILE_NODE_A> for bool {
    #[inline(always)]
    fn from(variant: MOBILE_NODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOBILE_NODE`"]
pub type MOBILE_NODE_R = crate::R<bool, MOBILE_NODE_A>;
impl MOBILE_NODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOBILE_NODE_A {
        match self.bits {
            false => MOBILE_NODE_A::STATIC_NODE,
            true => MOBILE_NODE_A::MOBILE_NODE,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC_NODE`"]
    #[inline(always)]
    pub fn is_static_node(&self) -> bool {
        *self == MOBILE_NODE_A::STATIC_NODE
    }
    #[doc = "Checks if the value of the field is `MOBILE_NODE`"]
    #[inline(always)]
    pub fn is_mobile_node(&self) -> bool {
        *self == MOBILE_NODE_A::MOBILE_NODE
    }
}
#[doc = "Write proxy for field `MOBILE_NODE`"]
pub struct MOBILE_NODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOBILE_NODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOBILE_NODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Static node"]
    #[inline(always)]
    pub fn static_node(self) -> &'a mut W {
        self.variant(MOBILE_NODE_A::STATIC_NODE)
    }
    #[doc = "Mobile node"]
    #[inline(always)]
    pub fn mobile_node(self) -> &'a mut W {
        self.variant(MOBILE_NODE_A::MOBILE_NODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - LNA gain set by regiter or internal AGC loop"]
    #[inline(always)]
    pub fn lna_gain_setter(&self) -> LNA_GAIN_SETTER_R {
        LNA_GAIN_SETTER_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Mobile node static or mobile"]
    #[inline(always)]
    pub fn mobile_node(&self) -> MOBILE_NODE_R {
        MOBILE_NODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - LNA gain set by regiter or internal AGC loop"]
    #[inline(always)]
    pub fn lna_gain_setter(&mut self) -> LNA_GAIN_SETTER_W {
        LNA_GAIN_SETTER_W { w: self }
    }
    #[doc = "Bit 3 - Mobile node static or mobile"]
    #[inline(always)]
    pub fn mobile_node(&mut self) -> MOBILE_NODE_W {
        MOBILE_NODE_W { w: self }
    }
}
