#[doc = "Register `RESP` reader"]
pub struct R(crate::R<RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP_WRLEV` reader - Write Leveling Response Voltage levels can be swept on the PAD_DQ and the receiver response is observable in this register"]
pub type RESP_WRLEV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Leveling Response Voltage levels can be swept on the PAD_DQ and the receiver response is observable in this register"]
    #[inline(always)]
    pub fn resp_wrlev(&self) -> RESP_WRLEV_R {
        RESP_WRLEV_R::new(self.bits)
    }
}
#[doc = "PHY Response Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](index.html) module"]
pub struct RESP_SPEC;
impl crate::RegisterSpec for RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp::R](R) reader structure"]
impl crate::Readable for RESP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP to value 0"]
impl crate::Resettable for RESP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
