#[doc = "Register `QUANTTABLE1[%s]` reader"]
pub struct R(crate::R<QUANTTABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUANTTABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUANTTABLE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUANTTABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QT1CONTENT0` reader - An entry in Quantization Table 1"]
pub type QT1CONTENT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QT1CONTENT1` reader - An entry in Quantization Table 1"]
pub type QT1CONTENT1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - An entry in Quantization Table 1"]
    #[inline(always)]
    pub fn qt1content0(&self) -> QT1CONTENT0_R {
        QT1CONTENT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - An entry in Quantization Table 1"]
    #[inline(always)]
    pub fn qt1content1(&self) -> QT1CONTENT1_R {
        QT1CONTENT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Quantization Table number 1 downloaded from a JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quanttable1](index.html) module"]
pub struct QUANTTABLE1_SPEC;
impl crate::RegisterSpec for QUANTTABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quanttable1::R](R) reader structure"]
impl crate::Readable for QUANTTABLE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUANTTABLE1[%s]
to value 0"]
impl crate::Resettable for QUANTTABLE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
