#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_COMPLETE_MASKED` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type FRAME_COMPLETE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `JPEG_CORE_MASKED` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type JPEG_CORE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `FETCH_ERROR_MASKED` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type FETCH_ERROR_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `STORE_ERROR_MASKED` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type STORE_ERROR_MASKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn frame_complete_masked(&self) -> FRAME_COMPLETE_MASKED_R {
        FRAME_COMPLETE_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn jpeg_core_masked(&self) -> JPEG_CORE_MASKED_R {
        JPEG_CORE_MASKED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn fetch_error_masked(&self) -> FETCH_ERROR_MASKED_R {
        FETCH_ERROR_MASKED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn store_error_masked(&self) -> STORE_ERROR_MASKED_R {
        STORE_ERROR_MASKED_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt masked bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
