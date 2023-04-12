#[doc = "Register `HUFFMANTABLEDC0GROUPNO[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEDC0GROUPNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEDC0GROUPNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEDC0GROUPNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEDC0GROUPNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC0GROUPNO` reader - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 0."]
pub type DC0GROUPNO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 0."]
    #[inline(always)]
    pub fn dc0groupno(&self) -> DC0GROUPNO_R {
        DC0GROUPNO_R::new(self.bits)
    }
}
#[doc = "Shows the group numbers in the order of descending appearance frequencies for DC Huffman Table 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantabledc0groupno](index.html) module"]
pub struct HUFFMANTABLEDC0GROUPNO_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEDC0GROUPNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantabledc0groupno::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEDC0GROUPNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEDC0GROUPNO[%s]
to value 0x0f0f_0f0f"]
impl crate::Resettable for HUFFMANTABLEDC0GROUPNO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
