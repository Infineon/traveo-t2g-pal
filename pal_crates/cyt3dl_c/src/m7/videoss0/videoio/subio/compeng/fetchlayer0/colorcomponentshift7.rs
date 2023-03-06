#[doc = "Register `COLORCOMPONENTSHIFT7` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT7` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT7_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA7` reader - Alpha."]
pub type COMPONENTSHIFTALPHA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA7` writer - Alpha."]
pub type COMPONENTSHIFTALPHA7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT7_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE7` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE7` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT7_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN7` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN7` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT7_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED7` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED7` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT7_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha7(&self) -> COMPONENTSHIFTALPHA7_R {
        COMPONENTSHIFTALPHA7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue7(&self) -> COMPONENTSHIFTBLUE7_R {
        COMPONENTSHIFTBLUE7_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen7(&self) -> COMPONENTSHIFTGREEN7_R {
        COMPONENTSHIFTGREEN7_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred7(&self) -> COMPONENTSHIFTRED7_R {
        COMPONENTSHIFTRED7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha7(&mut self) -> COMPONENTSHIFTALPHA7_W<0> {
        COMPONENTSHIFTALPHA7_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue7(&mut self) -> COMPONENTSHIFTBLUE7_W<8> {
        COMPONENTSHIFTBLUE7_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen7(&mut self) -> COMPONENTSHIFTGREEN7_W<16> {
        COMPONENTSHIFTGREEN7_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred7(&mut self) -> COMPONENTSHIFTRED7_W<24> {
        COMPONENTSHIFTRED7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift7](index.html) module"]
pub struct COLORCOMPONENTSHIFT7_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift7::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift7::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT7 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT7_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
