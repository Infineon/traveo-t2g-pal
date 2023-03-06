#[doc = "Register `QUANTTABLENO` reader"]
pub struct R(crate::R<QUANTTABLENO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUANTTABLENO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUANTTABLENO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUANTTABLENO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QT1COLOR` reader - Quantization table number of 1st color component"]
pub type QT1COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT2COLOR` reader - Quantization table number of 2nd color component"]
pub type QT2COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT3COLOR` reader - Quantization table number of 3rd color component"]
pub type QT3COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT4COLOR` reader - Quantization table number of 4th color component"]
pub type QT4COLOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Quantization table number of 1st color component"]
    #[inline(always)]
    pub fn qt1color(&self) -> QT1COLOR_R {
        QT1COLOR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Quantization table number of 2nd color component"]
    #[inline(always)]
    pub fn qt2color(&self) -> QT2COLOR_R {
        QT2COLOR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Quantization table number of 3rd color component"]
    #[inline(always)]
    pub fn qt3color(&self) -> QT3COLOR_R {
        QT3COLOR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Quantization table number of 4th color component"]
    #[inline(always)]
    pub fn qt4color(&self) -> QT4COLOR_R {
        QT4COLOR_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Quantization table number downloaded from the JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quanttableno](index.html) module"]
pub struct QUANTTABLENO_SPEC;
impl crate::RegisterSpec for QUANTTABLENO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quanttableno::R](R) reader structure"]
impl crate::Readable for QUANTTABLENO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUANTTABLENO to value 0"]
impl crate::Resettable for QUANTTABLENO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
