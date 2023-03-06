#[doc = "Register `WRHS3_MIR` reader"]
pub struct R(crate::R<WRHS3_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRHS3_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRHS3_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRHS3_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRHS3_MIR` writer"]
pub struct W(crate::W<WRHS3_MIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRHS3_MIR_SPEC>;
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
impl From<crate::W<WRHS3_MIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRHS3_MIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DP` reader - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
pub type DP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DP` writer - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
pub type DP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRHS3_MIR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(&self) -> DP_R {
        DP_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
    #[inline(always)]
    #[must_use]
    pub fn dp(&mut self) -> DP_W<0> {
        DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Header Section 3 (mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrhs3_mir](index.html) module"]
pub struct WRHS3_MIR_SPEC;
impl crate::RegisterSpec for WRHS3_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrhs3_mir::R](R) reader structure"]
impl crate::Readable for WRHS3_MIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrhs3_mir::W](W) writer structure"]
impl crate::Writable for WRHS3_MIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRHS3_MIR to value 0"]
impl crate::Resettable for WRHS3_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
