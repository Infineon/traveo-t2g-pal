#[doc = "Register `TASKBUFFEREXECOFFSET4` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB4` reader - Task buffer 4 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB4 has offset zero."]
pub type EXECOFFSETTB4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 4 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB4 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb4(&self) -> EXECOFFSETTB4_R {
        EXECOFFSETTB4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 4 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset4](index.html) module"]
pub struct TASKBUFFEREXECOFFSET4_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset4::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET4 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
