#[doc = "Register `CLOCKCOUNTEROVERFLOW` reader"]
pub struct R(crate::R<CLOCKCOUNTEROVERFLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKCOUNTEROVERFLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKCOUNTEROVERFLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKCOUNTEROVERFLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLOCKCOUNTEROVERFLOW` reader - This bit is set if ClockCounter overflows."]
pub type CLOCKCOUNTEROVERFLOW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit is set if ClockCounter overflows."]
    #[inline(always)]
    pub fn clockcounteroverflow(&self) -> CLOCKCOUNTEROVERFLOW_R {
        CLOCKCOUNTEROVERFLOW_R::new((self.bits & 1) != 0)
    }
}
#[doc = "One of Clock Counter registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockcounteroverflow](index.html) module"]
pub struct CLOCKCOUNTEROVERFLOW_SPEC;
impl crate::RegisterSpec for CLOCKCOUNTEROVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockcounteroverflow::R](R) reader structure"]
impl crate::Readable for CLOCKCOUNTEROVERFLOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLOCKCOUNTEROVERFLOW to value 0"]
impl crate::Resettable for CLOCKCOUNTEROVERFLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
