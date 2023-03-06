#[doc = "Register `ROT_BLK_MAX` reader"]
pub struct R(crate::R<ROT_BLK_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROT_BLK_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROT_BLK_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROT_BLK_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Maximum value of block-based index register. The number and size blocks in an MPC is design time configurable and for embedded memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
pub type VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum value of block-based index register. The number and size blocks in an MPC is design time configurable and for embedded memories defaults to covering the entire memory using 4kB blocks; see product datasheet for details on protection of external memories."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "Max value of block-based index register for ROT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rot_blk_max](index.html) module"]
pub struct ROT_BLK_MAX_SPEC;
impl crate::RegisterSpec for ROT_BLK_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rot_blk_max::R](R) reader structure"]
impl crate::Readable for ROT_BLK_MAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROT_BLK_MAX to value 0"]
impl crate::Resettable for ROT_BLK_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
