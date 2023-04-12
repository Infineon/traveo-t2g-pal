#[doc = "Register `MAPBIT19_16_DUAL` reader"]
pub struct R(crate::R<MAPBIT19_16_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT19_16_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT19_16_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT19_16_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT19_16_DUAL` writer"]
pub struct W(crate::W<MAPBIT19_16_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT19_16_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT19_16_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT19_16_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT16_DUAL` reader - Same as MapBit16 for 2nd channel"]
pub type MAPBIT16_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT16_DUAL` writer - Same as MapBit16 for 2nd channel"]
pub type MAPBIT16_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT19_16_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT17_DUAL` reader - Same as MapBit17 for 2nd channel"]
pub type MAPBIT17_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT17_DUAL` writer - Same as MapBit17 for 2nd channel"]
pub type MAPBIT17_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT19_16_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT18_DUAL` reader - Same as MapBit18 for 2nd channel"]
pub type MAPBIT18_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT18_DUAL` writer - Same as MapBit18 for 2nd channel"]
pub type MAPBIT18_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT19_16_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT19_DUAL` reader - Same as MapBit19 for 2nd channel"]
pub type MAPBIT19_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT19_DUAL` writer - Same as MapBit19 for 2nd channel"]
pub type MAPBIT19_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT19_16_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit16 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit16_dual(&self) -> MAPBIT16_DUAL_R {
        MAPBIT16_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit17 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit17_dual(&self) -> MAPBIT17_DUAL_R {
        MAPBIT17_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit18 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit18_dual(&self) -> MAPBIT18_DUAL_R {
        MAPBIT18_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit19 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit19_dual(&self) -> MAPBIT19_DUAL_R {
        MAPBIT19_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit16 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit16_dual(&mut self) -> MAPBIT16_DUAL_W<0> {
        MAPBIT16_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit17 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit17_dual(&mut self) -> MAPBIT17_DUAL_W<8> {
        MAPBIT17_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit18 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit18_dual(&mut self) -> MAPBIT18_DUAL_W<16> {
        MAPBIT18_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit19 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit19_dual(&mut self) -> MAPBIT19_DUAL_W<24> {
        MAPBIT19_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit19_16 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit19_16_dual](index.html) module"]
pub struct MAPBIT19_16_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT19_16_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit19_16_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT19_16_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit19_16_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT19_16_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT19_16_DUAL to value 0x1312_1110"]
impl crate::Resettable for MAPBIT19_16_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1312_1110;
}
