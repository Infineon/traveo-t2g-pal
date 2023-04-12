#[doc = "Register `SPR2` reader"]
pub struct R(crate::R<SPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB2` reader - Holds the scheduled line of the frame that is beeing executed in TB2. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is beeing executed in TB2. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb2(&self) -> LINECOUNTERTB2_R {
        LINECOUNTERTB2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register (internal register with id = 18)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr2](index.html) module"]
pub struct SPR2_SPEC;
impl crate::RegisterSpec for SPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr2::R](R) reader structure"]
impl crate::Readable for SPR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR2 to value 0"]
impl crate::Resettable for SPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
