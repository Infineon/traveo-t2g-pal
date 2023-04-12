#[doc = "Register `MAPBIT3_0_DUAL` reader"]
pub struct R(crate::R<MAPBIT3_0_DUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT3_0_DUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT3_0_DUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT3_0_DUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT3_0_DUAL` writer"]
pub struct W(crate::W<MAPBIT3_0_DUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT3_0_DUAL_SPEC>;
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
impl From<crate::W<MAPBIT3_0_DUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT3_0_DUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT0_DUAL` reader - Same as MapBit0 for 2nd channel"]
pub type MAPBIT0_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT0_DUAL` writer - Same as MapBit0 for 2nd channel"]
pub type MAPBIT0_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT3_0_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT1_DUAL` reader - Same as MapBit1 for 2nd channel"]
pub type MAPBIT1_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT1_DUAL` writer - Same as MapBit1 for 2nd channel"]
pub type MAPBIT1_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT3_0_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT2_DUAL` reader - Same as MapBit2 for 2nd channel"]
pub type MAPBIT2_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT2_DUAL` writer - Same as MapBit2 for 2nd channel"]
pub type MAPBIT2_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT3_0_DUAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT3_DUAL` reader - Same as MapBit3 for 2nd channel"]
pub type MAPBIT3_DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT3_DUAL` writer - Same as MapBit3 for 2nd channel"]
pub type MAPBIT3_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAPBIT3_0_DUAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Same as MapBit0 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit0_dual(&self) -> MAPBIT0_DUAL_R {
        MAPBIT0_DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as MapBit1 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit1_dual(&self) -> MAPBIT1_DUAL_R {
        MAPBIT1_DUAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as MapBit2 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit2_dual(&self) -> MAPBIT2_DUAL_R {
        MAPBIT2_DUAL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as MapBit3 for 2nd channel"]
    #[inline(always)]
    pub fn mapbit3_dual(&self) -> MAPBIT3_DUAL_R {
        MAPBIT3_DUAL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Same as MapBit0 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit0_dual(&mut self) -> MAPBIT0_DUAL_W<0> {
        MAPBIT0_DUAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Same as MapBit1 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit1_dual(&mut self) -> MAPBIT1_DUAL_W<8> {
        MAPBIT1_DUAL_W::new(self)
    }
    #[doc = "Bits 16:20 - Same as MapBit2 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit2_dual(&mut self) -> MAPBIT2_DUAL_W<16> {
        MAPBIT2_DUAL_W::new(self)
    }
    #[doc = "Bits 24:28 - Same as MapBit3 for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit3_dual(&mut self) -> MAPBIT3_DUAL_W<24> {
        MAPBIT3_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same as MapBit3_0 for 2nd channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit3_0_dual](index.html) module"]
pub struct MAPBIT3_0_DUAL_SPEC;
impl crate::RegisterSpec for MAPBIT3_0_DUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit3_0_dual::R](R) reader structure"]
impl crate::Readable for MAPBIT3_0_DUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit3_0_dual::W](W) writer structure"]
impl crate::Writable for MAPBIT3_0_DUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT3_0_DUAL to value 0x0302_0100"]
impl crate::Resettable for MAPBIT3_0_DUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0100;
}
