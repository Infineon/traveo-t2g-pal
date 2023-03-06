#[doc = "Register `TASKBUFFERSTOPOFFSET4` reader"]
pub struct R(crate::R<TASKBUFFERSTOPOFFSET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERSTOPOFFSET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERSTOPOFFSET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERSTOPOFFSET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERSTOPOFFSET4` writer"]
pub struct W(crate::W<TASKBUFFERSTOPOFFSET4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERSTOPOFFSET4_SPEC>;
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
impl From<crate::W<TASKBUFFERSTOPOFFSET4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERSTOPOFFSET4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPOFFSETTB4` reader - Task buffer 4 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
pub type STOPOFFSETTB4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STOPOFFSETTB4` writer - Task buffer 4 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
pub type STOPOFFSETTB4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERSTOPOFFSET4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Task buffer 4 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
    #[inline(always)]
    pub fn stopoffsettb4(&self) -> STOPOFFSETTB4_R {
        STOPOFFSETTB4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Task buffer 4 stop offset in multiple of 32-bit words to determine the last valid scheduling instruction. Only scheduling instructions up to this offset are executed or with other words instruction at the StopOffset-1 is the last instruction which is executed."]
    #[inline(always)]
    #[must_use]
    pub fn stopoffsettb4(&mut self) -> STOPOFFSETTB4_W<0> {
        STOPOFFSETTB4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 4 stop offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferstopoffset4](index.html) module"]
pub struct TASKBUFFERSTOPOFFSET4_SPEC;
impl crate::RegisterSpec for TASKBUFFERSTOPOFFSET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferstopoffset4::R](R) reader structure"]
impl crate::Readable for TASKBUFFERSTOPOFFSET4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbufferstopoffset4::W](W) writer structure"]
impl crate::Writable for TASKBUFFERSTOPOFFSET4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERSTOPOFFSET4 to value 0"]
impl crate::Resettable for TASKBUFFERSTOPOFFSET4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
