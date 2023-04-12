#[doc = "Register `NMV2` reader"]
pub struct R(crate::R<NMV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA4` reader - Data4"]
pub type DATA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA5` reader - Data5"]
pub type DATA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` reader - Data6"]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA7` reader - Data7"]
pub type DATA7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Data4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Network Management Vector 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmv2](index.html) module"]
pub struct NMV2_SPEC;
impl crate::RegisterSpec for NMV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmv2::R](R) reader structure"]
impl crate::Readable for NMV2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NMV2 to value 0"]
impl crate::Resettable for NMV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
