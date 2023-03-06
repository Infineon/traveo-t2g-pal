#[doc = "Register `QUANTTABLE3[%s]` reader"]
pub struct R(crate::R<QUANTTABLE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUANTTABLE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUANTTABLE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUANTTABLE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QT3CONTENT0` reader - An entry in Quantization Table 2"]
pub type QT3CONTENT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT3CONTENT1` reader - An entry in Quantization Table 2"]
pub type QT3CONTENT1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - An entry in Quantization Table 2"]
    #[inline(always)]
    pub fn qt3content0(&self) -> QT3CONTENT0_R {
        QT3CONTENT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - An entry in Quantization Table 2"]
    #[inline(always)]
    pub fn qt3content1(&self) -> QT3CONTENT1_R {
        QT3CONTENT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Quantization Table number 3 downloaded from a JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quanttable3](index.html) module"]
pub struct QUANTTABLE3_SPEC;
impl crate::RegisterSpec for QUANTTABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quanttable3::R](R) reader structure"]
impl crate::Readable for QUANTTABLE3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUANTTABLE3[%s]
to value 0"]
impl crate::Resettable for QUANTTABLE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
