#[doc = "Register `COLORCOMPONENTBITS0` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS0` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS0_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA0` reader - Alpha."]
pub type COMPONENTBITSALPHA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA0` writer - Alpha."]
pub type COMPONENTBITSALPHA0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS0_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE0` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE0` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS0_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN0` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN0` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS0_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED0` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED0` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT0` reader - When ComponentBitsRed/Green/Blue is set to 10 and this mode enabled (value 1), then input format is considered 8.2 instead of 10.0 bits (max color is 1020 instead of 1023). This is compliant to ITU 656 standard."]
pub type ITUFORMAT0_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT0` writer - When ComponentBitsRed/Green/Blue is set to 10 and this mode enabled (value 1), then input format is considered 8.2 instead of 10.0 bits (max color is 1020 instead of 1023). This is compliant to ITU 656 standard."]
pub type ITUFORMAT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha0(&self) -> COMPONENTBITSALPHA0_R {
        COMPONENTBITSALPHA0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue0(&self) -> COMPONENTBITSBLUE0_R {
        COMPONENTBITSBLUE0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen0(&self) -> COMPONENTBITSGREEN0_R {
        COMPONENTBITSGREEN0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred0(&self) -> COMPONENTBITSRED0_R {
        COMPONENTBITSRED0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - When ComponentBitsRed/Green/Blue is set to 10 and this mode enabled (value 1), then input format is considered 8.2 instead of 10.0 bits (max color is 1020 instead of 1023). This is compliant to ITU 656 standard."]
    #[inline(always)]
    pub fn ituformat0(&self) -> ITUFORMAT0_R {
        ITUFORMAT0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha0(&mut self) -> COMPONENTBITSALPHA0_W<0> {
        COMPONENTBITSALPHA0_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue0(&mut self) -> COMPONENTBITSBLUE0_W<8> {
        COMPONENTBITSBLUE0_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen0(&mut self) -> COMPONENTBITSGREEN0_W<16> {
        COMPONENTBITSGREEN0_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred0(&mut self) -> COMPONENTBITSRED0_W<24> {
        COMPONENTBITSRED0_W::new(self)
    }
    #[doc = "Bit 31 - When ComponentBitsRed/Green/Blue is set to 10 and this mode enabled (value 1), then input format is considered 8.2 instead of 10.0 bits (max color is 1020 instead of 1023). This is compliant to ITU 656 standard."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat0(&mut self) -> ITUFORMAT0_W<31> {
        ITUFORMAT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits0](index.html) module"]
pub struct COLORCOMPONENTBITS0_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits0::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits0::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS0 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
