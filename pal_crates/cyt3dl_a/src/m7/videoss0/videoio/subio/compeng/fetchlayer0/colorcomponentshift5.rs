#[doc = "Register `COLORCOMPONENTSHIFT5` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT5` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT5_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA5` reader - Alpha."]
pub type COMPONENTSHIFTALPHA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA5` writer - Alpha."]
pub type COMPONENTSHIFTALPHA5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT5_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE5` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE5` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT5_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN5` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN5` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT5_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED5` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED5` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT5_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha5(&self) -> COMPONENTSHIFTALPHA5_R {
        COMPONENTSHIFTALPHA5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue5(&self) -> COMPONENTSHIFTBLUE5_R {
        COMPONENTSHIFTBLUE5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen5(&self) -> COMPONENTSHIFTGREEN5_R {
        COMPONENTSHIFTGREEN5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred5(&self) -> COMPONENTSHIFTRED5_R {
        COMPONENTSHIFTRED5_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha5(&mut self) -> COMPONENTSHIFTALPHA5_W<0> {
        COMPONENTSHIFTALPHA5_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue5(&mut self) -> COMPONENTSHIFTBLUE5_W<8> {
        COMPONENTSHIFTBLUE5_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen5(&mut self) -> COMPONENTSHIFTGREEN5_W<16> {
        COMPONENTSHIFTGREEN5_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred5(&mut self) -> COMPONENTSHIFTRED5_W<24> {
        COMPONENTSHIFTRED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 5).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift5](index.html) module"]
pub struct COLORCOMPONENTSHIFT5_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift5::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift5::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT5 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT5_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
