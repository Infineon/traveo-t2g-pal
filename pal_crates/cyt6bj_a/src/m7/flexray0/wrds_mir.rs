#[doc = "Register `WRDS_MIR[%s]` reader"]
pub struct R(crate::R<WRDS_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRDS_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRDS_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRDS_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRDS_MIR[%s]` writer"]
pub struct W(crate::W<WRDS_MIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRDS_MIR_SPEC>;
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
impl From<crate::W<WRDS_MIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRDS_MIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
pub type MD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MD` writer - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRDS_MIR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Data Section \\[1...64\\]
(mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrds_mir](index.html) module"]
pub struct WRDS_MIR_SPEC;
impl crate::RegisterSpec for WRDS_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrds_mir::R](R) reader structure"]
impl crate::Readable for WRDS_MIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrds_mir::W](W) writer structure"]
impl crate::Writable for WRDS_MIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRDS_MIR[%s]
to value 0"]
impl crate::Resettable for WRDS_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
