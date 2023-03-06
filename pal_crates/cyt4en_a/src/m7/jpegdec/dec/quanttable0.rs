#[doc = "Register `QUANTTABLE0[%s]` reader"]
pub struct R(crate::R<QUANTTABLE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUANTTABLE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUANTTABLE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUANTTABLE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QT0CONTENT0` reader - An entry in Quantization Table 0"]
pub type QT0CONTENT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT0CONTENT1` reader - An entry in Quantization Table 0"]
pub type QT0CONTENT1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - An entry in Quantization Table 0"]
    #[inline(always)]
    pub fn qt0content0(&self) -> QT0CONTENT0_R {
        QT0CONTENT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - An entry in Quantization Table 0"]
    #[inline(always)]
    pub fn qt0content1(&self) -> QT0CONTENT1_R {
        QT0CONTENT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Quantization Table number 0 downloaded from a JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quanttable0](index.html) module"]
pub struct QUANTTABLE0_SPEC;
impl crate::RegisterSpec for QUANTTABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quanttable0::R](R) reader structure"]
impl crate::Readable for QUANTTABLE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUANTTABLE0[%s]
to value 0"]
impl crate::Resettable for QUANTTABLE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
