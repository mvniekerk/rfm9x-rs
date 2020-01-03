#[doc = "Reader of register HOP_CHANNEL"]
pub type R = crate::R<u8, super::HOP_CHANNEL>;
#[doc = "Reader of field `FHSS_PRESENT_CHANNEL`"]
pub type FHSS_PRESENT_CHANNEL_R = crate::R<u8, u8>;
#[doc = "CRC information extracted from the received packet header\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PAYLOAD_CRC_ON_A {
    #[doc = "0: Header indicates CRC off"]
    CRC_OFF = 0,
    #[doc = "1: Header indicates CRC on"]
    CRC_ON = 1,
}
impl From<RX_PAYLOAD_CRC_ON_A> for bool {
    #[inline(always)]
    fn from(variant: RX_PAYLOAD_CRC_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_PAYLOAD_CRC_ON`"]
pub type RX_PAYLOAD_CRC_ON_R = crate::R<bool, RX_PAYLOAD_CRC_ON_A>;
impl RX_PAYLOAD_CRC_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PAYLOAD_CRC_ON_A {
        match self.bits {
            false => RX_PAYLOAD_CRC_ON_A::CRC_OFF,
            true => RX_PAYLOAD_CRC_ON_A::CRC_ON,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_OFF`"]
    #[inline(always)]
    pub fn is_crc_off(&self) -> bool {
        *self == RX_PAYLOAD_CRC_ON_A::CRC_OFF
    }
    #[doc = "Checks if the value of the field is `CRC_ON`"]
    #[inline(always)]
    pub fn is_crc_on(&self) -> bool {
        *self == RX_PAYLOAD_CRC_ON_A::CRC_ON
    }
}
#[doc = "Whether PLL failed to lock while attempting a TX/RX/CAD operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_TIMEOUT_A {
    #[doc = "0: PLL did lock"]
    PLL_DID_LOCK = 0,
    #[doc = "1: PLL did not lock"]
    PLL_DID_NOT_LOCK = 1,
}
impl From<PLL_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_TIMEOUT`"]
pub type PLL_TIMEOUT_R = crate::R<bool, PLL_TIMEOUT_A>;
impl PLL_TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_TIMEOUT_A {
        match self.bits {
            false => PLL_TIMEOUT_A::PLL_DID_LOCK,
            true => PLL_TIMEOUT_A::PLL_DID_NOT_LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_DID_LOCK`"]
    #[inline(always)]
    pub fn is_pll_did_lock(&self) -> bool {
        *self == PLL_TIMEOUT_A::PLL_DID_LOCK
    }
    #[doc = "Checks if the value of the field is `PLL_DID_NOT_LOCK`"]
    #[inline(always)]
    pub fn is_pll_did_not_lock(&self) -> bool {
        *self == PLL_TIMEOUT_A::PLL_DID_NOT_LOCK
    }
}
impl R {
    #[doc = "Bits 0:5 - Current value of frequency hopping channel in use"]
    #[inline(always)]
    pub fn fhss_present_channel(&self) -> FHSS_PRESENT_CHANNEL_R {
        FHSS_PRESENT_CHANNEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CRC information extracted from the received packet header"]
    #[inline(always)]
    pub fn rx_payload_crc_on(&self) -> RX_PAYLOAD_CRC_ON_R {
        RX_PAYLOAD_CRC_ON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Whether PLL failed to lock while attempting a TX/RX/CAD operation"]
    #[inline(always)]
    pub fn pll_timeout(&self) -> PLL_TIMEOUT_R {
        PLL_TIMEOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
