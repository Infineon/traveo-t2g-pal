#[doc = "Register `TASKBUFFEREXECOFFSET7` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB7` reader - Task buffer 7 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB7 has offset zero."]
pub type EXECOFFSETTB7_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 7 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB7 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb7(&self) -> EXECOFFSETTB7_R {
        EXECOFFSETTB7_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 7 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset7](index.html) module"]
pub struct TASKBUFFEREXECOFFSET7_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset7::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET7 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
