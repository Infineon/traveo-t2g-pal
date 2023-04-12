#[doc = "Register `COLORCOMPONENTSHIFT6` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT6` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT6_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA6` reader - Alpha."]
pub type COMPONENTSHIFTALPHA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA6` writer - Alpha."]
pub type COMPONENTSHIFTALPHA6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT6_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE6` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE6` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT6_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN6` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN6` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT6_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED6` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED6` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT6_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha6(&self) -> COMPONENTSHIFTALPHA6_R {
        COMPONENTSHIFTALPHA6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue6(&self) -> COMPONENTSHIFTBLUE6_R {
        COMPONENTSHIFTBLUE6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen6(&self) -> COMPONENTSHIFTGREEN6_R {
        COMPONENTSHIFTGREEN6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred6(&self) -> COMPONENTSHIFTRED6_R {
        COMPONENTSHIFTRED6_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha6(&mut self) -> COMPONENTSHIFTALPHA6_W<0> {
        COMPONENTSHIFTALPHA6_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue6(&mut self) -> COMPONENTSHIFTBLUE6_W<8> {
        COMPONENTSHIFTBLUE6_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen6(&mut self) -> COMPONENTSHIFTGREEN6_W<16> {
        COMPONENTSHIFTGREEN6_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred6(&mut self) -> COMPONENTSHIFTRED6_W<24> {
        COMPONENTSHIFTRED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 6).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift6](index.html) module"]
pub struct COLORCOMPONENTSHIFT6_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift6::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift6::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT6 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT6_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
