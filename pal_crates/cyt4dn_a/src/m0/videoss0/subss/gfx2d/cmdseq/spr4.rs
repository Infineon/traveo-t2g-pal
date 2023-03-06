#[doc = "Register `SPR4` reader"]
pub struct R(crate::R<SPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINECOUNTERTB4` reader - Holds the scheduled line of the frame that is beeing executed in TB4. The value of this register is routed outside CmdSeq."]
pub type LINECOUNTERTB4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Holds the scheduled line of the frame that is beeing executed in TB4. The value of this register is routed outside CmdSeq."]
    #[inline(always)]
    pub fn linecountertb4(&self) -> LINECOUNTERTB4_R {
        LINECOUNTERTB4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Special Purpose Register (internal register with id = 20)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spr4](index.html) module"]
pub struct SPR4_SPEC;
impl crate::RegisterSpec for SPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spr4::R](R) reader structure"]
impl crate::Readable for SPR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPR4 to value 0"]
impl crate::Resettable for SPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
