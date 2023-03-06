#[doc = "Register `DESTINATIONBUFFERDIMENSIONS` reader"]
pub struct R(crate::R<DESTINATIONBUFFERDIMENSIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONBUFFERDIMENSIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONBUFFERDIMENSIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONBUFFERDIMENSIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONBUFFERDIMENSIONS` writer"]
pub struct W(crate::W<DESTINATIONBUFFERDIMENSIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONBUFFERDIMENSIONS_SPEC>;
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
impl From<crate::W<DESTINATIONBUFFERDIMENSIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONBUFFERDIMENSIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTBUFFERWIDTH` reader - Maximum width of the destination buffer in pixels minus one."]
pub type DSTBUFFERWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSTBUFFERWIDTH` writer - Maximum width of the destination buffer in pixels minus one."]
pub type DSTBUFFERWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERDIMENSIONS_SPEC, u16, u16, 14, O>;
#[doc = "Field `DSTBUFFERHEIGHT` reader - Maximum height of the destination buffer in pixels minus one."]
pub type DSTBUFFERHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSTBUFFERHEIGHT` writer - Maximum height of the destination buffer in pixels minus one."]
pub type DSTBUFFERHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERDIMENSIONS_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Maximum width of the destination buffer in pixels minus one."]
    #[inline(always)]
    pub fn dstbufferwidth(&self) -> DSTBUFFERWIDTH_R {
        DSTBUFFERWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Maximum height of the destination buffer in pixels minus one."]
    #[inline(always)]
    pub fn dstbufferheight(&self) -> DSTBUFFERHEIGHT_R {
        DSTBUFFERHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum width of the destination buffer in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn dstbufferwidth(&mut self) -> DSTBUFFERWIDTH_W<0> {
        DSTBUFFERWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Maximum height of the destination buffer in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn dstbufferheight(&mut self) -> DSTBUFFERHEIGHT_W<16> {
        DSTBUFFERHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum dimensions of the destination buffer. The resulting alpha frame is limited to this dimensions. However, the active area can be smaller and is given by ActiveDimensions register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationbufferdimensions](index.html) module"]
pub struct DESTINATIONBUFFERDIMENSIONS_SPEC;
impl crate::RegisterSpec for DESTINATIONBUFFERDIMENSIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationbufferdimensions::R](R) reader structure"]
impl crate::Readable for DESTINATIONBUFFERDIMENSIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationbufferdimensions::W](W) writer structure"]
impl crate::Writable for DESTINATIONBUFFERDIMENSIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONBUFFERDIMENSIONS to value 0"]
impl crate::Resettable for DESTINATIONBUFFERDIMENSIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
