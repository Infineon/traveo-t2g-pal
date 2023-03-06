#[doc = "Register `TASKBUFFEREXECOFFSET2` reader"]
pub struct R(crate::R<TASKBUFFEREXECOFFSET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFEREXECOFFSET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFEREXECOFFSET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFEREXECOFFSET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXECOFFSETTB2` reader - Task buffer 2 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at adress = AddressTB2 has offset zero."]
pub type EXECOFFSETTB2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 2 execution offset in multiple of 32-bit words. Offset of the scheduling instruction that is to be executed next. Instruction at adress = AddressTB2 has offset zero."]
    #[inline(always)]
    pub fn execoffsettb2(&self) -> EXECOFFSETTB2_R {
        EXECOFFSETTB2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Task buffer 2 execute offset register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferexecoffset2](index.html) module"]
pub struct TASKBUFFEREXECOFFSET2_SPEC;
impl crate::RegisterSpec for TASKBUFFEREXECOFFSET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferexecoffset2::R](R) reader structure"]
impl crate::Readable for TASKBUFFEREXECOFFSET2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TASKBUFFEREXECOFFSET2 to value 0"]
impl crate::Resettable for TASKBUFFEREXECOFFSET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
