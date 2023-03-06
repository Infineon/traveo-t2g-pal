#[doc = "Register `RSLTCOMP2BINCNT` reader"]
pub struct R(crate::R<RSLTCOMP2BINCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP2BINCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP2BINCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP2BINCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSLTCOMP2BINCNT` writer"]
pub struct W(crate::W<RSLTCOMP2BINCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSLTCOMP2BINCNT_SPEC>;
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
impl From<crate::W<RSLTCOMP2BINCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSLTCOMP2BINCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2_BINCNT` reader - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value."]
pub type COMP2_BINCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP2_BINCNT` writer - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value."]
pub type COMP2_BINCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RSLTCOMP2BINCNT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value."]
    #[inline(always)]
    pub fn comp2_bincnt(&self) -> COMP2_BINCNT_R {
        COMP2_BINCNT_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value."]
    #[inline(always)]
    #[must_use]
    pub fn comp2_bincnt(&mut self) -> COMP2_BINCNT_W<0> {
        COMP2_BINCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bin counters of component 2 histogram.This register do NOT support debugger access followed by software access (they clear on read)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp2bincnt](index.html) module"]
pub struct RSLTCOMP2BINCNT_SPEC;
impl crate::RegisterSpec for RSLTCOMP2BINCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp2bincnt::R](R) reader structure"]
impl crate::Readable for RSLTCOMP2BINCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsltcomp2bincnt::W](W) writer structure"]
impl crate::Writable for RSLTCOMP2BINCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSLTCOMP2BINCNT to value 0"]
impl crate::Resettable for RSLTCOMP2BINCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
