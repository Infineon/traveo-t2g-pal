#[doc = "Register `EIES` reader"]
pub struct R(crate::R<EIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIES` writer"]
pub struct W(crate::W<EIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIES_SPEC>;
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
impl From<crate::W<EIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEMCE` reader - POC Error Mode Changed Interrupt Enable"]
pub type PEMCE_R = crate::BitReader<bool>;
#[doc = "Field `PEMCE` writer - POC Error Mode Changed Interrupt Enable"]
pub type PEMCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `CNAE` reader - Command Not Accepted Interrupt Enable"]
pub type CNAE_R = crate::BitReader<bool>;
#[doc = "Field `CNAE` writer - Command Not Accepted Interrupt Enable"]
pub type CNAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `SFBME` reader - Sync Frames Below Minimum Interrupt Enable"]
pub type SFBME_R = crate::BitReader<bool>;
#[doc = "Field `SFBME` writer - Sync Frames Below Minimum Interrupt Enable"]
pub type SFBME_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `SFOE` reader - Sync Frame Overflow Interrupt Enable"]
pub type SFOE_R = crate::BitReader<bool>;
#[doc = "Field `SFOE` writer - Sync Frame Overflow Interrupt Enable"]
pub type SFOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `CCFE` reader - Clock Correction Failure Interrupt Enable"]
pub type CCFE_R = crate::BitReader<bool>;
#[doc = "Field `CCFE` writer - Clock Correction Failure Interrupt Enable"]
pub type CCFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `CCLE` reader - CHI Command Locked Interrupt Enable"]
pub type CCLE_R = crate::BitReader<bool>;
#[doc = "Field `CCLE` writer - CHI Command Locked Interrupt Enable"]
pub type CCLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `PERRE` reader - Parity Error Interrupt Enable"]
pub type PERRE_R = crate::BitReader<bool>;
#[doc = "Field `PERRE` writer - Parity Error Interrupt Enable"]
pub type PERRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `RFOE` reader - Receive FIFO Overrun Interrupt Enable"]
pub type RFOE_R = crate::BitReader<bool>;
#[doc = "Field `RFOE` writer - Receive FIFO Overrun Interrupt Enable"]
pub type RFOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `EFAE` reader - Empty FIFO Access Interrupt Enable"]
pub type EFAE_R = crate::BitReader<bool>;
#[doc = "Field `EFAE` writer - Empty FIFO Access Interrupt Enable"]
pub type EFAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `IIBAE` reader - Illegal Input Buffer Access Interrupt Enable"]
pub type IIBAE_R = crate::BitReader<bool>;
#[doc = "Field `IIBAE` writer - Illegal Input Buffer Access Interrupt Enable"]
pub type IIBAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `IOBAE` reader - Illegal Output Buffer Access Interrupt Enable"]
pub type IOBAE_R = crate::BitReader<bool>;
#[doc = "Field `IOBAE` writer - Illegal Output Buffer Access Interrupt Enable"]
pub type IOBAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `MHFE` reader - Message Handler Constraints Flag Interrupt Enable"]
pub type MHFE_R = crate::BitReader<bool>;
#[doc = "Field `MHFE` writer - Message Handler Constraints Flag Interrupt Enable"]
pub type MHFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `EDAE` reader - Error Detected on Channel A Interrupt Enable"]
pub type EDAE_R = crate::BitReader<bool>;
#[doc = "Field `EDAE` writer - Error Detected on Channel A Interrupt Enable"]
pub type EDAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `LTVAE` reader - Latest Transmit Violation Channel A Interrupt Enable"]
pub type LTVAE_R = crate::BitReader<bool>;
#[doc = "Field `LTVAE` writer - Latest Transmit Violation Channel A Interrupt Enable"]
pub type LTVAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `TABAE` reader - Transmission Across Boundary Channel A Interrupt Enable"]
pub type TABAE_R = crate::BitReader<bool>;
#[doc = "Field `TABAE` writer - Transmission Across Boundary Channel A Interrupt Enable"]
pub type TABAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `EDBE` reader - Error Detected on Channel B Interrupt Enable"]
pub type EDBE_R = crate::BitReader<bool>;
#[doc = "Field `EDBE` writer - Error Detected on Channel B Interrupt Enable"]
pub type EDBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `LTVBE` reader - Latest Transmit Violation Channel B Interrupt Enable"]
pub type LTVBE_R = crate::BitReader<bool>;
#[doc = "Field `LTVBE` writer - Latest Transmit Violation Channel B Interrupt Enable"]
pub type LTVBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
#[doc = "Field `TABBE` reader - Transmission Across Boundary Channel B Interrupt Enable"]
pub type TABBE_R = crate::BitReader<bool>;
#[doc = "Field `TABBE` writer - Transmission Across Boundary Channel B Interrupt Enable"]
pub type TABBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIES_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - POC Error Mode Changed Interrupt Enable"]
    #[inline(always)]
    pub fn pemce(&self) -> PEMCE_R {
        PEMCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Not Accepted Interrupt Enable"]
    #[inline(always)]
    pub fn cnae(&self) -> CNAE_R {
        CNAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum Interrupt Enable"]
    #[inline(always)]
    pub fn sfbme(&self) -> SFBME_R {
        SFBME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sfoe(&self) -> SFOE_R {
        SFOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Correction Failure Interrupt Enable"]
    #[inline(always)]
    pub fn ccfe(&self) -> CCFE_R {
        CCFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHI Command Locked Interrupt Enable"]
    #[inline(always)]
    pub fn ccle(&self) -> CCLE_R {
        CCLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rfoe(&self) -> RFOE_R {
        RFOE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Empty FIFO Access Interrupt Enable"]
    #[inline(always)]
    pub fn efae(&self) -> EFAE_R {
        EFAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access Interrupt Enable"]
    #[inline(always)]
    pub fn iibae(&self) -> IIBAE_R {
        IIBAE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Illegal Output Buffer Access Interrupt Enable"]
    #[inline(always)]
    pub fn iobae(&self) -> IOBAE_R {
        IOBAE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag Interrupt Enable"]
    #[inline(always)]
    pub fn mhfe(&self) -> MHFE_R {
        MHFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Detected on Channel A Interrupt Enable"]
    #[inline(always)]
    pub fn edae(&self) -> EDAE_R {
        EDAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A Interrupt Enable"]
    #[inline(always)]
    pub fn ltvae(&self) -> LTVAE_R {
        LTVAE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A Interrupt Enable"]
    #[inline(always)]
    pub fn tabae(&self) -> TABAE_R {
        TABAE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Detected on Channel B Interrupt Enable"]
    #[inline(always)]
    pub fn edbe(&self) -> EDBE_R {
        EDBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B Interrupt Enable"]
    #[inline(always)]
    pub fn ltvbe(&self) -> LTVBE_R {
        LTVBE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B Interrupt Enable"]
    #[inline(always)]
    pub fn tabbe(&self) -> TABBE_R {
        TABBE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POC Error Mode Changed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pemce(&mut self) -> PEMCE_W<0> {
        PEMCE_W::new(self)
    }
    #[doc = "Bit 1 - Command Not Accepted Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnae(&mut self) -> CNAE_W<1> {
        CNAE_W::new(self)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfbme(&mut self) -> SFBME_W<2> {
        SFBME_W::new(self)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfoe(&mut self) -> SFOE_W<3> {
        SFOE_W::new(self)
    }
    #[doc = "Bit 4 - Clock Correction Failure Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccfe(&mut self) -> CCFE_W<4> {
        CCFE_W::new(self)
    }
    #[doc = "Bit 5 - CHI Command Locked Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccle(&mut self) -> CCLE_W<5> {
        CCLE_W::new(self)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perre(&mut self) -> PERRE_W<6> {
        PERRE_W::new(self)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoe(&mut self) -> RFOE_W<7> {
        RFOE_W::new(self)
    }
    #[doc = "Bit 8 - Empty FIFO Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn efae(&mut self) -> EFAE_W<8> {
        EFAE_W::new(self)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iibae(&mut self) -> IIBAE_W<9> {
        IIBAE_W::new(self)
    }
    #[doc = "Bit 10 - Illegal Output Buffer Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iobae(&mut self) -> IOBAE_W<10> {
        IOBAE_W::new(self)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mhfe(&mut self) -> MHFE_W<11> {
        MHFE_W::new(self)
    }
    #[doc = "Bit 16 - Error Detected on Channel A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edae(&mut self) -> EDAE_W<16> {
        EDAE_W::new(self)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltvae(&mut self) -> LTVAE_W<17> {
        LTVAE_W::new(self)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabae(&mut self) -> TABAE_W<18> {
        TABAE_W::new(self)
    }
    #[doc = "Bit 24 - Error Detected on Channel B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edbe(&mut self) -> EDBE_W<24> {
        EDBE_W::new(self)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltvbe(&mut self) -> LTVBE_W<25> {
        LTVBE_W::new(self)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabbe(&mut self) -> TABBE_W<26> {
        TABBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eies](index.html) module"]
pub struct EIES_SPEC;
impl crate::RegisterSpec for EIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eies::R](R) reader structure"]
impl crate::Readable for EIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eies::W](W) writer structure"]
impl crate::Writable for EIES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIES to value 0"]
impl crate::Resettable for EIES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
