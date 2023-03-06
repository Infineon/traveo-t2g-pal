#[doc = "Register `STORE0BUF2` reader"]
pub struct R(crate::R<STORE0BUF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE0BUF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE0BUF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE0BUF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE0BUF2` writer"]
pub struct W(crate::W<STORE0BUF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE0BUF2_SPEC>;
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
impl From<crate::W<STORE0BUF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE0BUF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STORELENGTH0` reader - Destination buffer 0 length in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
pub type STORELENGTH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STORELENGTH0` writer - Destination buffer 0 length in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
pub type STORELENGTH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE0BUF2_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 6:31 - Destination buffer 0 length in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    pub fn storelength0(&self) -> STORELENGTH0_R {
        STORELENGTH0_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Destination buffer 0 length in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    #[must_use]
    pub fn storelength0(&mut self) -> STORELENGTH0_W<6> {
        STORELENGTH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer configuration (Y, YUV, RGB).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0buf2](index.html) module"]
pub struct STORE0BUF2_SPEC;
impl crate::RegisterSpec for STORE0BUF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store0buf2::R](R) reader structure"]
impl crate::Readable for STORE0BUF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store0buf2::W](W) writer structure"]
impl crate::Writable for STORE0BUF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE0BUF2 to value 0"]
impl crate::Resettable for STORE0BUF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
