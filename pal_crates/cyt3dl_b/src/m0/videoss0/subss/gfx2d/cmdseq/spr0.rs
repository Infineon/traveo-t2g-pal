#[doc = "Register `SPR0` reader"]
pub struct R(crate::R<SPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB0` reader - Holds the scheduled line of the frame that is being executed in TB0. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is being executed in TB0. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb0(&self) -> LINECOUNTERTB0_R {
        LINECOUNTERTB0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register. (Internal register with id = 16)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr0](index.html) module"]
pub struct SPR0_SPEC;
impl crate::RegisterSpec for SPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr0::R](R) reader structure"]
impl crate::Readable for SPR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR0 to value 0"]
impl crate::Resettable for SPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
