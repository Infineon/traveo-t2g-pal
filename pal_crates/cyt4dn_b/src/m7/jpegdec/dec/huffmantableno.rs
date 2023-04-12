#[doc = "Register `HUFFMANTABLENO` reader"]
pub struct R(crate::R<HUFFMANTABLENO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMANTABLENO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMANTABLENO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMANTABLENO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HTDC1COLOR` reader - DC Huffman table number of 1st color component"]
pub type HTDC1COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTAC1COLOR` reader - AC Huffman table number of 1st color component"]
pub type HTAC1COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTDC2COLOR` reader - DC Huffman table number of 2nd color component"]
pub type HTDC2COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTAC2COLOR` reader - AC Huffman table number of 2nd color component"]
pub type HTAC2COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTDC3COLOR` reader - DC Huffman table number of 3rd color component"]
pub type HTDC3COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTAC3COLOR` reader - AC Huffman table number of 3rd color component"]
pub type HTAC3COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTDC4COLOR` reader - DC Huffman table number of 4th color component"]
pub type HTDC4COLOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTAC4COLOR` reader - AC Huffman table number of 4th color component"]
pub type HTAC4COLOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - DC Huffman table number of 1st color component"]
    #[inline(always)]
    pub fn htdc1color(&self) -> HTDC1COLOR_R {
        HTDC1COLOR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AC Huffman table number of 1st color component"]
    #[inline(always)]
    pub fn htac1color(&self) -> HTAC1COLOR_R {
        HTAC1COLOR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DC Huffman table number of 2nd color component"]
    #[inline(always)]
    pub fn htdc2color(&self) -> HTDC2COLOR_R {
        HTDC2COLOR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - AC Huffman table number of 2nd color component"]
    #[inline(always)]
    pub fn htac2color(&self) -> HTAC2COLOR_R {
        HTAC2COLOR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DC Huffman table number of 3rd color component"]
    #[inline(always)]
    pub fn htdc3color(&self) -> HTDC3COLOR_R {
        HTDC3COLOR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - AC Huffman table number of 3rd color component"]
    #[inline(always)]
    pub fn htac3color(&self) -> HTAC3COLOR_R {
        HTAC3COLOR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DC Huffman table number of 4th color component"]
    #[inline(always)]
    pub fn htdc4color(&self) -> HTDC4COLOR_R {
        HTDC4COLOR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - AC Huffman table number of 4th color component"]
    #[inline(always)]
    pub fn htac4color(&self) -> HTAC4COLOR_R {
        HTAC4COLOR_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Huffman Table number downloaded from the JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmantableno](index.html) module"]
pub struct HUFFMANTABLENO_SPEC;
impl crate::RegisterSpec for HUFFMANTABLENO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmantableno::R](R) reader structure"]
impl crate::Readable for HUFFMANTABLENO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HUFFMANTABLENO to value 0"]
impl crate::Resettable for HUFFMANTABLENO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
