#[doc = "Register `IBCM_MIR` reader"]
pub struct R(crate::R<IBCM_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBCM_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBCM_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBCM_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBCM_MIR` writer"]
pub struct W(crate::W<IBCM_MIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBCM_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IBCM_MIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBCM_MIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LHSH` reader - Load Header Section Host 1 = Header section selected for transfer from Input Buffer to the Message RAM 0 = Header section is not updated"]
pub type LHSH_R = crate::BitReader<LHSH_A>;
#[doc = "Load Header Section Host 1 = Header section selected for transfer from Input Buffer to the Message RAM 0 = Header section is not updated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LHSH_A {
    #[doc = "0: N/A"]
    HEADER_SECTION_NOT_UPDATED = 0,
    #[doc = "1: N/A"]
    HEADER_SECTION_UPDATED = 1,
}
impl From<LHSH_A> for bool {
    #[inline(always)]
    fn from(variant: LHSH_A) -> Self {
        variant as u8 != 0
    }
}
impl LHSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LHSH_A {
        match self.bits {
            false => LHSH_A::HEADER_SECTION_NOT_UPDATED,
            true => LHSH_A::HEADER_SECTION_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_header_section_not_updated(&self) -> bool {
        *self == LHSH_A::HEADER_SECTION_NOT_UPDATED
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_UPDATED`"]
    #[inline(always)]
    pub fn is_header_section_updated(&self) -> bool {
        *self == LHSH_A::HEADER_SECTION_UPDATED
    }
}
#[doc = "Field `LHSH` writer - Load Header Section Host 1 = Header section selected for transfer from Input Buffer to the Message RAM 0 = Header section is not updated"]
pub type LHSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBCM_MIR_SPEC, LHSH_A, O>;
impl<'a, const O: u8> LHSH_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_section_not_updated(self) -> &'a mut W {
        self.variant(LHSH_A::HEADER_SECTION_NOT_UPDATED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_section_updated(self) -> &'a mut W {
        self.variant(LHSH_A::HEADER_SECTION_UPDATED)
    }
}
#[doc = "Field `LDSH` reader - Load Data Section Host 1 = Data section selected for transfer from Input Buffer to the Message RAM 0 = Data section is not updated"]
pub type LDSH_R = crate::BitReader<LDSH_A>;
#[doc = "Load Data Section Host 1 = Data section selected for transfer from Input Buffer to the Message RAM 0 = Data section is not updated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDSH_A {
    #[doc = "0: N/A"]
    DATA_SECTION_NOT_UPDATED = 0,
    #[doc = "1: N/A"]
    DATA_SECTION_UPDATED = 1,
}
impl From<LDSH_A> for bool {
    #[inline(always)]
    fn from(variant: LDSH_A) -> Self {
        variant as u8 != 0
    }
}
impl LDSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDSH_A {
        match self.bits {
            false => LDSH_A::DATA_SECTION_NOT_UPDATED,
            true => LDSH_A::DATA_SECTION_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_data_section_not_updated(&self) -> bool {
        *self == LDSH_A::DATA_SECTION_NOT_UPDATED
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_UPDATED`"]
    #[inline(always)]
    pub fn is_data_section_updated(&self) -> bool {
        *self == LDSH_A::DATA_SECTION_UPDATED
    }
}
#[doc = "Field `LDSH` writer - Load Data Section Host 1 = Data section selected for transfer from Input Buffer to the Message RAM 0 = Data section is not updated"]
pub type LDSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBCM_MIR_SPEC, LDSH_A, O>;
impl<'a, const O: u8> LDSH_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_section_not_updated(self) -> &'a mut W {
        self.variant(LDSH_A::DATA_SECTION_NOT_UPDATED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_section_updated(self) -> &'a mut W {
        self.variant(LDSH_A::DATA_SECTION_UPDATED)
    }
}
#[doc = "Field `STXRH` reader - Set Transmission Request Host If this bit is set to '1', the TXR flag for the selected message buffer is set in the TXRQ1/2/3/4 registers to release the message buffer for transmission. In single-shot mode the flag is cleared by the CC after transmission has completed. TXR is evaluated for transmit buffers only. 1 = Set TXR flag, transmit buffer released for transmission 0 = Reset TXR flag"]
pub type STXRH_R = crate::BitReader<STXRH_A>;
#[doc = "Set Transmission Request Host If this bit is set to '1', the TXR flag for the selected message buffer is set in the TXRQ1/2/3/4 registers to release the message buffer for transmission. In single-shot mode the flag is cleared by the CC after transmission has completed. TXR is evaluated for transmit buffers only. 1 = Set TXR flag, transmit buffer released for transmission 0 = Reset TXR flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STXRH_A {
    #[doc = "0: N/A"]
    TXR_FLAG_RESET = 0,
    #[doc = "1: N/A"]
    TXR_FLAG_SET = 1,
}
impl From<STXRH_A> for bool {
    #[inline(always)]
    fn from(variant: STXRH_A) -> Self {
        variant as u8 != 0
    }
}
impl STXRH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STXRH_A {
        match self.bits {
            false => STXRH_A::TXR_FLAG_RESET,
            true => STXRH_A::TXR_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `TXR_FLAG_RESET`"]
    #[inline(always)]
    pub fn is_txr_flag_reset(&self) -> bool {
        *self == STXRH_A::TXR_FLAG_RESET
    }
    #[doc = "Checks if the value of the field is `TXR_FLAG_SET`"]
    #[inline(always)]
    pub fn is_txr_flag_set(&self) -> bool {
        *self == STXRH_A::TXR_FLAG_SET
    }
}
#[doc = "Field `STXRH` writer - Set Transmission Request Host If this bit is set to '1', the TXR flag for the selected message buffer is set in the TXRQ1/2/3/4 registers to release the message buffer for transmission. In single-shot mode the flag is cleared by the CC after transmission has completed. TXR is evaluated for transmit buffers only. 1 = Set TXR flag, transmit buffer released for transmission 0 = Reset TXR flag"]
pub type STXRH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IBCM_MIR_SPEC, STXRH_A, O>;
impl<'a, const O: u8> STXRH_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn txr_flag_reset(self) -> &'a mut W {
        self.variant(STXRH_A::TXR_FLAG_RESET)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn txr_flag_set(self) -> &'a mut W {
        self.variant(STXRH_A::TXR_FLAG_SET)
    }
}
#[doc = "Field `LHSS` reader - Load Header Section Shadow 1 = Header section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Header section is not updated"]
pub type LHSS_R = crate::BitReader<LHSS_A>;
#[doc = "Load Header Section Shadow 1 = Header section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Header section is not updated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LHSS_A {
    #[doc = "0: N/A"]
    HEADER_SECTION_NOT_UPDATED = 0,
    #[doc = "1: N/A"]
    HEADER_SECTION_UPDATED = 1,
}
impl From<LHSS_A> for bool {
    #[inline(always)]
    fn from(variant: LHSS_A) -> Self {
        variant as u8 != 0
    }
}
impl LHSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LHSS_A {
        match self.bits {
            false => LHSS_A::HEADER_SECTION_NOT_UPDATED,
            true => LHSS_A::HEADER_SECTION_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_header_section_not_updated(&self) -> bool {
        *self == LHSS_A::HEADER_SECTION_NOT_UPDATED
    }
    #[doc = "Checks if the value of the field is `HEADER_SECTION_UPDATED`"]
    #[inline(always)]
    pub fn is_header_section_updated(&self) -> bool {
        *self == LHSS_A::HEADER_SECTION_UPDATED
    }
}
#[doc = "Field `LDSS` reader - Load Data Section Shadow 1 = Data section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Data section is not updated"]
pub type LDSS_R = crate::BitReader<LDSS_A>;
#[doc = "Load Data Section Shadow 1 = Data section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Data section is not updated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDSS_A {
    #[doc = "0: N/A"]
    DATA_SECTION_NOT_UPDATED = 0,
    #[doc = "1: N/A"]
    DATA_SECTION_UPDATED = 1,
}
impl From<LDSS_A> for bool {
    #[inline(always)]
    fn from(variant: LDSS_A) -> Self {
        variant as u8 != 0
    }
}
impl LDSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDSS_A {
        match self.bits {
            false => LDSS_A::DATA_SECTION_NOT_UPDATED,
            true => LDSS_A::DATA_SECTION_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_data_section_not_updated(&self) -> bool {
        *self == LDSS_A::DATA_SECTION_NOT_UPDATED
    }
    #[doc = "Checks if the value of the field is `DATA_SECTION_UPDATED`"]
    #[inline(always)]
    pub fn is_data_section_updated(&self) -> bool {
        *self == LDSS_A::DATA_SECTION_UPDATED
    }
}
#[doc = "Field `STXRS` reader - Set Transmission Request Shadow 1 = Set TXR flag, transmit buffer released for transmission (operation ongoing or finished) 0 = Reset TXR flag"]
pub type STXRS_R = crate::BitReader<STXRS_A>;
#[doc = "Set Transmission Request Shadow 1 = Set TXR flag, transmit buffer released for transmission (operation ongoing or finished) 0 = Reset TXR flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STXRS_A {
    #[doc = "0: N/A"]
    TXR_FLAG_RESET = 0,
    #[doc = "1: N/A"]
    TXR_FLAG_SET = 1,
}
impl From<STXRS_A> for bool {
    #[inline(always)]
    fn from(variant: STXRS_A) -> Self {
        variant as u8 != 0
    }
}
impl STXRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STXRS_A {
        match self.bits {
            false => STXRS_A::TXR_FLAG_RESET,
            true => STXRS_A::TXR_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `TXR_FLAG_RESET`"]
    #[inline(always)]
    pub fn is_txr_flag_reset(&self) -> bool {
        *self == STXRS_A::TXR_FLAG_RESET
    }
    #[doc = "Checks if the value of the field is `TXR_FLAG_SET`"]
    #[inline(always)]
    pub fn is_txr_flag_set(&self) -> bool {
        *self == STXRS_A::TXR_FLAG_SET
    }
}
impl R {
    #[doc = "Bit 0 - Load Header Section Host 1 = Header section selected for transfer from Input Buffer to the Message RAM 0 = Header section is not updated"]
    #[inline(always)]
    pub fn lhsh(&self) -> LHSH_R {
        LHSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Data Section Host 1 = Data section selected for transfer from Input Buffer to the Message RAM 0 = Data section is not updated"]
    #[inline(always)]
    pub fn ldsh(&self) -> LDSH_R {
        LDSH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set Transmission Request Host If this bit is set to '1', the TXR flag for the selected message buffer is set in the TXRQ1/2/3/4 registers to release the message buffer for transmission. In single-shot mode the flag is cleared by the CC after transmission has completed. TXR is evaluated for transmit buffers only. 1 = Set TXR flag, transmit buffer released for transmission 0 = Reset TXR flag"]
    #[inline(always)]
    pub fn stxrh(&self) -> STXRH_R {
        STXRH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Load Header Section Shadow 1 = Header section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Header section is not updated"]
    #[inline(always)]
    pub fn lhss(&self) -> LHSS_R {
        LHSS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Load Data Section Shadow 1 = Data section selected for transfer from Input Buffer to the Message RAM transfer ongoing or finished) 0 = Data section is not updated"]
    #[inline(always)]
    pub fn ldss(&self) -> LDSS_R {
        LDSS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set Transmission Request Shadow 1 = Set TXR flag, transmit buffer released for transmission (operation ongoing or finished) 0 = Reset TXR flag"]
    #[inline(always)]
    pub fn stxrs(&self) -> STXRS_R {
        STXRS_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Load Header Section Host 1 = Header section selected for transfer from Input Buffer to the Message RAM 0 = Header section is not updated"]
    #[inline(always)]
    #[must_use]
    pub fn lhsh(&mut self) -> LHSH_W<0> {
        LHSH_W::new(self)
    }
    #[doc = "Bit 1 - Load Data Section Host 1 = Data section selected for transfer from Input Buffer to the Message RAM 0 = Data section is not updated"]
    #[inline(always)]
    #[must_use]
    pub fn ldsh(&mut self) -> LDSH_W<1> {
        LDSH_W::new(self)
    }
    #[doc = "Bit 2 - Set Transmission Request Host If this bit is set to '1', the TXR flag for the selected message buffer is set in the TXRQ1/2/3/4 registers to release the message buffer for transmission. In single-shot mode the flag is cleared by the CC after transmission has completed. TXR is evaluated for transmit buffers only. 1 = Set TXR flag, transmit buffer released for transmission 0 = Reset TXR flag"]
    #[inline(always)]
    #[must_use]
    pub fn stxrh(&mut self) -> STXRH_W<2> {
        STXRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Buffer Command Mask (mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibcm_mir](index.html) module"]
pub struct IBCM_MIR_SPEC;
impl crate::RegisterSpec for IBCM_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibcm_mir::R](R) reader structure"]
impl crate::Readable for IBCM_MIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibcm_mir::W](W) writer structure"]
impl crate::Writable for IBCM_MIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBCM_MIR to value 0"]
impl crate::Resettable for IBCM_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
