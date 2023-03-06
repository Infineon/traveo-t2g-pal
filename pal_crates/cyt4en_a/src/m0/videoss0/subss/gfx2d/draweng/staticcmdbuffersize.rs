#[doc = "Register `STATICCMDBUFFERSIZE` reader"]
pub struct R(crate::R<STATICCMDBUFFERSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCMDBUFFERSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCMDBUFFERSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCMDBUFFERSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCMDBUFFERSIZE` writer"]
pub struct W(crate::W<STATICCMDBUFFERSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCMDBUFFERSIZE_SPEC>;
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
impl From<crate::W<STATICCMDBUFFERSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCMDBUFFERSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDBUFFERSIZE` reader - Command buffer size in number of 32-bit words. Only power-of-two values are allowed. The value programmed into this field must be one less than the actual number of words."]
pub type CMDBUFFERSIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDBUFFERSIZE` writer - Command buffer size in number of 32-bit words. Only power-of-two values are allowed. The value programmed into this field must be one less than the actual number of words."]
pub type CMDBUFFERSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCMDBUFFERSIZE_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Command buffer size in number of 32-bit words. Only power-of-two values are allowed. The value programmed into this field must be one less than the actual number of words."]
    #[inline(always)]
    pub fn cmdbuffersize(&self) -> CMDBUFFERSIZE_R {
        CMDBUFFERSIZE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Command buffer size in number of 32-bit words. Only power-of-two values are allowed. The value programmed into this field must be one less than the actual number of words."]
    #[inline(always)]
    #[must_use]
    pub fn cmdbuffersize(&mut self) -> CMDBUFFERSIZE_W<0> {
        CMDBUFFERSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcmdbuffersize](index.html) module"]
pub struct STATICCMDBUFFERSIZE_SPEC;
impl crate::RegisterSpec for STATICCMDBUFFERSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcmdbuffersize::R](R) reader structure"]
impl crate::Readable for STATICCMDBUFFERSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcmdbuffersize::W](W) writer structure"]
impl crate::Writable for STATICCMDBUFFERSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCMDBUFFERSIZE to value 0x01ff"]
impl crate::Resettable for STATICCMDBUFFERSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
