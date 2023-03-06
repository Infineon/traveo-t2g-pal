#[doc = "Register `RSLTCOMP0BINIDX` reader"]
pub struct R(crate::R<RSLTCOMP0BINIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP0BINIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP0BINIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP0BINIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP0_BINIDX_MAXCNT` reader - The index of the bin of the component 0 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 0 means the red value, in the case of YUV pixels, component0 means the Y value. If luma/luminance calculation is switched on by field lum_mode component0 is the calculated luma/luminance."]
pub type COMP0_BINIDX_MAXCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP0_BINIDX_NXT` reader - Do not use this filed. The index of the bin, that will be next accessed by register RsltComp0Bincnt. In the case of RGB pixels, component0 means the red value, in the case of YUV pixels, component 0 means the Y value. If luma/luminance calculation is switched on by field lum_mode component0 is the calculated luma/luminance."]
pub type COMP0_BINIDX_NXT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - The index of the bin of the component 0 histogram, which contains the highest counter value, which were read until now for the ongoing readout. In the case of RGB pixels, component 0 means the red value, in the case of YUV pixels, component0 means the Y value. If luma/luminance calculation is switched on by field lum_mode component0 is the calculated luma/luminance."]
    #[inline(always)]
    pub fn comp0_binidx_maxcnt(&self) -> COMP0_BINIDX_MAXCNT_R {
        COMP0_BINIDX_MAXCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Do not use this filed. The index of the bin, that will be next accessed by register RsltComp0Bincnt. In the case of RGB pixels, component0 means the red value, in the case of YUV pixels, component 0 means the Y value. If luma/luminance calculation is switched on by field lum_mode component0 is the calculated luma/luminance."]
    #[inline(always)]
    pub fn comp0_binidx_nxt(&self) -> COMP0_BINIDX_NXT_R {
        COMP0_BINIDX_NXT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Results of frame measurement for component 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp0binidx](index.html) module"]
pub struct RSLTCOMP0BINIDX_SPEC;
impl crate::RegisterSpec for RSLTCOMP0BINIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp0binidx::R](R) reader structure"]
impl crate::Readable for RSLTCOMP0BINIDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP0BINIDX to value 0"]
impl crate::Resettable for RSLTCOMP0BINIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
