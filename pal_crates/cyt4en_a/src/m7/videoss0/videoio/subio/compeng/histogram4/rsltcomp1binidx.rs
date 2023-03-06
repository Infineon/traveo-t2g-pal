#[doc = "Register `RSLTCOMP1BINIDX` reader"]
pub struct R(crate::R<RSLTCOMP1BINIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP1BINIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP1BINIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP1BINIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP1_BINIDX_MAXCNT` reader - The index of the bin of the component 1 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. (format is unsigned integer)"]
pub type COMP1_BINIDX_MAXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_BINIDX_NXT` reader - The index of the bin, that will be next accessed by register RsltComp1Bincnt. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. (format is unsigned integer)"]
pub type COMP1_BINIDX_NXT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - The index of the bin of the component 1 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. (format is unsigned integer)"]
    #[inline(always)]
    pub fn comp1_binidx_maxcnt(&self) -> COMP1_BINIDX_MAXCNT_R {
        COMP1_BINIDX_MAXCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The index of the bin, that will be next accessed by register RsltComp1Bincnt. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. (format is unsigned integer)"]
    #[inline(always)]
    pub fn comp1_binidx_nxt(&self) -> COMP1_BINIDX_NXT_R {
        COMP1_BINIDX_NXT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Results of frame measurement for component 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp1binidx](index.html) module"]
pub struct RSLTCOMP1BINIDX_SPEC;
impl crate::RegisterSpec for RSLTCOMP1BINIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp1binidx::R](R) reader structure"]
impl crate::Readable for RSLTCOMP1BINIDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP1BINIDX to value 0"]
impl crate::Resettable for RSLTCOMP1BINIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
