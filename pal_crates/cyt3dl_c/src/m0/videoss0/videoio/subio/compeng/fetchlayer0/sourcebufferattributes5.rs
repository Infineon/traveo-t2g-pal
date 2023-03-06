#[doc = "Register `SOURCEBUFFERATTRIBUTES5` reader"]
pub struct R(crate::R<SOURCEBUFFERATTRIBUTES5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERATTRIBUTES5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERATTRIBUTES5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERATTRIBUTES5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERATTRIBUTES5` writer"]
pub struct W(crate::W<SOURCEBUFFERATTRIBUTES5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERATTRIBUTES5_SPEC>;
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
impl From<crate::W<SOURCEBUFFERATTRIBUTES5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERATTRIBUTES5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE5` reader - See Stride0."]
pub type STRIDE5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRIDE5` writer - See Stride0."]
pub type STRIDE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES5_SPEC, u16, u16, 16, O>;
#[doc = "Field `BITSPERPIXEL5` reader - See BitsPerPixel0."]
pub type BITSPERPIXEL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL5` writer - See BitsPerPixel0."]
pub type BITSPERPIXEL5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERATTRIBUTES5_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    pub fn stride5(&self) -> STRIDE5_R {
        STRIDE5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    pub fn bitsperpixel5(&self) -> BITSPERPIXEL5_R {
        BITSPERPIXEL5_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - See Stride0."]
    #[inline(always)]
    #[must_use]
    pub fn stride5(&mut self) -> STRIDE5_W<0> {
        STRIDE5_W::new(self)
    }
    #[doc = "Bits 16:21 - See BitsPerPixel0."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel5(&mut self) -> BITSPERPIXEL5_W<16> {
        BITSPERPIXEL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer attributes for layer 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferattributes5](index.html) module"]
pub struct SOURCEBUFFERATTRIBUTES5_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERATTRIBUTES5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferattributes5::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERATTRIBUTES5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferattributes5::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERATTRIBUTES5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERATTRIBUTES5 to value 0x0020_0003"]
impl crate::Resettable for SOURCEBUFFERATTRIBUTES5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0003;
}
