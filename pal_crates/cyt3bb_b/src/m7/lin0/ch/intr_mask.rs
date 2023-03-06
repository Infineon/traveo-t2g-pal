#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HEADER_DONE` reader - Mask for corresponding field in INTR register."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_DONE` writer - Mask for corresponding field in INTR register."]
pub type TX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_DONE` reader - Mask for corresponding field in INTR register."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` writer - Mask for corresponding field in INTR register."]
pub type TX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TX_WAKEUP_DONE` reader - Mask for corresponding field in INTR register."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` writer - Mask for corresponding field in INTR register."]
pub type TX_WAKEUP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_DONE` reader - Mask for corresponding field in INTR register."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` writer - Mask for corresponding field in INTR register."]
pub type RX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_DONE` reader - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` writer - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_BREAK_WAKEUP_DONE` reader - Mask for corresponding field in INTR register."]
pub type RX_BREAK_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_BREAK_WAKEUP_DONE` writer - Mask for corresponding field in INTR register."]
pub type RX_BREAK_WAKEUP_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_SYNC_DONE` reader - Mask for corresponding field in INTR register."]
pub type RX_HEADER_SYNC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_DONE` writer - Mask for corresponding field in INTR register."]
pub type RX_HEADER_SYNC_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_NOISE_DETECT` reader - Mask for corresponding field in INTR register."]
pub type RX_NOISE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOISE_DETECT` writer - Mask for corresponding field in INTR register."]
pub type RX_NOISE_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - Mask for corresponding field in INTR register."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Mask for corresponding field in INTR register."]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TX_HEADER_BIT_ERROR` reader - Mask for corresponding field in INTR register."]
pub type TX_HEADER_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_BIT_ERROR` writer - Mask for corresponding field in INTR register."]
pub type TX_HEADER_BIT_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` reader - Mask for corresponding field in INTR register."]
pub type TX_RESPONSE_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` writer - Mask for corresponding field in INTR register."]
pub type TX_RESPONSE_BIT_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` reader - Mask for corresponding field in INTR register."]
pub type RX_HEADER_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` writer - Mask for corresponding field in INTR register."]
pub type RX_HEADER_FRAME_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` reader - Mask for corresponding field in INTR register."]
pub type RX_HEADER_SYNC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` writer - Mask for corresponding field in INTR register."]
pub type RX_HEADER_SYNC_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Mask for corresponding field in INTR register."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` writer - Mask for corresponding field in INTR register."]
pub type RX_HEADER_PARITY_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` reader - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` writer - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_FRAME_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` reader - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_CHECKSUM_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` writer - Mask for corresponding field in INTR register."]
pub type RX_RESPONSE_CHECKSUM_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_break_wakeup_done(&self) -> RX_BREAK_WAKEUP_DONE_R {
        RX_BREAK_WAKEUP_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_header_sync_done(&self) -> RX_HEADER_SYNC_DONE_R {
        RX_HEADER_SYNC_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_noise_detect(&self) -> RX_NOISE_DETECT_R {
        RX_NOISE_DETECT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn tx_header_bit_error(&self) -> TX_HEADER_BIT_ERROR_R {
        TX_HEADER_BIT_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn tx_response_bit_error(&self) -> TX_RESPONSE_BIT_ERROR_R {
        TX_RESPONSE_BIT_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_header_frame_error(&self) -> RX_HEADER_FRAME_ERROR_R {
        RX_HEADER_FRAME_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_header_sync_error(&self) -> RX_HEADER_SYNC_ERROR_R {
        RX_HEADER_SYNC_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_response_frame_error(&self) -> RX_RESPONSE_FRAME_ERROR_R {
        RX_RESPONSE_FRAME_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn rx_response_checksum_error(&self) -> RX_RESPONSE_CHECKSUM_ERROR_R {
        RX_RESPONSE_CHECKSUM_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_done(&mut self) -> TX_HEADER_DONE_W<0> {
        TX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_done(&mut self) -> TX_RESPONSE_DONE_W<1> {
        TX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 2 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wakeup_done(&mut self) -> TX_WAKEUP_DONE_W<2> {
        TX_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 8 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_done(&mut self) -> RX_HEADER_DONE_W<8> {
        RX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 9 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_done(&mut self) -> RX_RESPONSE_DONE_W<9> {
        RX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 10 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_break_wakeup_done(&mut self) -> RX_BREAK_WAKEUP_DONE_W<10> {
        RX_BREAK_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 11 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_sync_done(&mut self) -> RX_HEADER_SYNC_DONE_W<11> {
        RX_HEADER_SYNC_DONE_W::new(self)
    }
    #[doc = "Bit 13 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_noise_detect(&mut self) -> RX_NOISE_DETECT_W<13> {
        RX_NOISE_DETECT_W::new(self)
    }
    #[doc = "Bit 14 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_bit_error(&mut self) -> TX_HEADER_BIT_ERROR_W<16> {
        TX_HEADER_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_bit_error(&mut self) -> TX_RESPONSE_BIT_ERROR_W<17> {
        TX_RESPONSE_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 24 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_frame_error(&mut self) -> RX_HEADER_FRAME_ERROR_W<24> {
        RX_HEADER_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 25 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_sync_error(&mut self) -> RX_HEADER_SYNC_ERROR_W<25> {
        RX_HEADER_SYNC_ERROR_W::new(self)
    }
    #[doc = "Bit 26 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_parity_error(&mut self) -> RX_HEADER_PARITY_ERROR_W<26> {
        RX_HEADER_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 27 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_frame_error(&mut self) -> RX_RESPONSE_FRAME_ERROR_W<27> {
        RX_RESPONSE_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 28 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_checksum_error(&mut self) -> RX_RESPONSE_CHECKSUM_ERROR_W<28> {
        RX_RESPONSE_CHECKSUM_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
