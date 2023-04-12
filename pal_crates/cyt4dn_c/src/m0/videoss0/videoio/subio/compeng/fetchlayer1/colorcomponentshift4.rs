#[doc = "Register `COLORCOMPONENTSHIFT4` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT4` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT4_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA4` reader - Alpha."]
pub type COMPONENTSHIFTALPHA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA4` writer - Alpha."]
pub type COMPONENTSHIFTALPHA4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT4_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE4` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE4` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT4_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN4` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN4` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT4_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED4` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED4` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha4(&self) -> COMPONENTSHIFTALPHA4_R {
        COMPONENTSHIFTALPHA4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue4(&self) -> COMPONENTSHIFTBLUE4_R {
        COMPONENTSHIFTBLUE4_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen4(&self) -> COMPONENTSHIFTGREEN4_R {
        COMPONENTSHIFTGREEN4_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred4(&self) -> COMPONENTSHIFTRED4_R {
        COMPONENTSHIFTRED4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha4(&mut self) -> COMPONENTSHIFTALPHA4_W<0> {
        COMPONENTSHIFTALPHA4_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue4(&mut self) -> COMPONENTSHIFTBLUE4_W<8> {
        COMPONENTSHIFTBLUE4_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen4(&mut self) -> COMPONENTSHIFTGREEN4_W<16> {
        COMPONENTSHIFTGREEN4_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred4(&mut self) -> COMPONENTSHIFTRED4_W<24> {
        COMPONENTSHIFTRED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 4).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift4](index.html) module"]
pub struct COLORCOMPONENTSHIFT4_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift4::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift4::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT4 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT4_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
