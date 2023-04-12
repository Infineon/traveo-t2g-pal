#[doc = "Register `TASKBUFFERSIZE7` reader"]
pub struct R(crate::R<TASKBUFFERSIZE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERSIZE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERSIZE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERSIZE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERSIZE7` writer"]
pub struct W(crate::W<TASKBUFFERSIZE7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERSIZE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TASKBUFFERSIZE7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERSIZE7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZETB7` reader - Task buffer 7 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
pub type SIZETB7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIZETB7` writer - Task buffer 7 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
pub type SIZETB7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERSIZE7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 7 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
    #[inline(always)]
    pub fn sizetb7(&self) -> SIZETB7_R {
        SIZETB7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Task buffer 7 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn sizetb7(&mut self) -> SIZETB7_W<0> {
        SIZETB7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 7 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbuffersize7](index.html) module"]
pub struct TASKBUFFERSIZE7_SPEC;
impl crate::RegisterSpec for TASKBUFFERSIZE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbuffersize7::R](R) reader structure"]
impl crate::Readable for TASKBUFFERSIZE7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbuffersize7::W](W) writer structure"]
impl crate::Writable for TASKBUFFERSIZE7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERSIZE7 to value 0"]
impl crate::Resettable for TASKBUFFERSIZE7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
