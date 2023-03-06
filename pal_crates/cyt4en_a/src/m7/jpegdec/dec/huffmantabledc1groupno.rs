#[doc = "Register `HUFFMANTABLEDC1GROUPNO[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEDC1GROUPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEDC1GROUPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEDC1GROUPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEDC1GROUPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC1GROUPNO` reader - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 1."]
pub type DC1GROUPNO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 1."]
    #[inline(always)]
    pub fn dc1groupno(&self) -> DC1GROUPNO_R {
        DC1GROUPNO_R::new(self.bits)
    }
}
#[doc = "Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantabledc1groupno](index.html) module"]
pub struct HUFFMANTABLEDC1GROUPNO_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEDC1GROUPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantabledc1groupno::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEDC1GROUPNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEDC1GROUPNO[%s]
to value 0x0f0f_0f0f"]
impl crate::Resettable for HUFFMANTABLEDC1GROUPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
