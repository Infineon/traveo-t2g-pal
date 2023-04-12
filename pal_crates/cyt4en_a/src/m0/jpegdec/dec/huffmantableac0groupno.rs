#[doc = "Register `HUFFMANTABLEAC0GROUPNO[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEAC0GROUPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEAC0GROUPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEAC0GROUPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEAC0GROUPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AC0GROUPNO` reader - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 0. Note: For the last word (offset 0x3D0), only upper half-word is used. Its default value is 0xFFFF0000."]
pub type AC0GROUPNO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 0. Note: For the last word (offset 0x3D0), only upper half-word is used. Its default value is 0xFFFF0000."]
    #[inline(always)]
    pub fn ac0groupno(&self) -> AC0GROUPNO_R {
        AC0GROUPNO_R::new(self.bits)
    }
}
#[doc = "Shows the zero run-length+group number values in the order of descending appearance frequencies for AC Huffman Table 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantableac0groupno](index.html) module"]
pub struct HUFFMANTABLEAC0GROUPNO_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEAC0GROUPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantableac0groupno::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEAC0GROUPNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEAC0GROUPNO[%s]
to value 0xffff_ffff"]
impl crate::Resettable for HUFFMANTABLEAC0GROUPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
