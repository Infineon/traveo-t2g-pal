#[doc = "Register `COLORCOMPONENTBITS2` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS2` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS2_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA2` reader - Alpha."]
pub type COMPONENTBITSALPHA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA2` writer - Alpha."]
pub type COMPONENTBITSALPHA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE2` reader - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE2` writer - Blue and V (chroma)."]
pub type COMPONENTBITSBLUE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN2` reader - Green and U (chroma)."]
pub type COMPONENTBITSGREEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN2` writer - Green and U (chroma)."]
pub type COMPONENTBITSGREEN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED2` reader - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED2` writer - Red, Y (luma) and palette index."]
pub type COMPONENTBITSRED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITUFORMAT2` reader - See ITUFormat0."]
pub type ITUFORMAT2_R = crate::BitReader<bool>;
#[doc = "Field `ITUFORMAT2` writer - See ITUFormat0."]
pub type ITUFORMAT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COLORCOMPONENTBITS2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    pub fn componentbitsalpha2(&self) -> COMPONENTBITSALPHA2_R {
        COMPONENTBITSALPHA2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    pub fn componentbitsblue2(&self) -> COMPONENTBITSBLUE2_R {
        COMPONENTBITSBLUE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    pub fn componentbitsgreen2(&self) -> COMPONENTBITSGREEN2_R {
        COMPONENTBITSGREEN2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    pub fn componentbitsred2(&self) -> COMPONENTBITSRED2_R {
        COMPONENTBITSRED2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    pub fn ituformat2(&self) -> ITUFORMAT2_R {
        ITUFORMAT2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha2(&mut self) -> COMPONENTBITSALPHA2_W<0> {
        COMPONENTBITSALPHA2_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue and V (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue2(&mut self) -> COMPONENTBITSBLUE2_W<8> {
        COMPONENTBITSBLUE2_W::new(self)
    }
    #[doc = "Bits 16:19 - Green and U (chroma)."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen2(&mut self) -> COMPONENTBITSGREEN2_W<16> {
        COMPONENTBITSGREEN2_W::new(self)
    }
    #[doc = "Bits 24:27 - Red, Y (luma) and palette index."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred2(&mut self) -> COMPONENTBITSRED2_W<24> {
        COMPONENTBITSRED2_W::new(self)
    }
    #[doc = "Bit 31 - See ITUFormat0."]
    #[inline(always)]
    #[must_use]
    pub fn ituformat2(&mut self) -> ITUFORMAT2_W<31> {
        ITUFORMAT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of color components for RGB, YUV and index formats (layer 2).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits2](index.html) module"]
pub struct COLORCOMPONENTBITS2_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits2::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits2::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS2 to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
