#[doc = "Register `SPR5` reader"]
pub struct R(crate::R<SPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB5` reader - Holds the scheduled line of the frame that is being executed in TB5. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is being executed in TB5. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb5(&self) -> LINECOUNTERTB5_R {
        LINECOUNTERTB5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register (internal register with id = 21)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr5](index.html) module"]
pub struct SPR5_SPEC;
impl crate::RegisterSpec for SPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr5::R](R) reader structure"]
impl crate::Readable for SPR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR5 to value 0"]
impl crate::Resettable for SPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
