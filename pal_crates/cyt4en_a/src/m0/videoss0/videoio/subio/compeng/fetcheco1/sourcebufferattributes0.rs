#[doc = "Register `SOURCEBUFFERATTRIBUTES0` reader"]
pub struct R(crate::R<SOURCEBUFFERATTRIBUTES0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERATTRIBUTES0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERATTRIBUTES0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERATTRIBUTES0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERATTRIBUTES0` writer"]
pub struct W(crate::W<SOURCEBUFFERATTRIBUTES0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERATTRIBUTES0_SPEC>;
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
impl From<crate::W<SOURCEBUFFERATTRIBUTES0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERATTRIBUTES0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE0` reader - Source buffer stride of the layer in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
pub type STRIDE0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRIDE0` writer - Source buffer stride of the layer in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
pub type STRIDE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES0_SPEC, u16, u16, 16, O>;
#[doc = "Field `BITSPERPIXEL0` reader - Number of bits per pixel in the source buffer. Must be 1, 2, 4, 8, 16, 18, 24 or 32. \\[Exception\\]
FetchEco does not support 18 and 24."]
pub type BITSPERPIXEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL0` writer - Number of bits per pixel in the source buffer. Must be 1, 2, 4, 8, 16, 18, 24 or 32. \\[Exception\\]
FetchEco does not support 18 and 24."]
pub type BITSPERPIXEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - Source buffer stride of the layer in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
    #[inline(always)]
    pub fn stride0(&self) -> STRIDE0_R {
        STRIDE0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Number of bits per pixel in the source buffer. Must be 1, 2, 4, 8, 16, 18, 24 or 32. \\[Exception\\]
FetchEco does not support 18 and 24."]
    #[inline(always)]
    pub fn bitsperpixel0(&self) -> BITSPERPIXEL0_R {
        BITSPERPIXEL0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source buffer stride of the layer in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
    #[inline(always)]
    #[must_use]
    pub fn stride0(&mut self) -> STRIDE0_W<0> {
        STRIDE0_W::new(self)
    }
    #[doc = "Bits 16:21 - Number of bits per pixel in the source buffer. Must be 1, 2, 4, 8, 16, 18, 24 or 32. \\[Exception\\]
FetchEco does not support 18 and 24."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel0(&mut self) -> BITSPERPIXEL0_W<16> {
        BITSPERPIXEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer attributes for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferattributes0](index.html) module"]
pub struct SOURCEBUFFERATTRIBUTES0_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERATTRIBUTES0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferattributes0::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERATTRIBUTES0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferattributes0::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERATTRIBUTES0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERATTRIBUTES0 to value 0x0020_04ff"]
impl crate::Resettable for SOURCEBUFFERATTRIBUTES0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_04ff;
}
