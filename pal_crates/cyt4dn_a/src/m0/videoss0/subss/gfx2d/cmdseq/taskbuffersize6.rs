#[doc = "Register `TASKBUFFERSIZE6` reader"]
pub struct R(crate::R<TASKBUFFERSIZE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERSIZE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERSIZE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERSIZE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERSIZE6` writer"]
pub struct W(crate::W<TASKBUFFERSIZE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERSIZE6_SPEC>;
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
impl From<crate::W<TASKBUFFERSIZE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERSIZE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZETB6` reader - Task buffer 6 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
pub type SIZETB6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIZETB6` writer - Task buffer 6 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
pub type SIZETB6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERSIZE6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 6 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
    #[inline(always)]
    pub fn sizetb6(&self) -> SIZETB6_R {
        SIZETB6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Task buffer 6 size in multiple of 32-bit words. The maximal size of task buffer equals to 4 * (2^16 - 1) bytes. The minimal size required to start execution is 2, because the minimal value of the stop offset is 1 and it has to be smaller then the task buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn sizetb6(&mut self) -> SIZETB6_W<0> {
        SIZETB6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 6 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbuffersize6](index.html) module"]
pub struct TASKBUFFERSIZE6_SPEC;
impl crate::RegisterSpec for TASKBUFFERSIZE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbuffersize6::R](R) reader structure"]
impl crate::Readable for TASKBUFFERSIZE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbuffersize6::W](W) writer structure"]
impl crate::Writable for TASKBUFFERSIZE6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERSIZE6 to value 0"]
impl crate::Resettable for TASKBUFFERSIZE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
