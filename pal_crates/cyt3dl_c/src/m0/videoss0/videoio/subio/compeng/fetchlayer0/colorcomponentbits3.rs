#[doc = "Register `COLORCOMPONENTBITS3` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS3` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS3_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA3` reader - Alpha."]
pub type COMPONENTBITSALPHA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA3` writer - Alpha."]
pub type COMPONENTBITSALPHA3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE3` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE3` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN3` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN3` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED3` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED3` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT3` reader - See ITUFormat0."]
pub type ITUFORMAT3_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT3` writer - See ITUFormat0."]
pub type ITUFORMAT3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha3(&self) -> COMPONENTBITSALPHA3_R {
        COMPONENTBITSALPHA3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue3(&self) -> COMPONENTBITSBLUE3_R {
        COMPONENTBITSBLUE3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen3(&self) -> COMPONENTBITSGREEN3_R {
        COMPONENTBITSGREEN3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred3(&self) -> COMPONENTBITSRED3_R {
        COMPONENTBITSRED3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat3(&self) -> ITUFORMAT3_R {
        ITUFORMAT3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha3(&mut self) -> COMPONENTBITSALPHA3_W<0> {
        COMPONENTBITSALPHA3_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue3(&mut self) -> COMPONENTBITSBLUE3_W<8> {
        COMPONENTBITSBLUE3_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen3(&mut self) -> COMPONENTBITSGREEN3_W<16> {
        COMPONENTBITSGREEN3_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred3(&mut self) -> COMPONENTBITSRED3_W<24> {
        COMPONENTBITSRED3_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat3(&mut self) -> ITUFORMAT3_W<31> {
        ITUFORMAT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 3).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits3](index.html) module"]
pub struct COLORCOMPONENTBITS3_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits3::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits3::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS3 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
