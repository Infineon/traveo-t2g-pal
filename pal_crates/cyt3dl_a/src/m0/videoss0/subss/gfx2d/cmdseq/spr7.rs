#[doc = "Register `SPR7` reader"]
pub struct R(crate::R<SPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB7` reader - Holds the scheduled line of the frame that is being executed in TB7. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB7_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is being executed in TB7. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb7(&self) -> LINECOUNTERTB7_R {
        LINECOUNTERTB7_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register (internal register with id = 23)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr7](index.html) module"]
pub struct SPR7_SPEC;
impl crate::RegisterSpec for SPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr7::R](R) reader structure"]
impl crate::Readable for SPR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR7 to value 0"]
impl crate::Resettable for SPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
