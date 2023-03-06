#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUMPIDLE` reader - If read as '1', the framedump unit is currently not dumping any frames. If read as '0' the framedump unit is currently dumping frames."]
pub type DUMPIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - If read as '1', the framedump unit is currently not dumping any frames. If read as '0' the framedump unit is currently dumping frames."]
    #[inline(always)]
    pub fn dumpidle(&self) -> DUMPIDLE_R {
        DUMPIDLE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
