#[doc = "Register `RDDS[%s]` reader"]
pub struct R(crate::R<RDDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MD` reader - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
pub type MD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data MD\\[7:0\\]
= DW2n-1, byte4n-4 MD\\[15:8\\]
= DW2n-1, byte4n-3 MD\\[23:16\\]
= DW2n, byte4n-2 MD\\[31:24\\]
= DW2n, byte4n-1"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(self.bits)
    }
}
#[doc = "Read Data Section \\[1...64\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdds](index.html) module"]
pub struct RDDS_SPEC;
impl crate::RegisterSpec for RDDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdds::R](R) reader structure"]
impl crate::Readable for RDDS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDDS[%s]
to value 0"]
impl crate::Resettable for RDDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
