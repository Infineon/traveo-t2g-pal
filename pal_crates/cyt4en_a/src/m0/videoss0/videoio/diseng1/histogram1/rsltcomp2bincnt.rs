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
#[doc = "Field `COMP2_BINCNT` reader - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value. (format is unsigned integer)"]
pub type COMP2_BINCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Counter of current bin index in component 2 histogram. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cb value. (format is unsigned integer)"]
    #[inline(always)]
    pub fn comp2_bincnt(&self) -> COMP2_BINCNT_R {
        COMP2_BINCNT_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Bin counters of component 2 histogram. This register do NOT support debugger access followed by software access (they clear on read)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp2bincnt](index.html) module"]
pub struct RSLTCOMP2BINCNT_SPEC;
impl crate::RegisterSpec for RSLTCOMP2BINCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp2bincnt::R](R) reader structure"]
impl crate::Readable for RSLTCOMP2BINCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP2BINCNT to value 0"]
impl crate::Resettable for RSLTCOMP2BINCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
