#[doc = "Register `RSLTCOMP2BINIDX` reader"]
pub struct R(crate::R<RSLTCOMP2BINIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP2BINIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP2BINIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP2BINIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP2_BINIDX_MAXCNT` reader - The index of the bin of the component 2 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value."]
pub type COMP2_BINIDX_MAXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2_BINIDX_NXT` reader - The index of the bin, that will be next accessed by register RsltComp2Bincnt. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value."]
pub type COMP2_BINIDX_NXT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - The index of the bin of the component 2 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value."]
    #[inline(always)]
    pub fn comp2_binidx_maxcnt(&self) -> COMP2_BINIDX_MAXCNT_R {
        COMP2_BINIDX_MAXCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The index of the bin, that will be next accessed by register RsltComp2Bincnt. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value."]
    #[inline(always)]
    pub fn comp2_binidx_nxt(&self) -> COMP2_BINIDX_NXT_R {
        COMP2_BINIDX_NXT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Results of frame measurement for component 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp2binidx](index.html) module"]
pub struct RSLTCOMP2BINIDX_SPEC;
impl crate::RegisterSpec for RSLTCOMP2BINIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp2binidx::R](R) reader structure"]
impl crate::Readable for RSLTCOMP2BINIDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP2BINIDX to value 0"]
impl crate::Resettable for RSLTCOMP2BINIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
