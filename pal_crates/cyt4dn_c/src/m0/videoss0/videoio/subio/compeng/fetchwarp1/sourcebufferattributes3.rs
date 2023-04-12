#[doc = "Register `SOURCEBUFFERATTRIBUTES3` reader"]
pub struct R(crate::R<SOURCEBUFFERATTRIBUTES3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERATTRIBUTES3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERATTRIBUTES3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERATTRIBUTES3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERATTRIBUTES3` writer"]
pub struct W(crate::W<SOURCEBUFFERATTRIBUTES3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERATTRIBUTES3_SPEC>;
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
impl From<crate::W<SOURCEBUFFERATTRIBUTES3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERATTRIBUTES3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE3` reader - See Stride0."]
pub type STRIDE3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRIDE3` writer - See Stride0."]
pub type STRIDE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES3_SPEC, u16, u16, 16, O>;
#[doc = "Field `BITSPERPIXEL3` reader - See BitsPerPixel0."]
pub type BITSPERPIXEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL3` writer - See BitsPerPixel0."]
pub type BITSPERPIXEL3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES3_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    pub fn stride3(&self) -> STRIDE3_R {
        STRIDE3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    pub fn bitsperpixel3(&self) -> BITSPERPIXEL3_R {
        BITSPERPIXEL3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    #[must_use]
    pub fn stride3(&mut self) -> STRIDE3_W<0> {
        STRIDE3_W::new(self)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel3(&mut self) -> BITSPERPIXEL3_W<16> {
        BITSPERPIXEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer attributes for layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferattributes3](index.html) module"]
pub struct SOURCEBUFFERATTRIBUTES3_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERATTRIBUTES3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferattributes3::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERATTRIBUTES3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferattributes3::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERATTRIBUTES3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERATTRIBUTES3 to value 0x0020_0003"]
impl crate::Resettable for SOURCEBUFFERATTRIBUTES3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0003;
}
