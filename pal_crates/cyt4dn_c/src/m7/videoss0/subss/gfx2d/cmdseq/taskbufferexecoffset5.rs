#[doc = "Register `TASKBUFFEREXECOFFSET5` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB5` reader - Task buffer 5 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB5 has offset zero."]
pub type EXECOFFSETTB5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 5 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB5 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb5(&self) -> EXECOFFSETTB5_R {
        EXECOFFSETTB5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 5 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset5](index.html) module"]
pub struct TASKBUFFEREXECOFFSET5_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset5::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET5 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
