#[doc = "Register `MAPBIT23_20_DUAL` reader"]
pub struct R(crate::R<MAPBIT23_20_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT23_20_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT23_20_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT23_20_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT23_20_DUAL` writer"]
pub struct W(crate::W<MAPBIT23_20_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT23_20_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT23_20_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT23_20_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT20_DUAL` reader - Same as MapBit20 for 2nd channel"]
pub type MAPBIT20_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT20_DUAL` writer - Same as MapBit20 for 2nd channel"]
pub type MAPBIT20_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT23_20_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT21_DUAL` reader - Same as MapBit21 for 2nd channel"]
pub type MAPBIT21_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT21_DUAL` writer - Same as MapBit21 for 2nd channel"]
pub type MAPBIT21_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT23_20_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT22_DUAL` reader - Same as MapBit22 for 2nd channel"]
pub type MAPBIT22_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT22_DUAL` writer - Same as MapBit22 for 2nd channel"]
pub type MAPBIT22_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT23_20_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT23_DUAL` reader - Same as MapBit23 for 2nd channel"]
pub type MAPBIT23_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT23_DUAL` writer - Same as MapBit23 for 2nd channel"]
pub type MAPBIT23_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT23_20_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit20 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit20_dual(&self) -> MAPBIT20_DUAL_R {
        MAPBIT20_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit21 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit21_dual(&self) -> MAPBIT21_DUAL_R {
        MAPBIT21_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit22 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit22_dual(&self) -> MAPBIT22_DUAL_R {
        MAPBIT22_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit23 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit23_dual(&self) -> MAPBIT23_DUAL_R {
        MAPBIT23_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit20 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit20_dual(&mut self) -> MAPBIT20_DUAL_W<0> {
        MAPBIT20_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit21 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit21_dual(&mut self) -> MAPBIT21_DUAL_W<8> {
        MAPBIT21_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit22 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit22_dual(&mut self) -> MAPBIT22_DUAL_W<16> {
        MAPBIT22_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit23 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit23_dual(&mut self) -> MAPBIT23_DUAL_W<24> {
        MAPBIT23_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit23_20 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit23_20_dual](index.html) module"]
pub struct MAPBIT23_20_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT23_20_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit23_20_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT23_20_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit23_20_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT23_20_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT23_20_DUAL to value 0x1716_1514"]
impl crate::Resettable for MAPBIT23_20_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1716_1514;
}
