#[doc = "Register `NMV3` reader"]
pub struct R(crate::R<NMV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA8` reader - Data8"]
pub type DATA8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA9` reader - Data9"]
pub type DATA9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA10` reader - Data10"]
pub type DATA10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA11` reader - Data11"]
pub type DATA11_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Data8"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data9"]
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data10"]
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data11"]
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Network Management Vector 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmv3](index.html) module"]
pub struct NMV3_SPEC;
impl crate::RegisterSpec for NMV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmv3::R](R) reader structure"]
impl crate::Readable for NMV3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NMV3 to value 0"]
impl crate::Resettable for NMV3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
