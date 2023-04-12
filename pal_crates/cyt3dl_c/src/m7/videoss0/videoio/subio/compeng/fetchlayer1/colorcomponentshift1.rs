#[doc = "Register `COLORCOMPONENTSHIFT1` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT1` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT1_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA1` reader - Alpha."]
pub type COMPONENTSHIFTALPHA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA1` writer - Alpha."]
pub type COMPONENTSHIFTALPHA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE1` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE1` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN1` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN1` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED1` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED1` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha1(&self) -> COMPONENTSHIFTALPHA1_R {
        COMPONENTSHIFTALPHA1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue1(&self) -> COMPONENTSHIFTBLUE1_R {
        COMPONENTSHIFTBLUE1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen1(&self) -> COMPONENTSHIFTGREEN1_R {
        COMPONENTSHIFTGREEN1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred1(&self) -> COMPONENTSHIFTRED1_R {
        COMPONENTSHIFTRED1_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha1(&mut self) -> COMPONENTSHIFTALPHA1_W<0> {
        COMPONENTSHIFTALPHA1_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue1(&mut self) -> COMPONENTSHIFTBLUE1_W<8> {
        COMPONENTSHIFTBLUE1_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen1(&mut self) -> COMPONENTSHIFTGREEN1_W<16> {
        COMPONENTSHIFTGREEN1_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred1(&mut self) -> COMPONENTSHIFTRED1_W<24> {
        COMPONENTSHIFTRED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift1](index.html) module"]
pub struct COLORCOMPONENTSHIFT1_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift1::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift1::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT1 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
