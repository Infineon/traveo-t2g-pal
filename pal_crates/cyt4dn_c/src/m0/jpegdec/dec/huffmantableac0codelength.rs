#[doc = "Register `HUFFMANTABLEAC0CODELENGTH[%s]` reader"]
pub struct R(crate::R<HUFFMANTABLEAC0CODELENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLEAC0CODELENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLEAC0CODELENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLEAC0CODELENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AC0NUMCODELENGTHN` reader - Number of codes with the code length of n (n=1~16) in AC Huffman Table 0."]
pub type AC0NUMCODELENGTHN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of codes with the code length of n (n=1~16) in AC Huffman Table 0."]
    #[inline(always)]
    pub fn ac0numcodelengthn(&self) -> AC0NUMCODELENGTHN_R {
        AC0NUMCODELENGTHN_R::new(self.bits)
    }
}
#[doc = "Shows the code length distribution for AC Huffman Table 0. For each Huffman code lengths (0 to 16), the number of used codes is shown. e.g. Field for code length 3 has the value '5' when 010/011/100/101/110 are used.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantableac0codelength](index.html) module"]
pub struct HUFFMANTABLEAC0CODELENGTH_SPEC;
impl crate::RegisterSpec for HUFFMANTABLEAC0CODELENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantableac0codelength::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLEAC0CODELENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLEAC0CODELENGTH[%s]
to value 0"]
impl crate::Resettable for HUFFMANTABLEAC0CODELENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
