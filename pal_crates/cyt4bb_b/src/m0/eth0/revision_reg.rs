#[doc = "Register `REVISION_REG` reader"]
pub struct R(crate::R<REVISION_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODULE_REVISION` reader - Module revision - fixed value specific to the revision of the design which is incremented for each non-fix release of the IP."]
pub type MODULE_REVISION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MODULE_IDENTIFICATION_NUMBER` reader - Module identification number - for the GEM, this value is fixed."]
pub type MODULE_IDENTIFICATION_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIX_NUMBER` reader - Fix number - incremented for fix releases."]
pub type FIX_NUMBER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Module revision - fixed value specific to the revision of the design which is incremented for each non-fix release of the IP."]
    #[inline(always)]
    pub fn module_revision(&self) -> MODULE_REVISION_R {
        MODULE_REVISION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Module identification number - for the GEM, this value is fixed."]
    #[inline(always)]
    pub fn module_identification_number(&self) -> MODULE_IDENTIFICATION_NUMBER_R {
        MODULE_IDENTIFICATION_NUMBER_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - Fix number - incremented for fix releases."]
    #[inline(always)]
    pub fn fix_number(&self) -> FIX_NUMBER_R {
        FIX_NUMBER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "This register indicates a Cadence module identification number and module revision. The value of this register is read only as defined by `gem_revision_reg_value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision_reg](index.html) module"]
pub struct REVISION_REG_SPEC;
impl crate::RegisterSpec for REVISION_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revision_reg::R](R) reader structure"]
impl crate::Readable for REVISION_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REVISION_REG to value 0x0007_0109"]
impl crate::Resettable for REVISION_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0109;
}
