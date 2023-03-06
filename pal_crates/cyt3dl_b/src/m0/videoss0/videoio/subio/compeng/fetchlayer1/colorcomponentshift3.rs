#[doc = "Register `COLORCOMPONENTSHIFT3` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT3` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT3_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA3` reader - Alpha."]
pub type COMPONENTSHIFTALPHA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA3` writer - Alpha."]
pub type COMPONENTSHIFTALPHA3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE3` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE3` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN3` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN3` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT3_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED3` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED3` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha3(&self) -> COMPONENTSHIFTALPHA3_R {
        COMPONENTSHIFTALPHA3_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue3(&self) -> COMPONENTSHIFTBLUE3_R {
        COMPONENTSHIFTBLUE3_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen3(&self) -> COMPONENTSHIFTGREEN3_R {
        COMPONENTSHIFTGREEN3_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred3(&self) -> COMPONENTSHIFTRED3_R {
        COMPONENTSHIFTRED3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha3(&mut self) -> COMPONENTSHIFTALPHA3_W<0> {
        COMPONENTSHIFTALPHA3_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue3(&mut self) -> COMPONENTSHIFTBLUE3_W<8> {
        COMPONENTSHIFTBLUE3_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen3(&mut self) -> COMPONENTSHIFTGREEN3_W<16> {
        COMPONENTSHIFTGREEN3_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred3(&mut self) -> COMPONENTSHIFTRED3_W<24> {
        COMPONENTSHIFTRED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 3).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift3](index.html) module"]
pub struct COLORCOMPONENTSHIFT3_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift3::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift3::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT3 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
