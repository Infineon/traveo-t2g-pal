#[doc = "Register `EILS` reader"]
pub struct R(crate::R<EILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EILS` writer"]
pub struct W(crate::W<EILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EILS_SPEC>;
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
impl From<crate::W<EILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEMCL` reader - POC Error Mode Changed Interrupt Line"]
pub type PEMCL_R = crate::BitReader<bool>;
#[doc = "Field `PEMCL` writer - POC Error Mode Changed Interrupt Line"]
pub type PEMCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `CNAL` reader - Command Not Accepted Interrupt Line"]
pub type CNAL_R = crate::BitReader<bool>;
#[doc = "Field `CNAL` writer - Command Not Accepted Interrupt Line"]
pub type CNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `SFBML` reader - Sync Frames Below Minimum Interrupt Line"]
pub type SFBML_R = crate::BitReader<bool>;
#[doc = "Field `SFBML` writer - Sync Frames Below Minimum Interrupt Line"]
pub type SFBML_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `SFOL` reader - Sync Frame Overflow Interrupt Line"]
pub type SFOL_R = crate::BitReader<bool>;
#[doc = "Field `SFOL` writer - Sync Frame Overflow Interrupt Line"]
pub type SFOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `CCFL` reader - Clock Correction Failure Interrupt Line"]
pub type CCFL_R = crate::BitReader<bool>;
#[doc = "Field `CCFL` writer - Clock Correction Failure Interrupt Line"]
pub type CCFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `CCLL` reader - CHI Command Locked Interrupt Line"]
pub type CCLL_R = crate::BitReader<bool>;
#[doc = "Field `CCLL` writer - CHI Command Locked Interrupt Line"]
pub type CCLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `PERRL` reader - Parity Error Interrupt Line"]
pub type PERRL_R = crate::BitReader<bool>;
#[doc = "Field `PERRL` writer - Parity Error Interrupt Line"]
pub type PERRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `RFOL` reader - Receive FIFO Overrun Interrupt Line"]
pub type RFOL_R = crate::BitReader<bool>;
#[doc = "Field `RFOL` writer - Receive FIFO Overrun Interrupt Line"]
pub type RFOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `EFAL` reader - Empty FIFO Access Interrupt Line"]
pub type EFAL_R = crate::BitReader<bool>;
#[doc = "Field `EFAL` writer - Empty FIFO Access Interrupt Line"]
pub type EFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `IIBAL` reader - Illegal Input Buffer Access Interrupt Line"]
pub type IIBAL_R = crate::BitReader<bool>;
#[doc = "Field `IIBAL` writer - Illegal Input Buffer Access Interrupt Line"]
pub type IIBAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `IOBAL` reader - Illegal Output Buffer Access Interrupt Line"]
pub type IOBAL_R = crate::BitReader<bool>;
#[doc = "Field `IOBAL` writer - Illegal Output Buffer Access Interrupt Line"]
pub type IOBAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `MHFL` reader - Message Handler Constraints Flag Interrupt Line"]
pub type MHFL_R = crate::BitReader<bool>;
#[doc = "Field `MHFL` writer - Message Handler Constraints Flag Interrupt Line"]
pub type MHFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `EDAL` reader - Error Detected on Channel A Interrupt Line"]
pub type EDAL_R = crate::BitReader<bool>;
#[doc = "Field `EDAL` writer - Error Detected on Channel A Interrupt Line"]
pub type EDAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `LTVAL` reader - Latest Transmit Violation Channel A Interrupt Line"]
pub type LTVAL_R = crate::BitReader<bool>;
#[doc = "Field `LTVAL` writer - Latest Transmit Violation Channel A Interrupt Line"]
pub type LTVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `TABAL` reader - Transmission Across Boundary Channel A Interrupt Line"]
pub type TABAL_R = crate::BitReader<bool>;
#[doc = "Field `TABAL` writer - Transmission Across Boundary Channel A Interrupt Line"]
pub type TABAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `EDBL` reader - Error Detected on Channel B Interrupt Line"]
pub type EDBL_R = crate::BitReader<bool>;
#[doc = "Field `EDBL` writer - Error Detected on Channel B Interrupt Line"]
pub type EDBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `LTVBL` reader - Latest Transmit Violation Channel B Interrupt Line"]
pub type LTVBL_R = crate::BitReader<bool>;
#[doc = "Field `LTVBL` writer - Latest Transmit Violation Channel B Interrupt Line"]
pub type LTVBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
#[doc = "Field `TABBL` reader - Transmission Across Boundary Channel B Interrupt Line"]
pub type TABBL_R = crate::BitReader<bool>;
#[doc = "Field `TABBL` writer - Transmission Across Boundary Channel B Interrupt Line"]
pub type TABBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EILS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - POC Error Mode Changed Interrupt Line"]
    #[inline(always)]
    pub fn pemcl(&self) -> PEMCL_R {
        PEMCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Not Accepted Interrupt Line"]
    #[inline(always)]
    pub fn cnal(&self) -> CNAL_R {
        CNAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum Interrupt Line"]
    #[inline(always)]
    pub fn sfbml(&self) -> SFBML_R {
        SFBML_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Interrupt Line"]
    #[inline(always)]
    pub fn sfol(&self) -> SFOL_R {
        SFOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Correction Failure Interrupt Line"]
    #[inline(always)]
    pub fn ccfl(&self) -> CCFL_R {
        CCFL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHI Command Locked Interrupt Line"]
    #[inline(always)]
    pub fn ccll(&self) -> CCLL_R {
        CCLL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Line"]
    #[inline(always)]
    pub fn perrl(&self) -> PERRL_R {
        PERRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun Interrupt Line"]
    #[inline(always)]
    pub fn rfol(&self) -> RFOL_R {
        RFOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Empty FIFO Access Interrupt Line"]
    #[inline(always)]
    pub fn efal(&self) -> EFAL_R {
        EFAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access Interrupt Line"]
    #[inline(always)]
    pub fn iibal(&self) -> IIBAL_R {
        IIBAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Illegal Output Buffer Access Interrupt Line"]
    #[inline(always)]
    pub fn iobal(&self) -> IOBAL_R {
        IOBAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag Interrupt Line"]
    #[inline(always)]
    pub fn mhfl(&self) -> MHFL_R {
        MHFL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Detected on Channel A Interrupt Line"]
    #[inline(always)]
    pub fn edal(&self) -> EDAL_R {
        EDAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A Interrupt Line"]
    #[inline(always)]
    pub fn ltval(&self) -> LTVAL_R {
        LTVAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A Interrupt Line"]
    #[inline(always)]
    pub fn tabal(&self) -> TABAL_R {
        TABAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Detected on Channel B Interrupt Line"]
    #[inline(always)]
    pub fn edbl(&self) -> EDBL_R {
        EDBL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B Interrupt Line"]
    #[inline(always)]
    pub fn ltvbl(&self) -> LTVBL_R {
        LTVBL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B Interrupt Line"]
    #[inline(always)]
    pub fn tabbl(&self) -> TABBL_R {
        TABBL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POC Error Mode Changed Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn pemcl(&mut self) -> PEMCL_W<0> {
        PEMCL_W::new(self)
    }
    #[doc = "Bit 1 - Command Not Accepted Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn cnal(&mut self) -> CNAL_W<1> {
        CNAL_W::new(self)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sfbml(&mut self) -> SFBML_W<2> {
        SFBML_W::new(self)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sfol(&mut self) -> SFOL_W<3> {
        SFOL_W::new(self)
    }
    #[doc = "Bit 4 - Clock Correction Failure Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ccfl(&mut self) -> CCFL_W<4> {
        CCFL_W::new(self)
    }
    #[doc = "Bit 5 - CHI Command Locked Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ccll(&mut self) -> CCLL_W<5> {
        CCLL_W::new(self)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn perrl(&mut self) -> PERRL_W<6> {
        PERRL_W::new(self)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rfol(&mut self) -> RFOL_W<7> {
        RFOL_W::new(self)
    }
    #[doc = "Bit 8 - Empty FIFO Access Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn efal(&mut self) -> EFAL_W<8> {
        EFAL_W::new(self)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn iibal(&mut self) -> IIBAL_W<9> {
        IIBAL_W::new(self)
    }
    #[doc = "Bit 10 - Illegal Output Buffer Access Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn iobal(&mut self) -> IOBAL_W<10> {
        IOBAL_W::new(self)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn mhfl(&mut self) -> MHFL_W<11> {
        MHFL_W::new(self)
    }
    #[doc = "Bit 16 - Error Detected on Channel A Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn edal(&mut self) -> EDAL_W<16> {
        EDAL_W::new(self)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ltval(&mut self) -> LTVAL_W<17> {
        LTVAL_W::new(self)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tabal(&mut self) -> TABAL_W<18> {
        TABAL_W::new(self)
    }
    #[doc = "Bit 24 - Error Detected on Channel B Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn edbl(&mut self) -> EDBL_W<24> {
        EDBL_W::new(self)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn ltvbl(&mut self) -> LTVBL_W<25> {
        LTVBL_W::new(self)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tabbl(&mut self) -> TABBL_W<26> {
        TABBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Line Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eils](index.html) module"]
pub struct EILS_SPEC;
impl crate::RegisterSpec for EILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eils::R](R) reader structure"]
impl crate::Readable for EILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eils::W](W) writer structure"]
impl crate::Writable for EILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EILS to value 0"]
impl crate::Resettable for EILS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
