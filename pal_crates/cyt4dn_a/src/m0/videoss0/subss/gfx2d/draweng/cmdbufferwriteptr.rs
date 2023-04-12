#[doc = "Register `CMDBUFFERWRITEPTR` reader"]
pub struct R(crate::R<CMDBUFFERWRITEPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDBUFFERWRITEPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDBUFFERWRITEPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDBUFFERWRITEPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDBUFFERWRITEPTR` writer"]
pub struct W(crate::W<CMDBUFFERWRITEPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDBUFFERWRITEPTR_SPEC>;
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
impl From<crate::W<CMDBUFFERWRITEPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDBUFFERWRITEPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDBUFFERWRITEPTR` reader - Command buffer write pointer in words. This pointer must be set by the software to one word after the last valid command (start of the next operation)."]
pub type CMDBUFFERWRITEPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDBUFFERWRITEPTR` writer - Command buffer write pointer in words. This pointer must be set by the software to one word after the last valid command (start of the next operation)."]
pub type CMDBUFFERWRITEPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDBUFFERWRITEPTR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Command buffer write pointer in words. This pointer must be set by the software to one word after the last valid command (start of the next operation)."]
    #[inline(always)]
    pub fn cmdbufferwriteptr(&self) -> CMDBUFFERWRITEPTR_R {
        CMDBUFFERWRITEPTR_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Command buffer write pointer in words. This pointer must be set by the software to one word after the last valid command (start of the next operation)."]
    #[inline(always)]
    #[must_use]
    pub fn cmdbufferwriteptr(&mut self) -> CMDBUFFERWRITEPTR_W<0> {
        CMDBUFFERWRITEPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer end address of command list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdbufferwriteptr](index.html) module"]
pub struct CMDBUFFERWRITEPTR_SPEC;
impl crate::RegisterSpec for CMDBUFFERWRITEPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdbufferwriteptr::R](R) reader structure"]
impl crate::Readable for CMDBUFFERWRITEPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdbufferwriteptr::W](W) writer structure"]
impl crate::Writable for CMDBUFFERWRITEPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDBUFFERWRITEPTR to value 0"]
impl crate::Resettable for CMDBUFFERWRITEPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
