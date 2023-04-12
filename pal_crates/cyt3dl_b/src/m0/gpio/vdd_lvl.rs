#[doc = "Register `VDD_LVL` reader"]
pub struct R(crate::R<VDD_LVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_LVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_LVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_LVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDDIO_LVL` reader - Indicates level status of VDDIO supplies (i.e. other than VDDD, VDDA) on the device (supplies are numbered 0..n-1). '0': 1.2V '1': 1.8V When multiple VDDIO supplies are present, they will be assigned in alphanumeric ascending order to these bits during implementation and follows the VDD_ACTIVE.VDDIO_ACTIVE ordering."]
pub type VDDIO_LVL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates level status of VDDIO supplies (i.e. other than VDDD, VDDA) on the device (supplies are numbered 0..n-1). '0': 1.2V '1': 1.8V When multiple VDDIO supplies are present, they will be assigned in alphanumeric ascending order to these bits during implementation and follows the VDD_ACTIVE.VDDIO_ACTIVE ordering."]
    #[inline(always)]
    pub fn vddio_lvl(&self) -> VDDIO_LVL_R {
        VDDIO_LVL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "External power supply level register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_lvl](index.html) module"]
pub struct VDD_LVL_SPEC;
impl crate::RegisterSpec for VDD_LVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_lvl::R](R) reader structure"]
impl crate::Readable for VDD_LVL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VDD_LVL to value 0"]
impl crate::Resettable for VDD_LVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
