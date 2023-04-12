#[doc = "Register `HUFFMANTABLEDC0CODELENGTH[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEDC0CODELENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEDC0CODELENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEDC0CODELENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEDC0CODELENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC0NUMCODELENGTHN` reader - Number of codes with the code length of n (n=1~16) in DC Huffman Table 0."]
pub type DC0NUMCODELENGTHN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of codes with the code length of n (n=1~16) in DC Huffman Table 0."]
    #[inline(always)]
    pub fn dc0numcodelengthn(&self) -> DC0NUMCODELENGTHN_R {
        DC0NUMCODELENGTHN_R::new(self.bits)
    }
}
#[doc = "Shows the code length distribution for DC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantabledc0codelength](index.html) module"]
pub struct HUFFMANTABLEDC0CODELENGTH_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEDC0CODELENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantabledc0codelength::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEDC0CODELENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEDC0CODELENGTH[%s]
to value 0"]
impl crate::Resettable for HUFFMANTABLEDC0CODELENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
