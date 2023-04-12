#[doc = "Register `COLORCOMPONENTBITS5` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS5` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS5_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA5` reader - Alpha."]
pub type COMPONENTBITSALPHA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA5` writer - Alpha."]
pub type COMPONENTBITSALPHA5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS5_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE5` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE5` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS5_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN5` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN5` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS5_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED5` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED5` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS5_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT5` reader - See ITUFormat0."]
pub type ITUFORMAT5_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT5` writer - See ITUFormat0."]
pub type ITUFORMAT5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha5(&self) -> COMPONENTBITSALPHA5_R {
        COMPONENTBITSALPHA5_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue5(&self) -> COMPONENTBITSBLUE5_R {
        COMPONENTBITSBLUE5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen5(&self) -> COMPONENTBITSGREEN5_R {
        COMPONENTBITSGREEN5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred5(&self) -> COMPONENTBITSRED5_R {
        COMPONENTBITSRED5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat5(&self) -> ITUFORMAT5_R {
        ITUFORMAT5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha5(&mut self) -> COMPONENTBITSALPHA5_W<0> {
        COMPONENTBITSALPHA5_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue5(&mut self) -> COMPONENTBITSBLUE5_W<8> {
        COMPONENTBITSBLUE5_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen5(&mut self) -> COMPONENTBITSGREEN5_W<16> {
        COMPONENTBITSGREEN5_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred5(&mut self) -> COMPONENTBITSRED5_W<24> {
        COMPONENTBITSRED5_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat5(&mut self) -> ITUFORMAT5_W<31> {
        ITUFORMAT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 5).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits5](index.html) module"]
pub struct COLORCOMPONENTBITS5_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits5::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits5::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS5 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
