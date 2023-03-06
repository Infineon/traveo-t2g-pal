#[doc = "Register `COLORCOMPONENTBITS1` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS1` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS1_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA1` reader - Alpha."]
pub type COMPONENTBITSALPHA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA1` writer - Alpha."]
pub type COMPONENTBITSALPHA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE1` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE1` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN1` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN1` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED1` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED1` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT1` reader - See ITUFormat0."]
pub type ITUFORMAT1_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT1` writer - See ITUFormat0."]
pub type ITUFORMAT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha1(&self) -> COMPONENTBITSALPHA1_R {
        COMPONENTBITSALPHA1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue1(&self) -> COMPONENTBITSBLUE1_R {
        COMPONENTBITSBLUE1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen1(&self) -> COMPONENTBITSGREEN1_R {
        COMPONENTBITSGREEN1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred1(&self) -> COMPONENTBITSRED1_R {
        COMPONENTBITSRED1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat1(&self) -> ITUFORMAT1_R {
        ITUFORMAT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha1(&mut self) -> COMPONENTBITSALPHA1_W<0> {
        COMPONENTBITSALPHA1_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue1(&mut self) -> COMPONENTBITSBLUE1_W<8> {
        COMPONENTBITSBLUE1_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen1(&mut self) -> COMPONENTBITSGREEN1_W<16> {
        COMPONENTBITSGREEN1_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred1(&mut self) -> COMPONENTBITSRED1_W<24> {
        COMPONENTBITSRED1_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat1(&mut self) -> ITUFORMAT1_W<31> {
        ITUFORMAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits1](index.html) module"]
pub struct COLORCOMPONENTBITS1_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits1::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits1::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS1 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
