#[doc = "Register `TXBCIE` reader"]
pub struct R(crate::R<TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCIE` writer"]
pub struct W(crate::W<TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCIE_SPEC>;
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
impl From<crate::W<TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFIE` reader - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
pub type CFIE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFIE` writer - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
pub type CFIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBCIE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CFIE_W<0> {
        CFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](index.html) module"]
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcie::R](R) reader structure"]
impl crate::Readable for TXBCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcie::W](W) writer structure"]
impl crate::Writable for TXBCIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TXBCIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
