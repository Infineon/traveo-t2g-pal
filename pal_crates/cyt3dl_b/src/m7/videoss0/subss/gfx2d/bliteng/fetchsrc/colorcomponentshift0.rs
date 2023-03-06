#[doc = "Register `COLORCOMPONENTSHIFT0` reader"]
pub struct R(crate::R<COLORCOMPONENTSHIFT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTSHIFT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTSHIFT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTSHIFT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTSHIFT0` writer"]
pub struct W(crate::W<COLORCOMPONENTSHIFT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTSHIFT0_SPEC>;
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
impl From<crate::W<COLORCOMPONENTSHIFT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTSHIFT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTSHIFTALPHA0` reader - Alpha."]
pub type COMPONENTSHIFTALPHA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTALPHA0` writer - Alpha."]
pub type COMPONENTSHIFTALPHA0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTBLUE0` reader - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTBLUE0` writer - Blue and V (chroma)."]
pub type COMPONENTSHIFTBLUE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTGREEN0` reader - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTGREEN0` writer - Green and U (chroma)."]
pub type COMPONENTSHIFTGREEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPONENTSHIFTRED0` reader - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTSHIFTRED0` writer - Red, Y (luma) and palette index."]
pub type COMPONENTSHIFTRED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTSHIFT0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    pub fn componentshiftalpha0(&self) -> COMPONENTSHIFTALPHA0_R {
        COMPONENTSHIFTALPHA0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentshiftblue0(&self) -> COMPONENTSHIFTBLUE0_R {
        COMPONENTSHIFTBLUE0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentshiftgreen0(&self) -> COMPONENTSHIFTGREEN0_R {
        COMPONENTSHIFTGREEN0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentshiftred0(&self) -> COMPONENTSHIFTRED0_R {
        COMPONENTSHIFTRED0_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftalpha0(&mut self) -> COMPONENTSHIFTALPHA0_W<0> {
        COMPONENTSHIFTALPHA0_W::new(self)
    }
    #[doc = "Bits 8:12 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftblue0(&mut self) -> COMPONENTSHIFTBLUE0_W<8> {
        COMPONENTSHIFTBLUE0_W::new(self)
    }
    #[doc = "Bits 16:20 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftgreen0(&mut self) -> COMPONENTSHIFTGREEN0_W<16> {
        COMPONENTSHIFTGREEN0_W::new(self)
    }
    #[doc = "Bits 24:28 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentshiftred0(&mut self) -> COMPONENTSHIFTRED0_W<24> {
        COMPONENTSHIFTRED0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit position of color components for RGB, YUV and index formats (layer 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentshift0](index.html) module"]
pub struct COLORCOMPONENTSHIFT0_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTSHIFT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentshift0::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTSHIFT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentshift0::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTSHIFT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTSHIFT0 to value 0x1810_0800"]
impl crate::Resettable for COLORCOMPONENTSHIFT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1810_0800;
}
