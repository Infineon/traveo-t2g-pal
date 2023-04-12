#[doc = "Register `CREL` reader"]
pub struct R(crate::R<CREL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CREL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CREL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CREL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAY` reader - Time Stamp Day Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` reader - Time Stamp Month Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` reader - Time Stamp Year One digit, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release One digit, BCD-coded."]
pub type SUBSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEP` reader - Step of Core Release One digit, BCD-coded."]
pub type STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` reader - Core Release One digit, BCD-coded."]
pub type REL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month Two digits, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year One digit, BCD-coded. This field is set by generic parameter on M_TTCAN synthesis."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](index.html) module"]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crel::R](R) reader structure"]
impl crate::Readable for CREL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CREL to value 0x3238_0609"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3238_0609;
}
