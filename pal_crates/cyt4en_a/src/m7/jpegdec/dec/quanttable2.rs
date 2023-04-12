#[doc = "Register `QUANTTABLE2[%s]` reader"]
pub struct R(crate::R<QUANTTABLE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUANTTABLE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUANTTABLE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUANTTABLE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QT2CONTENT0` reader - An entry in Quantization Table 2"]
pub type QT2CONTENT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT2CONTENT1` reader - An entry in Quantization Table 2"]
pub type QT2CONTENT1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - An entry in Quantization Table 2"]
    #[inline(always)]
    pub fn qt2content0(&self) -> QT2CONTENT0_R {
        QT2CONTENT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - An entry in Quantization Table 2"]
    #[inline(always)]
    pub fn qt2content1(&self) -> QT2CONTENT1_R {
        QT2CONTENT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Quantization Table number 2 downloaded from a JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quanttable2](index.html) module"]
pub struct QUANTTABLE2_SPEC;
impl crate::RegisterSpec for QUANTTABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quanttable2::R](R) reader structure"]
impl crate::Readable for QUANTTABLE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUANTTABLE2[%s]
to value 0"]
impl crate::Resettable for QUANTTABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
