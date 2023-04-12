#[doc = "Register `CM7_0_INT_STATUS[%s]` reader"]
pub struct R(crate::R<CM7_0_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_0_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_0_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_0_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM7_0 activated system interrupt index for CPU interrupt. See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - Lowest CM7_0 activated system interrupt index for CPU interrupt. See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CM7 0 interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_0_int_status](index.html) module"]
pub struct CM7_0_INT_STATUS_SPEC;
impl crate::RegisterSpec for CM7_0_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_0_int_status::R](R) reader structure"]
impl crate::Readable for CM7_0_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM7_0_INT_STATUS[%s]
to value 0"]
impl crate::Resettable for CM7_0_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
