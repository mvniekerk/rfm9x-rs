#[doc = "Reader of register MODEM_CONFIG_2"]
pub type R = crate::R<u8, super::MODEM_CONFIG_2>;
#[doc = "Writer for register MODEM_CONFIG_2"]
pub type W = crate::W<u8, super::MODEM_CONFIG_2>;
#[doc = "Register MODEM_CONFIG_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MODEM_CONFIG_2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYMB_TIMEOUT_MSB`"]
pub type SYMB_TIMEOUT_MSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYMB_TIMEOUT_MSB`"]
pub struct SYMB_TIMEOUT_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYMB_TIMEOUT_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
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
#[doc = "Write proxy for field `RX_PAYLOAD_CRC_ON`"]
pub struct RX_PAYLOAD_CRC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PAYLOAD_CRC_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PAYLOAD_CRC_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Header indicates CRC off"]
    #[inline(always)]
    pub fn crc_off(self) -> &'a mut W {
        self.variant(RX_PAYLOAD_CRC_ON_A::CRC_OFF)
    }
    #[doc = "Header indicates CRC on"]
    #[inline(always)]
    pub fn crc_on(self) -> &'a mut W {
        self.variant(RX_PAYLOAD_CRC_ON_A::CRC_ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Whether Tx is in continuous mode or single packet mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CONTINUOUS_MODE_A {
    #[doc = "0: Single packet"]
    NORMAL = 0,
    #[doc = "1: Send multiple packets across the FIFO (used for spectral analysis)\n                                    "]
    CONTINUOUS_MODE = 1,
}
impl From<TX_CONTINUOUS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CONTINUOUS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_CONTINUOUS_MODE`"]
pub type TX_CONTINUOUS_MODE_R = crate::R<bool, TX_CONTINUOUS_MODE_A>;
impl TX_CONTINUOUS_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CONTINUOUS_MODE_A {
        match self.bits {
            false => TX_CONTINUOUS_MODE_A::NORMAL,
            true => TX_CONTINUOUS_MODE_A::CONTINUOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TX_CONTINUOUS_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_continuous_mode(&self) -> bool {
        *self == TX_CONTINUOUS_MODE_A::CONTINUOUS_MODE
    }
}
#[doc = "Write proxy for field `TX_CONTINUOUS_MODE`"]
pub struct TX_CONTINUOUS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CONTINUOUS_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CONTINUOUS_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single packet"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TX_CONTINUOUS_MODE_A::NORMAL)
    }
    #[doc = "Send multiple packets across the FIFO (used for spectral analysis)"]
    #[inline(always)]
    pub fn continuous_mode(self) -> &'a mut W {
        self.variant(TX_CONTINUOUS_MODE_A::CONTINUOUS_MODE)
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
#[doc = "Spreading factor (SF) rate (expressed as a base-2 logarithm)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPREADING_FACTOR_A {
    #[doc = "6: 64 chips / symbol"]
    _64 = 6,
    #[doc = "7: 128 chips / symbol"]
    _128 = 7,
    #[doc = "8: 256 chips / symbol"]
    _256 = 8,
    #[doc = "9: 512 chips / symbol"]
    _512 = 9,
    #[doc = "10: 1024 chips / symbol"]
    _1024 = 10,
    #[doc = "11: 2048 chips / symbol"]
    _2048 = 11,
    #[doc = "12: 4096 chips / symbol"]
    _4096 = 12,
}
impl From<SPREADING_FACTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPREADING_FACTOR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPREADING_FACTOR`"]
pub type SPREADING_FACTOR_R = crate::R<u8, SPREADING_FACTOR_A>;
impl SPREADING_FACTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPREADING_FACTOR_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(SPREADING_FACTOR_A::_64),
            7 => Val(SPREADING_FACTOR_A::_128),
            8 => Val(SPREADING_FACTOR_A::_256),
            9 => Val(SPREADING_FACTOR_A::_512),
            10 => Val(SPREADING_FACTOR_A::_1024),
            11 => Val(SPREADING_FACTOR_A::_2048),
            12 => Val(SPREADING_FACTOR_A::_4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SPREADING_FACTOR_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SPREADING_FACTOR_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SPREADING_FACTOR_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SPREADING_FACTOR_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SPREADING_FACTOR_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == SPREADING_FACTOR_A::_2048
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == SPREADING_FACTOR_A::_4096
    }
}
#[doc = "Write proxy for field `SPREADING_FACTOR`"]
pub struct SPREADING_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREADING_FACTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPREADING_FACTOR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 chips / symbol"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_64)
    }
    #[doc = "128 chips / symbol"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_128)
    }
    #[doc = "256 chips / symbol"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_256)
    }
    #[doc = "512 chips / symbol"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_512)
    }
    #[doc = "1024 chips / symbol"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_1024)
    }
    #[doc = "2048 chips / symbol"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_2048)
    }
    #[doc = "4096 chips / symbol"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(SPREADING_FACTOR_A::_4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RX timeout MSB"]
    #[inline(always)]
    pub fn symb_timeout_msb(&self) -> SYMB_TIMEOUT_MSB_R {
        SYMB_TIMEOUT_MSB_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - CRC information extracted from the received packet header"]
    #[inline(always)]
    pub fn rx_payload_crc_on(&self) -> RX_PAYLOAD_CRC_ON_R {
        RX_PAYLOAD_CRC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Whether Tx is in continuous mode or single packet mode"]
    #[inline(always)]
    pub fn tx_continuous_mode(&self) -> TX_CONTINUOUS_MODE_R {
        TX_CONTINUOUS_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Spreading factor (SF) rate (expressed as a base-2 logarithm)"]
    #[inline(always)]
    pub fn spreading_factor(&self) -> SPREADING_FACTOR_R {
        SPREADING_FACTOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RX timeout MSB"]
    #[inline(always)]
    pub fn symb_timeout_msb(&mut self) -> SYMB_TIMEOUT_MSB_W {
        SYMB_TIMEOUT_MSB_W { w: self }
    }
    #[doc = "Bit 2 - CRC information extracted from the received packet header"]
    #[inline(always)]
    pub fn rx_payload_crc_on(&mut self) -> RX_PAYLOAD_CRC_ON_W {
        RX_PAYLOAD_CRC_ON_W { w: self }
    }
    #[doc = "Bit 3 - Whether Tx is in continuous mode or single packet mode"]
    #[inline(always)]
    pub fn tx_continuous_mode(&mut self) -> TX_CONTINUOUS_MODE_W {
        TX_CONTINUOUS_MODE_W { w: self }
    }
    #[doc = "Bits 4:7 - Spreading factor (SF) rate (expressed as a base-2 logarithm)"]
    #[inline(always)]
    pub fn spreading_factor(&mut self) -> SPREADING_FACTOR_W {
        SPREADING_FACTOR_W { w: self }
    }
}
