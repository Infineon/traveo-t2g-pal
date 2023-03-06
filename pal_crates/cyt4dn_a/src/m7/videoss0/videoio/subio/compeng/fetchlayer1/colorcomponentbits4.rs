#[doc = "Register `COLORCOMPONENTBITS4` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS4` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS4_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA4` reader - Alpha."]
pub type COMPONENTBITSALPHA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA4` writer - Alpha."]
pub type COMPONENTBITSALPHA4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS4_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE4` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE4` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS4_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN4` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN4` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS4_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED4` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED4` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS4_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT4` reader - See ITUFormat0."]
pub type ITUFORMAT4_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT4` writer - See ITUFormat0."]
pub type ITUFORMAT4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha4(&self) -> COMPONENTBITSALPHA4_R {
        COMPONENTBITSALPHA4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue4(&self) -> COMPONENTBITSBLUE4_R {
        COMPONENTBITSBLUE4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen4(&self) -> COMPONENTBITSGREEN4_R {
        COMPONENTBITSGREEN4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred4(&self) -> COMPONENTBITSRED4_R {
        COMPONENTBITSRED4_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat4(&self) -> ITUFORMAT4_R {
        ITUFORMAT4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha4(&mut self) -> COMPONENTBITSALPHA4_W<0> {
        COMPONENTBITSALPHA4_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue4(&mut self) -> COMPONENTBITSBLUE4_W<8> {
        COMPONENTBITSBLUE4_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen4(&mut self) -> COMPONENTBITSGREEN4_W<16> {
        COMPONENTBITSGREEN4_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred4(&mut self) -> COMPONENTBITSRED4_W<24> {
        COMPONENTBITSRED4_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat4(&mut self) -> ITUFORMAT4_W<31> {
        ITUFORMAT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 4).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits4](index.html) module"]
pub struct COLORCOMPONENTBITS4_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits4::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits4::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS4 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
