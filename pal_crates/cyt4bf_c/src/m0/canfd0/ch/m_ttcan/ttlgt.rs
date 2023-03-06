#[doc = "Register `TTLGT` reader"]
pub struct R(crate::R<TTLGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTLGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTLGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTLGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LT` reader - Local Time Non-fractional part of local time, incremented once each local NTU (see Section 4.5). 0x0000-FFFF Local time value of TTCAN node"]
pub type LT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GT` reader - Global Time Non-fractional part of the sum of the node's local time and its local offset (see Section 4.5). 0x0000-FFFF Global time value of TTCAN network"]
pub type GT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Local Time Non-fractional part of local time, incremented once each local NTU (see Section 4.5). 0x0000-FFFF Local time value of TTCAN node"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Time Non-fractional part of the sum of the node's local time and its local offset (see Section 4.5). 0x0000-FFFF Global time value of TTCAN network"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TT Local &amp; Global Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttlgt](index.html) module"]
pub struct TTLGT_SPEC;
impl crate::RegisterSpec for TTLGT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttlgt::R](R) reader structure"]
impl crate::Readable for TTLGT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTLGT to value 0"]
impl crate::Resettable for TTLGT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
