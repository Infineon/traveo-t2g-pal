#[doc = "Register `RSLTRDY` reader"]
pub struct R(crate::R<RSLTRDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTRDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTRDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTRDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSLT_RDY` reader - A one indicates that measurement results (in registers Rslt...) are valid and can be read. (format is unsigned integer)"]
pub type RSLT_RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - A one indicates that measurement results (in registers Rslt...) are valid and can be read. (format is unsigned integer)"]
    #[inline(always)]
    pub fn rslt_rdy(&self) -> RSLT_RDY_R {
        RSLT_RDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Measurement result status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltrdy](index.html) module"]
pub struct RSLTRDY_SPEC;
impl crate::RegisterSpec for RSLTRDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltrdy::R](R) reader structure"]
impl crate::Readable for RSLTRDY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTRDY to value 0"]
impl crate::Resettable for RSLTRDY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
