#[doc = "Register `TASKBUFFERSTOPOFFSET7` reader"]
pub struct R(crate::R<TASKBUFFERSTOPOFFSET7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERSTOPOFFSET7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERSTOPOFFSET7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERSTOPOFFSET7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERSTOPOFFSET7` writer"]
pub struct W(crate::W<TASKBUFFERSTOPOFFSET7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERSTOPOFFSET7_SPEC>;
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
impl From<crate::W<TASKBUFFERSTOPOFFSET7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERSTOPOFFSET7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPOFFSETTB7` reader - Task buffer 7 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
pub type STOPOFFSETTB7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STOPOFFSETTB7` writer - Task buffer 7 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
pub type STOPOFFSETTB7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERSTOPOFFSET7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 7 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
    #[inline(always)]
    pub fn stopoffsettb7(&self) -> STOPOFFSETTB7_R {
        STOPOFFSETTB7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Task buffer 7 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
    #[inline(always)]
    #[must_use]
    pub fn stopoffsettb7(&mut self) -> STOPOFFSETTB7_W<0> {
        STOPOFFSETTB7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 7 stop offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferstopoffset7](index.html) module"]
pub struct TASKBUFFERSTOPOFFSET7_SPEC;
impl crate::RegisterSpec for TASKBUFFERSTOPOFFSET7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferstopoffset7::R](R) reader structure"]
impl crate::Readable for TASKBUFFERSTOPOFFSET7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbufferstopoffset7::W](W) writer structure"]
impl crate::Writable for TASKBUFFERSTOPOFFSET7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERSTOPOFFSET7 to value 0"]
impl crate::Resettable for TASKBUFFERSTOPOFFSET7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
