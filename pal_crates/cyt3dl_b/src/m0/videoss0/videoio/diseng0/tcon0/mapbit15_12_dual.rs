#[doc = "Register `MAPBIT15_12_DUAL` reader"]
pub struct R(crate::R<MAPBIT15_12_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT15_12_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT15_12_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT15_12_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT15_12_DUAL` writer"]
pub struct W(crate::W<MAPBIT15_12_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT15_12_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT15_12_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT15_12_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT12_DUAL` reader - Same as MapBit12 for 2nd channel"]
pub type MAPBIT12_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT12_DUAL` writer - Same as MapBit12 for 2nd channel"]
pub type MAPBIT12_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT15_12_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT13_DUAL` reader - Same as MapBit13 for 2nd channel"]
pub type MAPBIT13_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT13_DUAL` writer - Same as MapBit13 for 2nd channel"]
pub type MAPBIT13_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT15_12_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT14_DUAL` reader - Same as MapBit14 for 2nd channel"]
pub type MAPBIT14_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT14_DUAL` writer - Same as MapBit14 for 2nd channel"]
pub type MAPBIT14_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT15_12_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT15_DUAL` reader - Same as MapBit15 for 2nd channel"]
pub type MAPBIT15_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT15_DUAL` writer - Same as MapBit15 for 2nd channel"]
pub type MAPBIT15_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT15_12_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit12 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit12_dual(&self) -> MAPBIT12_DUAL_R {
        MAPBIT12_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit13 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit13_dual(&self) -> MAPBIT13_DUAL_R {
        MAPBIT13_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit14 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit14_dual(&self) -> MAPBIT14_DUAL_R {
        MAPBIT14_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit15 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit15_dual(&self) -> MAPBIT15_DUAL_R {
        MAPBIT15_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit12 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit12_dual(&mut self) -> MAPBIT12_DUAL_W<0> {
        MAPBIT12_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit13 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit13_dual(&mut self) -> MAPBIT13_DUAL_W<8> {
        MAPBIT13_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit14 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit14_dual(&mut self) -> MAPBIT14_DUAL_W<16> {
        MAPBIT14_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit15 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit15_dual(&mut self) -> MAPBIT15_DUAL_W<24> {
        MAPBIT15_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit15_12 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit15_12_dual](index.html) module"]
pub struct MAPBIT15_12_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT15_12_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit15_12_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT15_12_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit15_12_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT15_12_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT15_12_DUAL to value 0x0f0e_0d0c"]
impl crate::Resettable for MAPBIT15_12_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0e_0d0c;
}
