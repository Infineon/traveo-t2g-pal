#[doc = "Register `DESIGNCONFIGURATION` reader"]
pub struct R(crate::R<DESIGNCONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUFFERDEPTH` reader - The size of the buffer between the Scheduler and the Programmer."]
pub type BUFFERDEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFFERDEPTHSCHEDULER` reader - The buffer (axi decouple fifo) size inside of the scheduler."]
pub type BUFFERDEPTHSCHEDULER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFFERDEPTHPROGRAMMER` reader - The buffer (axi decouple fifo) size inside of the programmer."]
pub type BUFFERDEPTHPROGRAMMER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - The size of the buffer between the Scheduler and the Programmer."]
    #[inline(always)]
    pub fn bufferdepth(&self) -> BUFFERDEPTH_R {
        BUFFERDEPTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - The buffer (axi decouple fifo) size inside of the scheduler."]
    #[inline(always)]
    pub fn bufferdepthscheduler(&self) -> BUFFERDEPTHSCHEDULER_R {
        BUFFERDEPTHSCHEDULER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The buffer (axi decouple fifo) size inside of the programmer."]
    #[inline(always)]
    pub fn bufferdepthprogrammer(&self) -> BUFFERDEPTHPROGRAMMER_R {
        BUFFERDEPTHPROGRAMMER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Provides HW specific information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designconfiguration](index.html) module"]
pub struct DESIGNCONFIGURATION_SPEC;
impl crate::RegisterSpec for DESIGNCONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designconfiguration::R](R) reader structure"]
impl crate::Readable for DESIGNCONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCONFIGURATION to value 0"]
impl crate::Resettable for DESIGNCONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
