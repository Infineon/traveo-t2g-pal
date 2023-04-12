#[doc = "Register `TASKBUFFEREXECOFFSET3` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB3` reader - Task buffer 3 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB3 has offset zero."]
pub type EXECOFFSETTB3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 3 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB3 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb3(&self) -> EXECOFFSETTB3_R {
        EXECOFFSETTB3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 3 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset3](index.html) module"]
pub struct TASKBUFFEREXECOFFSET3_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset3::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET3 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
