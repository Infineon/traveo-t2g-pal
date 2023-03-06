#[doc = "Register `TASKBUFFEREXECOFFSET0` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB0` reader - Task buffer 0 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at adress = AddressTB0 has offset zero."]
pub type EXECOFFSETTB0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 0 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at adress = AddressTB0 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb0(&self) -> EXECOFFSETTB0_R {
        EXECOFFSETTB0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 0 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset0](index.html) module"]
pub struct TASKBUFFEREXECOFFSET0_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset0::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET0 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
