#[doc = "Register `MAPBIT7_4_DUAL` reader"]
pub struct R(crate::R<MAPBIT7_4_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT7_4_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT7_4_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT7_4_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT7_4_DUAL` writer"]
pub struct W(crate::W<MAPBIT7_4_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT7_4_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT7_4_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT7_4_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT4_DUAL` reader - Same as MapBit4 for 2nd channel"]
pub type MAPBIT4_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT4_DUAL` writer - Same as MapBit4 for 2nd channel"]
pub type MAPBIT4_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT7_4_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT5_DUAL` reader - Same as MapBit5 for 2nd channel"]
pub type MAPBIT5_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT5_DUAL` writer - Same as MapBit5 for 2nd channel"]
pub type MAPBIT5_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT7_4_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT6_DUAL` reader - Same as MapBit6 for 2nd channel"]
pub type MAPBIT6_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT6_DUAL` writer - Same as MapBit6 for 2nd channel"]
pub type MAPBIT6_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT7_4_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT7_DUAL` reader - Same as MapBit7 for 2nd channel"]
pub type MAPBIT7_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT7_DUAL` writer - Same as MapBit7 for 2nd channel"]
pub type MAPBIT7_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT7_4_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit4 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit4_dual(&self) -> MAPBIT4_DUAL_R {
        MAPBIT4_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit5 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit5_dual(&self) -> MAPBIT5_DUAL_R {
        MAPBIT5_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit6 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit6_dual(&self) -> MAPBIT6_DUAL_R {
        MAPBIT6_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit7 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit7_dual(&self) -> MAPBIT7_DUAL_R {
        MAPBIT7_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit4 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit4_dual(&mut self) -> MAPBIT4_DUAL_W<0> {
        MAPBIT4_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit5 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit5_dual(&mut self) -> MAPBIT5_DUAL_W<8> {
        MAPBIT5_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit6 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit6_dual(&mut self) -> MAPBIT6_DUAL_W<16> {
        MAPBIT6_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit7 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit7_dual(&mut self) -> MAPBIT7_DUAL_W<24> {
        MAPBIT7_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit7_4 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit7_4_dual](index.html) module"]
pub struct MAPBIT7_4_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT7_4_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit7_4_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT7_4_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit7_4_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT7_4_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT7_4_DUAL to value 0x0706_0504"]
impl crate::Resettable for MAPBIT7_4_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0706_0504;
}
