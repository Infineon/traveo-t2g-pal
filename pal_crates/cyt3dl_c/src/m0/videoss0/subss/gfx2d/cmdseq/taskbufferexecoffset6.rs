#[doc = "Register `TASKBUFFEREXECOFFSET6` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB6` reader - Task buffer 6 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB6 has offset zero."]
pub type EXECOFFSETTB6_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 6 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at address = AddressTB6 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb6(&self) -> EXECOFFSETTB6_R {
        EXECOFFSETTB6_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 6 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset6](index.html) module"]
pub struct TASKBUFFEREXECOFFSET6_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset6::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET6 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
