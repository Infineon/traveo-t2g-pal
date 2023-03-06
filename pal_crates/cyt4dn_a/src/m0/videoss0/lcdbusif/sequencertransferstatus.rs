#[doc = "Register `SEQUENCERTRANSFERSTATUS` reader"]
pub struct R(crate::R<SEQUENCERTRANSFERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQUENCERTRANSFERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQUENCERTRANSFERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQUENCERTRANSFERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRANSFERREMAINING` reader - Internal counter of the sequencer for current executed instruction NOP, IR_GET, DR_GET, WR_PIXEL or IRIS_FRAME."]
pub type TRANSFERREMAINING_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - Internal counter of the sequencer for current executed instruction NOP, IR_GET, DR_GET, WR_PIXEL or IRIS_FRAME."]
    #[inline(always)]
    pub fn transferremaining(&self) -> TRANSFERREMAINING_R {
        TRANSFERREMAINING_R::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Transfer remaining of current executed command.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sequencertransferstatus](index.html) module"]
pub struct SEQUENCERTRANSFERSTATUS_SPEC;
impl crate::RegisterSpec for SEQUENCERTRANSFERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sequencertransferstatus::R](R) reader structure"]
impl crate::Readable for SEQUENCERTRANSFERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEQUENCERTRANSFERSTATUS to value 0"]
impl crate::Resettable for SEQUENCERTRANSFERSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
