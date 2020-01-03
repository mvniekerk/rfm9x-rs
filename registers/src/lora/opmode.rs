#[doc = "Reader of register OPMODE"]
pub type R = crate::R<u8, super::OPMODE>;
#[doc = "Writer for register OPMODE"]
pub type W = crate::W<u8, super::OPMODE>;
#[doc = "Register OPMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::OPMODE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transceiver modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Sleep mode"]
    SLEEP = 0,
    #[doc = "1: Standby"]
    STDBY = 1,
    #[doc = "2: Fs mode TX"]
    FSTX = 2,
    #[doc = "3: Transmitter mode TX"]
    TX = 3,
    #[doc = "4: Frequency synthesis RX"]
    FSRX = 4,
    #[doc = "5: Receive continuous"]
    RXCONTINOUS = 5,
    #[doc = "6: Receive single"]
    RXSINGLE = 6,
    #[doc = "7: Channel activity detection"]
    CAD = 7,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::SLEEP,
            1 => MODE_A::STDBY,
            2 => MODE_A::FSTX,
            3 => MODE_A::TX,
            4 => MODE_A::FSRX,
            5 => MODE_A::RXCONTINOUS,
            6 => MODE_A::RXSINGLE,
            7 => MODE_A::CAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `STDBY`"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == MODE_A::STDBY
    }
    #[doc = "Checks if the value of the field is `FSTX`"]
    #[inline(always)]
    pub fn is_fstx(&self) -> bool {
        *self == MODE_A::FSTX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == MODE_A::TX
    }
    #[doc = "Checks if the value of the field is `FSRX`"]
    #[inline(always)]
    pub fn is_fsrx(&self) -> bool {
        *self == MODE_A::FSRX
    }
    #[doc = "Checks if the value of the field is `RXCONTINOUS`"]
    #[inline(always)]
    pub fn is_rxcontinous(&self) -> bool {
        *self == MODE_A::RXCONTINOUS
    }
    #[doc = "Checks if the value of the field is `RXSINGLE`"]
    #[inline(always)]
    pub fn is_rxsingle(&self) -> bool {
        *self == MODE_A::RXSINGLE
    }
    #[doc = "Checks if the value of the field is `CAD`"]
    #[inline(always)]
    pub fn is_cad(&self) -> bool {
        *self == MODE_A::CAD
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(MODE_A::SLEEP)
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut W {
        self.variant(MODE_A::STDBY)
    }
    #[doc = "Fs mode TX"]
    #[inline(always)]
    pub fn fstx(self) -> &'a mut W {
        self.variant(MODE_A::FSTX)
    }
    #[doc = "Transmitter mode TX"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(MODE_A::TX)
    }
    #[doc = "Frequency synthesis RX"]
    #[inline(always)]
    pub fn fsrx(self) -> &'a mut W {
        self.variant(MODE_A::FSRX)
    }
    #[doc = "Receive continuous"]
    #[inline(always)]
    pub fn rxcontinous(self) -> &'a mut W {
        self.variant(MODE_A::RXCONTINOUS)
    }
    #[doc = "Receive single"]
    #[inline(always)]
    pub fn rxsingle(self) -> &'a mut W {
        self.variant(MODE_A::RXSINGLE)
    }
    #[doc = "Channel activity detection"]
    #[inline(always)]
    pub fn cad(self) -> &'a mut W {
        self.variant(MODE_A::CAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Access low frequency mode registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOW_FREQ_MODE_A {
    #[doc = "0: High frequency mode (access to HF test registers)"]
    HF_MODE = 0,
    #[doc = "1: Low frequency mode (access to LF test registers)"]
    LF_MODE = 1,
}
impl From<LOW_FREQ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LOW_FREQ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOW_FREQ_MODE`"]
pub type LOW_FREQ_MODE_R = crate::R<bool, LOW_FREQ_MODE_A>;
impl LOW_FREQ_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOW_FREQ_MODE_A {
        match self.bits {
            false => LOW_FREQ_MODE_A::HF_MODE,
            true => LOW_FREQ_MODE_A::LF_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `HF_MODE`"]
    #[inline(always)]
    pub fn is_hf_mode(&self) -> bool {
        *self == LOW_FREQ_MODE_A::HF_MODE
    }
    #[doc = "Checks if the value of the field is `LF_MODE`"]
    #[inline(always)]
    pub fn is_lf_mode(&self) -> bool {
        *self == LOW_FREQ_MODE_A::LF_MODE
    }
}
#[doc = "Write proxy for field `LOW_FREQ_MODE`"]
pub struct LOW_FREQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_FREQ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOW_FREQ_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High frequency mode (access to HF test registers)"]
    #[inline(always)]
    pub fn hf_mode(self) -> &'a mut W {
        self.variant(LOW_FREQ_MODE_A::HF_MODE)
    }
    #[doc = "Low frequency mode (access to LF test registers)"]
    #[inline(always)]
    pub fn lf_mode(self) -> &'a mut W {
        self.variant(LOW_FREQ_MODE_A::LF_MODE)
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
#[doc = "Allows to access FSK registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCESS_SHARED_REG_A {
    #[doc = "0: Access LoRa registers page"]
    LORA = 0,
    #[doc = "1: Access FSK registers"]
    FSK = 1,
}
impl From<ACCESS_SHARED_REG_A> for bool {
    #[inline(always)]
    fn from(variant: ACCESS_SHARED_REG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACCESS_SHARED_REG`"]
pub type ACCESS_SHARED_REG_R = crate::R<bool, ACCESS_SHARED_REG_A>;
impl ACCESS_SHARED_REG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCESS_SHARED_REG_A {
        match self.bits {
            false => ACCESS_SHARED_REG_A::LORA,
            true => ACCESS_SHARED_REG_A::FSK,
        }
    }
    #[doc = "Checks if the value of the field is `LORA`"]
    #[inline(always)]
    pub fn is_lora(&self) -> bool {
        *self == ACCESS_SHARED_REG_A::LORA
    }
    #[doc = "Checks if the value of the field is `FSK`"]
    #[inline(always)]
    pub fn is_fsk(&self) -> bool {
        *self == ACCESS_SHARED_REG_A::FSK
    }
}
#[doc = "Write proxy for field `ACCESS_SHARED_REG`"]
pub struct ACCESS_SHARED_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCESS_SHARED_REG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCESS_SHARED_REG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access LoRa registers page"]
    #[inline(always)]
    pub fn lora(self) -> &'a mut W {
        self.variant(ACCESS_SHARED_REG_A::LORA)
    }
    #[doc = "Access FSK registers"]
    #[inline(always)]
    pub fn fsk(self) -> &'a mut W {
        self.variant(ACCESS_SHARED_REG_A::FSK)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Select whether the device should be in long range mode or FSK/OOK mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LORA_MODE_A {
    #[doc = "0: FSK/OOK mode"]
    FSK_OOK = 0,
    #[doc = "1: LoRa mode"]
    LORA = 1,
}
impl From<LORA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LORA_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LORA_MODE`"]
pub type LORA_MODE_R = crate::R<bool, LORA_MODE_A>;
impl LORA_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LORA_MODE_A {
        match self.bits {
            false => LORA_MODE_A::FSK_OOK,
            true => LORA_MODE_A::LORA,
        }
    }
    #[doc = "Checks if the value of the field is `FSK_OOK`"]
    #[inline(always)]
    pub fn is_fsk_ook(&self) -> bool {
        *self == LORA_MODE_A::FSK_OOK
    }
    #[doc = "Checks if the value of the field is `LORA`"]
    #[inline(always)]
    pub fn is_lora(&self) -> bool {
        *self == LORA_MODE_A::LORA
    }
}
#[doc = "Write proxy for field `LORA_MODE`"]
pub struct LORA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LORA_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LORA_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FSK/OOK mode"]
    #[inline(always)]
    pub fn fsk_ook(self) -> &'a mut W {
        self.variant(LORA_MODE_A::FSK_OOK)
    }
    #[doc = "LoRa mode"]
    #[inline(always)]
    pub fn lora(self) -> &'a mut W {
        self.variant(LORA_MODE_A::LORA)
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
    #[doc = "Bits 0:2 - Transceiver modes"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Access low frequency mode registers"]
    #[inline(always)]
    pub fn low_freq_mode(&self) -> LOW_FREQ_MODE_R {
        LOW_FREQ_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Allows to access FSK registers"]
    #[inline(always)]
    pub fn access_shared_reg(&self) -> ACCESS_SHARED_REG_R {
        ACCESS_SHARED_REG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select whether the device should be in long range mode or FSK/OOK mode"]
    #[inline(always)]
    pub fn lora_mode(&self) -> LORA_MODE_R {
        LORA_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transceiver modes"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Access low frequency mode registers"]
    #[inline(always)]
    pub fn low_freq_mode(&mut self) -> LOW_FREQ_MODE_W {
        LOW_FREQ_MODE_W { w: self }
    }
    #[doc = "Bit 6 - Allows to access FSK registers"]
    #[inline(always)]
    pub fn access_shared_reg(&mut self) -> ACCESS_SHARED_REG_W {
        ACCESS_SHARED_REG_W { w: self }
    }
    #[doc = "Bit 7 - Select whether the device should be in long range mode or FSK/OOK mode"]
    #[inline(always)]
    pub fn lora_mode(&mut self) -> LORA_MODE_W {
        LORA_MODE_W { w: self }
    }
}
