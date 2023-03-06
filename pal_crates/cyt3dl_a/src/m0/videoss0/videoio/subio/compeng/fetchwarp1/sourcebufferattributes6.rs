#[doc = "Register `SOURCEBUFFERATTRIBUTES6` reader"]
pub struct R(crate::R<SOURCEBUFFERATTRIBUTES6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERATTRIBUTES6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERATTRIBUTES6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERATTRIBUTES6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERATTRIBUTES6` writer"]
pub struct W(crate::W<SOURCEBUFFERATTRIBUTES6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERATTRIBUTES6_SPEC>;
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
impl From<crate::W<SOURCEBUFFERATTRIBUTES6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERATTRIBUTES6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE6` reader - See Stride0."]
pub type STRIDE6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRIDE6` writer - See Stride0."]
pub type STRIDE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES6_SPEC, u16, u16, 16, O>;
#[doc = "Field `BITSPERPIXEL6` reader - See BitsPerPixel0."]
pub type BITSPERPIXEL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL6` writer - See BitsPerPixel0."]
pub type BITSPERPIXEL6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES6_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    pub fn stride6(&self) -> STRIDE6_R {
        STRIDE6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    pub fn bitsperpixel6(&self) -> BITSPERPIXEL6_R {
        BITSPERPIXEL6_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    #[must_use]
    pub fn stride6(&mut self) -> STRIDE6_W<0> {
        STRIDE6_W::new(self)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel6(&mut self) -> BITSPERPIXEL6_W<16> {
        BITSPERPIXEL6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer attributes for layer 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferattributes6](index.html) module"]
pub struct SOURCEBUFFERATTRIBUTES6_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERATTRIBUTES6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferattributes6::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERATTRIBUTES6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferattributes6::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERATTRIBUTES6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERATTRIBUTES6 to value 0x0020_0003"]
impl crate::Resettable for SOURCEBUFFERATTRIBUTES6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0003;
}
