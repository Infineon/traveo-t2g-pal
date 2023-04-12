#[doc = "Register `HUFFMANTABLEDC1CODELENGTH[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEDC1CODELENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEDC1CODELENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEDC1CODELENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEDC1CODELENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC1NUMCODELENGTHN` reader - Number of codes with the code length of n (n=1~16) in DC Huffman Table 1."]
pub type DC1NUMCODELENGTHN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of codes with the code length of n (n=1~16) in DC Huffman Table 1."]
    #[inline(always)]
    pub fn dc1numcodelengthn(&self) -> DC1NUMCODELENGTHN_R {
        DC1NUMCODELENGTHN_R::new(self.bits)
    }
}
#[doc = "Shows the code length distribution for DC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantabledc1codelength](index.html) module"]
pub struct HUFFMANTABLEDC1CODELENGTH_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEDC1CODELENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantabledc1codelength::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEDC1CODELENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEDC1CODELENGTH[%s]
to value 0"]
impl crate::Resettable for HUFFMANTABLEDC1CODELENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
