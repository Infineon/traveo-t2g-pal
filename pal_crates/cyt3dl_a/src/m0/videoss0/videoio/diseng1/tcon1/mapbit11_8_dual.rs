#[doc = "Register `MAPBIT11_8_DUAL` reader"]
pub struct R(crate::R<MAPBIT11_8_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT11_8_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT11_8_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT11_8_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT11_8_DUAL` writer"]
pub struct W(crate::W<MAPBIT11_8_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT11_8_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT11_8_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT11_8_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT8_DUAL` reader - Same as MapBit8 for 2nd channel"]
pub type MAPBIT8_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT8_DUAL` writer - Same as MapBit8 for 2nd channel"]
pub type MAPBIT8_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT11_8_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT9_DUAL` reader - Same as MapBit9 for 2nd channel"]
pub type MAPBIT9_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT9_DUAL` writer - Same as MapBit9 for 2nd channel"]
pub type MAPBIT9_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT11_8_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT10_DUAL` reader - Same as MapBit10 for 2nd channel"]
pub type MAPBIT10_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT10_DUAL` writer - Same as MapBit10 for 2nd channel"]
pub type MAPBIT10_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT11_8_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT11_DUAL` reader - Same as MapBit11 for 2nd channel"]
pub type MAPBIT11_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT11_DUAL` writer - Same as MapBit11 for 2nd channel"]
pub type MAPBIT11_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT11_8_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit8 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit8_dual(&self) -> MAPBIT8_DUAL_R {
        MAPBIT8_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit9 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit9_dual(&self) -> MAPBIT9_DUAL_R {
        MAPBIT9_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit10 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit10_dual(&self) -> MAPBIT10_DUAL_R {
        MAPBIT10_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit11 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit11_dual(&self) -> MAPBIT11_DUAL_R {
        MAPBIT11_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit8 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit8_dual(&mut self) -> MAPBIT8_DUAL_W<0> {
        MAPBIT8_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit9 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit9_dual(&mut self) -> MAPBIT9_DUAL_W<8> {
        MAPBIT9_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit10 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit10_dual(&mut self) -> MAPBIT10_DUAL_W<16> {
        MAPBIT10_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit11 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit11_dual(&mut self) -> MAPBIT11_DUAL_W<24> {
        MAPBIT11_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit11_8 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit11_8_dual](index.html) module"]
pub struct MAPBIT11_8_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT11_8_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit11_8_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT11_8_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit11_8_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT11_8_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT11_8_DUAL to value 0x0b0a_0908"]
impl crate::Resettable for MAPBIT11_8_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b0a_0908;
}
