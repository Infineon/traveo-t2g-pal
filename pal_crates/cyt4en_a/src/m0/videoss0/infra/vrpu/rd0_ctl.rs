#[doc = "Register `RD0_CTL` reader"]
pub struct R(crate::R<RD0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIO` reader - N/A"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "VRAM protection for read requests with ID=0 (not used).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd0_ctl](index.html) module"]
pub struct RD0_CTL_SPEC;
impl crate::RegisterSpec for RD0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd0_ctl::R](R) reader structure"]
impl crate::Readable for RD0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD0_CTL to value 0"]
impl crate::Resettable for RD0_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
