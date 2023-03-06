#[doc = "Register `SPR3` reader"]
pub struct R(crate::R<SPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB3` reader - Holds the scheduled line of the frame that is being executed in TB3. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is being executed in TB3. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb3(&self) -> LINECOUNTERTB3_R {
        LINECOUNTERTB3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register (internal register with id = 19)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr3](index.html) module"]
pub struct SPR3_SPEC;
impl crate::RegisterSpec for SPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr3::R](R) reader structure"]
impl crate::Readable for SPR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR3 to value 0"]
impl crate::Resettable for SPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
