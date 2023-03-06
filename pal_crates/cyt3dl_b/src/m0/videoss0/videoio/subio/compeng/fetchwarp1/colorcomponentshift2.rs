#[doc = "Register `COLORCOMPONENTSHIFT2` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT2` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT2_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA2` reader - Alpha."]
pub type COMPONENTSHIFTALPHA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA2` writer - Alpha."]
pub type COMPONENTSHIFTALPHA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT2_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE2` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE2` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT2_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN2` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN2` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT2_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED2` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED2` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha2(&self) -> COMPONENTSHIFTALPHA2_R {
        COMPONENTSHIFTALPHA2_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue2(&self) -> COMPONENTSHIFTBLUE2_R {
        COMPONENTSHIFTBLUE2_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen2(&self) -> COMPONENTSHIFTGREEN2_R {
        COMPONENTSHIFTGREEN2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred2(&self) -> COMPONENTSHIFTRED2_R {
        COMPONENTSHIFTRED2_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha2(&mut self) -> COMPONENTSHIFTALPHA2_W<0> {
        COMPONENTSHIFTALPHA2_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue2(&mut self) -> COMPONENTSHIFTBLUE2_W<8> {
        COMPONENTSHIFTBLUE2_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen2(&mut self) -> COMPONENTSHIFTGREEN2_W<16> {
        COMPONENTSHIFTGREEN2_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred2(&mut self) -> COMPONENTSHIFTRED2_W<24> {
        COMPONENTSHIFTRED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 2).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift2](index.html) module"]
pub struct COLORCOMPONENTSHIFT2_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift2::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift2::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT2 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
