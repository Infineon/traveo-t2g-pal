#[doc = "Register `COLORCOMPONENTBITS` reader"]
pub struct R(crate::R<COLORCOMPONENTBITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORCOMPONENTBITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORCOMPONENTBITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORCOMPONENTBITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORCOMPONENTBITS` writer"]
pub struct W(crate::W<COLORCOMPONENTBITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORCOMPONENTBITS_SPEC>;
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
impl From<crate::W<COLORCOMPONENTBITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORCOMPONENTBITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPONENTBITSALPHA` reader - Alpha component bits."]
pub type COMPONENTBITSALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSALPHA` writer - Alpha component bits."]
pub type COMPONENTBITSALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSBLUE` reader - Blue/V component bits."]
pub type COMPONENTBITSBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSBLUE` writer - Blue/V component bits."]
pub type COMPONENTBITSBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSGREEN` reader - Green/U component bits."]
pub type COMPONENTBITSGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSGREEN` writer - Green/U component bits."]
pub type COMPONENTBITSGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPONENTBITSRED` reader - Red/Y component bits."]
pub type COMPONENTBITSRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPONENTBITSRED` writer - Red/Y component bits."]
pub type COMPONENTBITSRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORCOMPONENTBITS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Alpha component bits."]
    #[inline(always)]
    pub fn componentbitsalpha(&self) -> COMPONENTBITSALPHA_R {
        COMPONENTBITSALPHA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Blue/V component bits."]
    #[inline(always)]
    pub fn componentbitsblue(&self) -> COMPONENTBITSBLUE_R {
        COMPONENTBITSBLUE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Green/U component bits."]
    #[inline(always)]
    pub fn componentbitsgreen(&self) -> COMPONENTBITSGREEN_R {
        COMPONENTBITSGREEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Red/Y component bits."]
    #[inline(always)]
    pub fn componentbitsred(&self) -> COMPONENTBITSRED_R {
        COMPONENTBITSRED_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alpha component bits."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsalpha(&mut self) -> COMPONENTBITSALPHA_W<0> {
        COMPONENTBITSALPHA_W::new(self)
    }
    #[doc = "Bits 8:11 - Blue/V component bits."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsblue(&mut self) -> COMPONENTBITSBLUE_W<8> {
        COMPONENTBITSBLUE_W::new(self)
    }
    #[doc = "Bits 16:19 - Green/U component bits."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsgreen(&mut self) -> COMPONENTBITSGREEN_W<16> {
        COMPONENTBITSGREEN_W::new(self)
    }
    #[doc = "Bits 24:27 - Red/Y component bits."]
    #[inline(always)]
    #[must_use]
    pub fn componentbitsred(&mut self) -> COMPONENTBITSRED_W<24> {
        COMPONENTBITSRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color component size of destination buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorcomponentbits](index.html) module"]
pub struct COLORCOMPONENTBITS_SPEC;
impl crate::RegisterSpec for COLORCOMPONENTBITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorcomponentbits::R](R) reader structure"]
impl crate::Readable for COLORCOMPONENTBITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorcomponentbits::W](W) writer structure"]
impl crate::Writable for COLORCOMPONENTBITS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORCOMPONENTBITS to value 0x0808_0808"]
impl crate::Resettable for COLORCOMPONENTBITS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
