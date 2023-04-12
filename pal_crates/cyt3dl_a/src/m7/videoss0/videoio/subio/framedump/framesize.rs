#[doc = "Register `FRAMESIZE` reader"]
pub struct R(crate::R<FRAMESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMESIZE` writer"]
pub struct W(crate::W<FRAMESIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMESIZE_SPEC>;
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
impl From<crate::W<FRAMESIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMESIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEWIDTH` reader - Width of output frame of framedump, given minus one. Do not change during operation."]
pub type FRAMEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEWIDTH` writer - Width of output frame of framedump, given minus one. Do not change during operation."]
pub type FRAMEWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMESIZE_SPEC, u16, u16, 14, O>;
#[doc = "Field `FRAMEHEIGHT` reader - Height of output frame of framedump, given minus one. Do not change during operation."]
pub type FRAMEHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEHEIGHT` writer - Height of output frame of framedump, given minus one. Do not change during operation."]
pub type FRAMEHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMESIZE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width of output frame of framedump, given minus one. Do not change during operation."]
    #[inline(always)]
    pub fn framewidth(&self) -> FRAMEWIDTH_R {
        FRAMEWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Height of output frame of framedump, given minus one. Do not change during operation."]
    #[inline(always)]
    pub fn frameheight(&self) -> FRAMEHEIGHT_R {
        FRAMEHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of output frame of framedump, given minus one. Do not change during operation."]
    #[inline(always)]
    #[must_use]
    pub fn framewidth(&mut self) -> FRAMEWIDTH_W<0> {
        FRAMEWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Height of output frame of framedump, given minus one. Do not change during operation."]
    #[inline(always)]
    #[must_use]
    pub fn frameheight(&mut self) -> FRAMEHEIGHT_W<16> {
        FRAMEHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dump window size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framesize](index.html) module"]
pub struct FRAMESIZE_SPEC;
impl crate::RegisterSpec for FRAMESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framesize::R](R) reader structure"]
impl crate::Readable for FRAMESIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framesize::W](W) writer structure"]
impl crate::Writable for FRAMESIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMESIZE to value 0x00ef_013f"]
impl crate::Resettable for FRAMESIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ef_013f;
}
