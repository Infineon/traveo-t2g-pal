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
#[doc = "Field `DAY` reader - Design Time Stamp, Day Two digits, BCD-coded."]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` reader - Design Time Stamp, Month Two digits, BCD-coded."]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` reader - Design Time Stamp, Year One digit, BCD-coded."]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEP` reader - Step of Core Release Two digits, BCD-coded."]
pub type STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` reader - Core Release One digit, BCD-coded. Table 5: Coding for releases"]
pub type REL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Design Time Stamp, Day Two digits, BCD-coded."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Design Time Stamp, Month Two digits, BCD-coded."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Design Time Stamp, Year One digit, BCD-coded."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Step of Core Release Two digits, BCD-coded."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Core Release One digit, BCD-coded. Table 5: Coding for releases"]
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
#[doc = "`reset()` method sets CREL to value 0x1039_0206"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1039_0206;
}
