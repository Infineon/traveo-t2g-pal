#[doc = "Register `COLORCOMPONENTBITS7` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS7` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS7_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA7` reader - Alpha."]
pub type COMPONENTBITSALPHA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA7` writer - Alpha."]
pub type COMPONENTBITSALPHA7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS7_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE7` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE7` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS7_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN7` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN7` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS7_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED7` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED7` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS7_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT7` reader - See ITUFormat0."]
pub type ITUFORMAT7_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT7` writer - See ITUFormat0."]
pub type ITUFORMAT7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha7(&self) -> COMPONENTBITSALPHA7_R {
        COMPONENTBITSALPHA7_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue7(&self) -> COMPONENTBITSBLUE7_R {
        COMPONENTBITSBLUE7_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen7(&self) -> COMPONENTBITSGREEN7_R {
        COMPONENTBITSGREEN7_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred7(&self) -> COMPONENTBITSRED7_R {
        COMPONENTBITSRED7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat7(&self) -> ITUFORMAT7_R {
        ITUFORMAT7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha7(&mut self) -> COMPONENTBITSALPHA7_W<0> {
        COMPONENTBITSALPHA7_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue7(&mut self) -> COMPONENTBITSBLUE7_W<8> {
        COMPONENTBITSBLUE7_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen7(&mut self) -> COMPONENTBITSGREEN7_W<16> {
        COMPONENTBITSGREEN7_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred7(&mut self) -> COMPONENTBITSRED7_W<24> {
        COMPONENTBITSRED7_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat7(&mut self) -> ITUFORMAT7_W<31> {
        ITUFORMAT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits7](index.html) module"]
pub struct COLORCOMPONENTBITS7_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits7::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits7::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS7 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
