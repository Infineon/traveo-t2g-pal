#[doc = "Register `MAPBIT27_24_DUAL` reader"]
pub struct R(crate::R<MAPBIT27_24_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT27_24_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT27_24_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT27_24_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT27_24_DUAL` writer"]
pub struct W(crate::W<MAPBIT27_24_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT27_24_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT27_24_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT27_24_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT24_DUAL` reader - Same as MapBit24 for 2nd channel"]
pub type MAPBIT24_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT24_DUAL` writer - Same as MapBit24 for 2nd channel"]
pub type MAPBIT24_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT27_24_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT25_DUAL` reader - Same as MapBit25 for 2nd channel"]
pub type MAPBIT25_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT25_DUAL` writer - Same as MapBit25 for 2nd channel"]
pub type MAPBIT25_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT27_24_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT26_DUAL` reader - Same as MapBit26 for 2nd channel"]
pub type MAPBIT26_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT26_DUAL` writer - Same as MapBit26 for 2nd channel"]
pub type MAPBIT26_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT27_24_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT27_DUAL` reader - Same as MapBit27 for 2nd channel"]
pub type MAPBIT27_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT27_DUAL` writer - Same as MapBit27 for 2nd channel"]
pub type MAPBIT27_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT27_24_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit24 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit24_dual(&self) -> MAPBIT24_DUAL_R {
        MAPBIT24_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit25 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit25_dual(&self) -> MAPBIT25_DUAL_R {
        MAPBIT25_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit26 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit26_dual(&self) -> MAPBIT26_DUAL_R {
        MAPBIT26_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit27 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit27_dual(&self) -> MAPBIT27_DUAL_R {
        MAPBIT27_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit24 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit24_dual(&mut self) -> MAPBIT24_DUAL_W<0> {
        MAPBIT24_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit25 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit25_dual(&mut self) -> MAPBIT25_DUAL_W<8> {
        MAPBIT25_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit26 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit26_dual(&mut self) -> MAPBIT26_DUAL_W<16> {
        MAPBIT26_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit27 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit27_dual(&mut self) -> MAPBIT27_DUAL_W<24> {
        MAPBIT27_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit27_24 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit27_24_dual](index.html) module"]
pub struct MAPBIT27_24_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT27_24_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit27_24_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT27_24_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit27_24_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT27_24_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT27_24_DUAL to value 0x1b1a_1918"]
impl crate::Resettable for MAPBIT27_24_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1b1a_1918;
}
