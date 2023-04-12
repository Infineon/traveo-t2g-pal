#[doc = "Register `COLORCOMPONENTBITS6` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS6` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS6_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA6` reader - Alpha."]
pub type COMPONENTBITSALPHA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA6` writer - Alpha."]
pub type COMPONENTBITSALPHA6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS6_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE6` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE6` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS6_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN6` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN6` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS6_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED6` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED6` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS6_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT6` reader - See ITUFormat0."]
pub type ITUFORMAT6_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT6` writer - See ITUFormat0."]
pub type ITUFORMAT6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS6_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha6(&self) -> COMPONENTBITSALPHA6_R {
        COMPONENTBITSALPHA6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue6(&self) -> COMPONENTBITSBLUE6_R {
        COMPONENTBITSBLUE6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen6(&self) -> COMPONENTBITSGREEN6_R {
        COMPONENTBITSGREEN6_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred6(&self) -> COMPONENTBITSRED6_R {
        COMPONENTBITSRED6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat6(&self) -> ITUFORMAT6_R {
        ITUFORMAT6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha6(&mut self) -> COMPONENTBITSALPHA6_W<0> {
        COMPONENTBITSALPHA6_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue6(&mut self) -> COMPONENTBITSBLUE6_W<8> {
        COMPONENTBITSBLUE6_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen6(&mut self) -> COMPONENTBITSGREEN6_W<16> {
        COMPONENTBITSGREEN6_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred6(&mut self) -> COMPONENTBITSRED6_W<24> {
        COMPONENTBITSRED6_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat6(&mut self) -> ITUFORMAT6_W<31> {
        ITUFORMAT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 6).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits6](index.html) module"]
pub struct COLORCOMPONENTBITS6_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits6::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits6::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS6 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
