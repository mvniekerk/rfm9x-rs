#[doc = "Reader of register IRQ_FLAGS_MASK"]
pub type R = crate::R<u8, super::IRQ_FLAGS_MASK>;
#[doc = "Writer for register IRQ_FLAGS_MASK"]
pub type W = crate::W<u8, super::IRQ_FLAGS_MASK>;
#[doc = "Register IRQ_FLAGS_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_FLAGS_MASK {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAD_DETECTED_MASK`"]
pub type CAD_DETECTED_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAD_DETECTED_MASK`"]
pub struct CAD_DETECTED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CAD_DETECTED_MASK_W<'a> {
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
#[doc = "Reader of field `FHSS_CHANGE_CHANNEL_MASK`"]
pub type FHSS_CHANGE_CHANNEL_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FHSS_CHANGE_CHANNEL_MASK`"]
pub struct FHSS_CHANGE_CHANNEL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FHSS_CHANGE_CHANNEL_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CAD_DONE_MASK`"]
pub type CAD_DONE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAD_DONE_MASK`"]
pub struct CAD_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CAD_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TX_DONE_MASK`"]
pub type TX_DONE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE_MASK`"]
pub struct TX_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_MASK_W<'a> {
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
#[doc = "Reader of field `VALID_HEADER_MASK`"]
pub type VALID_HEADER_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID_HEADER_MASK`"]
pub struct VALID_HEADER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_HEADER_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PAYLOAD_CRC_ERROR_MASK`"]
pub type PAYLOAD_CRC_ERROR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAYLOAD_CRC_ERROR_MASK`"]
pub struct PAYLOAD_CRC_ERROR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_CRC_ERROR_MASK_W<'a> {
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
#[doc = "Reader of field `RX_DONE_MASK`"]
pub type RX_DONE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DONE_MASK`"]
pub struct RX_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RX_TIMEOUT_MASK`"]
pub type RX_TIMEOUT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TIMEOUT_MASK`"]
pub struct RX_TIMEOUT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_MASK_W<'a> {
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
    #[doc = "Bit 0 - Card detected interrupt mask. Setting this masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn cad_detected_mask(&self) -> CAD_DETECTED_MASK_R {
        CAD_DETECTED_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FHSS change channel interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn fhss_change_channel_mask(&self) -> FHSS_CHANGE_CHANNEL_MASK_R {
        FHSS_CHANGE_CHANNEL_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAD complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn cad_done_mask(&self) -> CAD_DONE_MASK_R {
        CAD_DONE_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Payload transmission complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn tx_done_mask(&self) -> TX_DONE_MASK_R {
        TX_DONE_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Valid header received in Rx mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn valid_header_mask(&self) -> VALID_HEADER_MASK_R {
        VALID_HEADER_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Payload CRC error interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn payload_crc_error_mask(&self) -> PAYLOAD_CRC_ERROR_MASK_R {
        PAYLOAD_CRC_ERROR_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Packet reception complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn rx_done_mask(&self) -> RX_DONE_MASK_R {
        RX_DONE_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timeout interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn rx_timeout_mask(&self) -> RX_TIMEOUT_MASK_R {
        RX_TIMEOUT_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detected interrupt mask. Setting this masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn cad_detected_mask(&mut self) -> CAD_DETECTED_MASK_W {
        CAD_DETECTED_MASK_W { w: self }
    }
    #[doc = "Bit 1 - FHSS change channel interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn fhss_change_channel_mask(&mut self) -> FHSS_CHANGE_CHANNEL_MASK_W {
        FHSS_CHANGE_CHANNEL_MASK_W { w: self }
    }
    #[doc = "Bit 2 - CAD complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn cad_done_mask(&mut self) -> CAD_DONE_MASK_W {
        CAD_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 3 - FIFO Payload transmission complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn tx_done_mask(&mut self) -> TX_DONE_MASK_W {
        TX_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Valid header received in Rx mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn valid_header_mask(&mut self) -> VALID_HEADER_MASK_W {
        VALID_HEADER_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Payload CRC error interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn payload_crc_error_mask(&mut self) -> PAYLOAD_CRC_ERROR_MASK_W {
        PAYLOAD_CRC_ERROR_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Packet reception complete interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn rx_done_mask(&mut self) -> RX_DONE_MASK_W {
        RX_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Timeout interrupt mask. Setting this bit masks the corresponding IRQ in RegIrqFlags"]
    #[inline(always)]
    pub fn rx_timeout_mask(&mut self) -> RX_TIMEOUT_MASK_W {
        RX_TIMEOUT_MASK_W { w: self }
    }
}
