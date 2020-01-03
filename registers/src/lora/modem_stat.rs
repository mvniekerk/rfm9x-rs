#[doc = "Reader of register MODEM_STAT"]
pub type R = crate::R<u8, super::MODEM_STAT>;
#[doc = "Reader of field `SIGNAL_DETECTED`"]
pub type SIGNAL_DETECTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIGNAL_SYNCHRONIZED`"]
pub type SIGNAL_SYNCHRONIZED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_ON_GOING`"]
pub type RX_ON_GOING_R = crate::R<bool, bool>;
#[doc = "Reader of field `HEADER_INFO_VALID`"]
pub type HEADER_INFO_VALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODEM_CLEAR`"]
pub type MODEM_CLEAR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LAST_HEADER_CODING_RATE`"]
pub type LAST_HEADER_CODING_RATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Signal detected"]
    #[inline(always)]
    pub fn signal_detected(&self) -> SIGNAL_DETECTED_R {
        SIGNAL_DETECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Signal synchronized"]
    #[inline(always)]
    pub fn signal_synchronized(&self) -> SIGNAL_SYNCHRONIZED_R {
        SIGNAL_SYNCHRONIZED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX on going"]
    #[inline(always)]
    pub fn rx_on_going(&self) -> RX_ON_GOING_R {
        RX_ON_GOING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Header info valid"]
    #[inline(always)]
    pub fn header_info_valid(&self) -> HEADER_INFO_VALID_R {
        HEADER_INFO_VALID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Model clear"]
    #[inline(always)]
    pub fn modem_clear(&self) -> MODEM_CLEAR_R {
        MODEM_CLEAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Coding rate of last header"]
    #[inline(always)]
    pub fn last_header_coding_rate(&self) -> LAST_HEADER_CODING_RATE_R {
        LAST_HEADER_CODING_RATE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
