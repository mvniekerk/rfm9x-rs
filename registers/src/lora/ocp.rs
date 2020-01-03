#[doc = "Reader of register OCP"]
pub type R = crate::R<u8, super::OCP>;
#[doc = "Writer for register OCP"]
pub type W = crate::W<u8, super::OCP>;
#[doc = "Register OCP `reset()`'s with value 0"]
impl crate::ResetValue for super::OCP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCP_TRIM`"]
pub type OCP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCP_TRIM`"]
pub struct OCP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `OCP_ON`"]
pub type OCP_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCP_ON`"]
pub struct OCP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> OCP_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trimming of OCP current"]
    #[inline(always)]
    pub fn ocp_trim(&self) -> OCP_TRIM_R {
        OCP_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enables overload current protection"]
    #[inline(always)]
    pub fn ocp_on(&self) -> OCP_ON_R {
        OCP_ON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trimming of OCP current"]
    #[inline(always)]
    pub fn ocp_trim(&mut self) -> OCP_TRIM_W {
        OCP_TRIM_W { w: self }
    }
    #[doc = "Bit 5 - Enables overload current protection"]
    #[inline(always)]
    pub fn ocp_on(&mut self) -> OCP_ON_W {
        OCP_ON_W { w: self }
    }
}
