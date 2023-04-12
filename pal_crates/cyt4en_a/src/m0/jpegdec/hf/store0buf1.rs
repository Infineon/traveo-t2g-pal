#[doc = "Register `STORE0BUF1` reader"]
pub struct R(crate::R<STORE0BUF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE0BUF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE0BUF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE0BUF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE0BUF1` writer"]
pub struct W(crate::W<STORE0BUF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE0BUF1_SPEC>;
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
impl From<crate::W<STORE0BUF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE0BUF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STORESTRIDE0` reader - Line stride of the destination buffer 0 in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
pub type STORESTRIDE0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STORESTRIDE0` writer - Line stride of the destination buffer 0 in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
pub type STORESTRIDE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE0BUF1_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 6:31 - Line stride of the destination buffer 0 in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    pub fn storestride0(&self) -> STORESTRIDE0_R {
        STORESTRIDE0_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Line stride of the destination buffer 0 in multiple of 64 bytes. RWS field, which is activated by START or RESUME command."]
    #[inline(always)]
    #[must_use]
    pub fn storestride0(&mut self) -> STORESTRIDE0_W<6> {
        STORESTRIDE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer configuration (Y, YUV, RGB).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0buf1](index.html) module"]
pub struct STORE0BUF1_SPEC;
impl crate::RegisterSpec for STORE0BUF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store0buf1::R](R) reader structure"]
impl crate::Readable for STORE0BUF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store0buf1::W](W) writer structure"]
impl crate::Writable for STORE0BUF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE0BUF1 to value 0"]
impl crate::Resettable for STORE0BUF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
