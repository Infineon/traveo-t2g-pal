#[doc = "Register `VU_STATUS` reader"]
pub struct R(crate::R<VU_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VU_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VU_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VU_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARRY` reader - STATUS CARRY field."]
pub type CARRY_R = crate::BitReader<bool>;
#[doc = "Field `EVEN` reader - STATUS EVEN field."]
pub type EVEN_R = crate::BitReader<bool>;
#[doc = "Field `ZERO` reader - STATUS ZERO field."]
pub type ZERO_R = crate::BitReader<bool>;
#[doc = "Field `ONE` reader - STATUS ONE field."]
pub type ONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - STATUS CARRY field."]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STATUS EVEN field."]
    #[inline(always)]
    pub fn even(&self) -> EVEN_R {
        EVEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STATUS ZERO field."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STATUS ONE field."]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Vector unit status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vu_status](index.html) module"]
pub struct VU_STATUS_SPEC;
impl crate::RegisterSpec for VU_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vu_status::R](R) reader structure"]
impl crate::Readable for VU_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VU_STATUS to value 0"]
impl crate::Resettable for VU_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
