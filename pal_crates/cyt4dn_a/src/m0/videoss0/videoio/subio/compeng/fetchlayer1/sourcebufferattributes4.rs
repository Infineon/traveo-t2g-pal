#[doc = "Register `SOURCEBUFFERATTRIBUTES4` reader"]
pub struct R(crate::R<SOURCEBUFFERATTRIBUTES4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERATTRIBUTES4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERATTRIBUTES4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERATTRIBUTES4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERATTRIBUTES4` writer"]
pub struct W(crate::W<SOURCEBUFFERATTRIBUTES4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERATTRIBUTES4_SPEC>;
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
impl From<crate::W<SOURCEBUFFERATTRIBUTES4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERATTRIBUTES4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE4` reader - See Stride0."]
pub type STRIDE4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRIDE4` writer - See Stride0."]
pub type STRIDE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES4_SPEC, u16, u16, 16, O>;
#[doc = "Field `BITSPERPIXEL4` reader - See BitsPerPixel0."]
pub type BITSPERPIXEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL4` writer - See BitsPerPixel0."]
pub type BITSPERPIXEL4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES4_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    pub fn stride4(&self) -> STRIDE4_R {
        STRIDE4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    pub fn bitsperpixel4(&self) -> BITSPERPIXEL4_R {
        BITSPERPIXEL4_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    #[must_use]
    pub fn stride4(&mut self) -> STRIDE4_W<0> {
        STRIDE4_W::new(self)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel4(&mut self) -> BITSPERPIXEL4_W<16> {
        BITSPERPIXEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer attributes for layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferattributes4](index.html) module"]
pub struct SOURCEBUFFERATTRIBUTES4_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERATTRIBUTES4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferattributes4::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERATTRIBUTES4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferattributes4::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERATTRIBUTES4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERATTRIBUTES4 to value 0x0020_0003"]
impl crate::Resettable for SOURCEBUFFERATTRIBUTES4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0003;
}
