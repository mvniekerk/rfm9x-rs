#[doc = "Reader of register IRQ_FLAGS"]
pub type R = crate::R<u8, super::IRQ_FLAGS>;
#[doc = "Writer for register IRQ_FLAGS"]
pub type W = crate::W<u8, super::IRQ_FLAGS>;
#[doc = "Register IRQ_FLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_FLAGS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAD_DETECTED`"]
pub type CAD_DETECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAD_DETECTED`"]
pub struct CAD_DETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CAD_DETECTED_W<'a> {
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
#[doc = "Reader of field `FHSS_CHANGE_CHANNEL`"]
pub type FHSS_CHANGE_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FHSS_CHANGE_CHANNEL`"]
pub struct FHSS_CHANGE_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FHSS_CHANGE_CHANNEL_W<'a> {
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
#[doc = "Reader of field `CAD_DONE`"]
pub type CAD_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAD_DONE`"]
pub struct CAD_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAD_DONE_W<'a> {
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
#[doc = "Reader of field `TX_DONE`"]
pub type TX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE`"]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
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
#[doc = "Reader of field `VALID_HEADER`"]
pub type VALID_HEADER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID_HEADER`"]
pub struct VALID_HEADER_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_HEADER_W<'a> {
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
#[doc = "Reader of field `PAYLOAD_CRC_ERROR`"]
pub type PAYLOAD_CRC_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAYLOAD_CRC_ERROR`"]
pub struct PAYLOAD_CRC_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_CRC_ERROR_W<'a> {
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
#[doc = "Reader of field `RX_DONE`"]
pub type RX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DONE`"]
pub struct RX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DONE_W<'a> {
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
#[doc = "Reader of field `RX_TIMEOUT`"]
pub type RX_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TIMEOUT`"]
pub struct RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_W<'a> {
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
    #[doc = "Bit 0 - Valid Lora signal detected during CAD operation - a write operation clears IRQ"]
    #[inline(always)]
    pub fn cad_detected(&self) -> CAD_DETECTED_R {
        CAD_DETECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FHSS change channel interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn fhss_change_channel(&self) -> FHSS_CHANGE_CHANNEL_R {
        FHSS_CHANGE_CHANNEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAD complete - a write operation clears IRQ"]
    #[inline(always)]
    pub fn cad_done(&self) -> CAD_DONE_R {
        CAD_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO Payload transmission complete interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Valid header received in Rx - a write operation clears IRQ"]
    #[inline(always)]
    pub fn valid_header(&self) -> VALID_HEADER_R {
        VALID_HEADER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Payload CRC error interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn payload_crc_error(&self) -> PAYLOAD_CRC_ERROR_R {
        PAYLOAD_CRC_ERROR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Packet reception complete interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timeout interrupt - a write operation IRQ"]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Valid Lora signal detected during CAD operation - a write operation clears IRQ"]
    #[inline(always)]
    pub fn cad_detected(&mut self) -> CAD_DETECTED_W {
        CAD_DETECTED_W { w: self }
    }
    #[doc = "Bit 1 - FHSS change channel interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn fhss_change_channel(&mut self) -> FHSS_CHANGE_CHANNEL_W {
        FHSS_CHANGE_CHANNEL_W { w: self }
    }
    #[doc = "Bit 2 - CAD complete - a write operation clears IRQ"]
    #[inline(always)]
    pub fn cad_done(&mut self) -> CAD_DONE_W {
        CAD_DONE_W { w: self }
    }
    #[doc = "Bit 3 - FIFO Payload transmission complete interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 4 - Valid header received in Rx - a write operation clears IRQ"]
    #[inline(always)]
    pub fn valid_header(&mut self) -> VALID_HEADER_W {
        VALID_HEADER_W { w: self }
    }
    #[doc = "Bit 5 - Payload CRC error interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn payload_crc_error(&mut self) -> PAYLOAD_CRC_ERROR_W {
        PAYLOAD_CRC_ERROR_W { w: self }
    }
    #[doc = "Bit 6 - Packet reception complete interrupt - a write operation clears IRQ"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W { w: self }
    }
    #[doc = "Bit 7 - Timeout interrupt - a write operation IRQ"]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W {
        RX_TIMEOUT_W { w: self }
    }
}
