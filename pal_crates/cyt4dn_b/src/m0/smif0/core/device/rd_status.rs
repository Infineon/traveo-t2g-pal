#[doc = "Register `RD_STATUS` reader"]
pub struct R(crate::R<RD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FS_STATUS` reader - Provides the Functional Safety Status Register of the memory received with the last read transfer."]
pub type FS_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Provides the Functional Safety Status Register of the memory received with the last read transfer."]
    #[inline(always)]
    pub fn fs_status(&self) -> FS_STATUS_R {
        FS_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Read status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_status](index.html) module"]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_status::R](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
