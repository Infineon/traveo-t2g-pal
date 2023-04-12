#[doc = "Register `LBHGLOBALSTATUS0` reader"]
pub struct R(crate::R<LBHGLOBALSTATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHGLOBALSTATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHGLOBALSTATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHGLOBALSTATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHCYCLECOUNTER` reader - This counter increments with every clock cycle."]
pub type LBHCYCLECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This counter increments with every clock cycle."]
    #[inline(always)]
    pub fn lbhcyclecounter(&self) -> LBHCYCLECOUNTER_R {
        LBHCYCLECOUNTER_R::new(self.bits)
    }
}
#[doc = "Global status register 0 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhglobalstatus0](index.html) module"]
pub struct LBHGLOBALSTATUS0_SPEC;
impl crate::RegisterSpec for LBHGLOBALSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhglobalstatus0::R](R) reader structure"]
impl crate::Readable for LBHGLOBALSTATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHGLOBALSTATUS0 to value 0"]
impl crate::Resettable for LBHGLOBALSTATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
