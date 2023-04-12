#[doc = "Register `HUFFMANTABLEAC1GROUPNO[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEAC1GROUPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEAC1GROUPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEAC1GROUPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEAC1GROUPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AC1GROUPNO` reader - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 1. Note: For the last word (offset 0x4E4), only upper half-word is used. Its default value is 0xFFFF0000."]
pub type AC1GROUPNO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 1. Note: For the last word (offset 0x4E4), only upper half-word is used. Its default value is 0xFFFF0000."]
    #[inline(always)]
    pub fn ac1groupno(&self) -> AC1GROUPNO_R {
        AC1GROUPNO_R::new(self.bits)
    }
}
#[doc = "Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantableac1groupno](index.html) module"]
pub struct HUFFMANTABLEAC1GROUPNO_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEAC1GROUPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantableac1groupno::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEAC1GROUPNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEAC1GROUPNO[%s]
to value 0xffff_ffff"]
impl crate::Resettable for HUFFMANTABLEAC1GROUPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
