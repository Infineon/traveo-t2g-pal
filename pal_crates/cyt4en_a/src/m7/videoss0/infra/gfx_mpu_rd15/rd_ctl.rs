#[doc = "Register `RD_CTL` reader"]
pub struct R(crate::R<RD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PC` reader - N/A"]
pub type PC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Memory protection context for AXI read requests (not used).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_ctl](index.html) module"]
pub struct RD_CTL_SPEC;
impl crate::RegisterSpec for RD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_ctl::R](R) reader structure"]
impl crate::Readable for RD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_CTL to value 0"]
impl crate::Resettable for RD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
