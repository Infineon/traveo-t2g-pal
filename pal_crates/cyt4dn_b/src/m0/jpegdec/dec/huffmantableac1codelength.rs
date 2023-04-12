#[doc = "Register `HUFFMANTABLEAC1CODELENGTH[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEAC1CODELENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEAC1CODELENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEAC1CODELENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEAC1CODELENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AC1NUMCODELENGTHN` reader - Number of codes with the code length of n (n=1~16) in AC Huffman Table 1."]
pub type AC1NUMCODELENGTHN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of codes with the code length of n (n=1~16) in AC Huffman Table 1."]
    #[inline(always)]
    pub fn ac1numcodelengthn(&self) -> AC1NUMCODELENGTHN_R {
        AC1NUMCODELENGTHN_R::new(self.bits)
    }
}
#[doc = "Shows the code length distribution for AC Huffman Table 1. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantableac1codelength](index.html) module"]
pub struct HUFFMANTABLEAC1CODELENGTH_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEAC1CODELENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantableac1codelength::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEAC1CODELENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEAC1CODELENGTH[%s]
to value 0"]
impl crate::Resettable for HUFFMANTABLEAC1CODELENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
