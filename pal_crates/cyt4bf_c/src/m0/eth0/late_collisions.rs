#[doc = "Register `LATE_COLLISIONS` reader"]
pub struct R(crate::R<LATE_COLLISIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LATE_COLLISIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LATE_COLLISIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LATE_COLLISIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT17` reader - Late collisions - a 10 bit register counting the number of late collision occurring after the slot time (512 bits) has expired. In 10/100 mode, late collisions are counted twice i.e. both as a collision and a late collision. In gigabit mode, a late collision causes the transmission to be aborted, thus the single and multi collision registers are not updated."]
pub type COUNT17_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Late collisions - a 10 bit register counting the number of late collision occurring after the slot time (512 bits) has expired. In 10/100 mode, late collisions are counted twice i.e. both as a collision and a late collision. In gigabit mode, a late collision causes the transmission to be aborted, thus the single and multi collision registers are not updated."]
    #[inline(always)]
    pub fn count17(&self) -> COUNT17_R {
        COUNT17_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Late Collisions. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [late_collisions](index.html) module"]
pub struct LATE_COLLISIONS_SPEC;
impl crate::RegisterSpec for LATE_COLLISIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [late_collisions::R](R) reader structure"]
impl crate::Readable for LATE_COLLISIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LATE_COLLISIONS to value 0"]
impl crate::Resettable for LATE_COLLISIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
