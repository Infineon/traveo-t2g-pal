#[doc = "Register `TASKBUFFEREXECOFFSET1` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB1` reader - Task buffer 1 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB1 has offset zero."]
pub type EXECOFFSETTB1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 1 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB1 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb1(&self) -> EXECOFFSETTB1_R {
        EXECOFFSETTB1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 1 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset1](index.html) module"]
pub struct TASKBUFFEREXECOFFSET1_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset1::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET1 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
