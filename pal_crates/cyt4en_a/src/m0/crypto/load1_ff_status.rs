#[doc = "Register `LOAD1_FF_STATUS` reader"]
pub struct R(crate::R<LOAD1_FF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD1_FF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD1_FF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD1_FF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED5` reader - See LOAD1_FF_STATUS.USED."]
pub type USED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` reader - See LOAD1_FF_STATUS.BUSY."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - See LOAD1_FF_STATUS.USED."]
    #[inline(always)]
    pub fn used5(&self) -> USED5_R {
        USED5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - See LOAD1_FF_STATUS.BUSY."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Load 1 FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load1_ff_status](index.html) module"]
pub struct LOAD1_FF_STATUS_SPEC;
impl crate::RegisterSpec for LOAD1_FF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load1_ff_status::R](R) reader structure"]
impl crate::Readable for LOAD1_FF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOAD1_FF_STATUS to value 0"]
impl crate::Resettable for LOAD1_FF_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
