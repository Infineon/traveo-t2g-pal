#[doc = "Register `OUTBYPEN1` reader"]
pub struct R(crate::R<OUTBYPEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTBYPEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTBYPEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTBYPEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTBYPEN1` writer"]
pub struct W(crate::W<OUTBYPEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTBYPEN1_SPEC>;
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
impl From<crate::W<OUTBYPEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTBYPEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQ` reader - Output Data Input for DQ bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DQ` writer - Output Data Input for DQ bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
pub type DQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTBYPEN1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Output Data Input for DQ bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    pub fn dq(&self) -> DQ_R {
        DQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Data Input for DQ bus When a bit in register OUTBYPEN* is set to 1, the value of the corresponding bit in register OUTD* is asserted at the output."]
    #[inline(always)]
    #[must_use]
    pub fn dq(&mut self) -> DQ_W<0> {
        DQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Bypass Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outbypen1](index.html) module"]
pub struct OUTBYPEN1_SPEC;
impl crate::RegisterSpec for OUTBYPEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outbypen1::R](R) reader structure"]
impl crate::Readable for OUTBYPEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outbypen1::W](W) writer structure"]
impl crate::Writable for OUTBYPEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTBYPEN1 to value 0"]
impl crate::Resettable for OUTBYPEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
