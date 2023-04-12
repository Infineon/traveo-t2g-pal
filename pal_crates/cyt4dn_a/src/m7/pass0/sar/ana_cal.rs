#[doc = "Register `ANA_CAL` reader"]
pub struct R(crate::R<ANA_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CAL` writer"]
pub struct W(crate::W<ANA_CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CAL_SPEC>;
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
impl From<crate::W<ANA_CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AOFFSET` reader - Analog offset correction"]
pub type AOFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AOFFSET` writer - Analog offset correction"]
pub type AOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `AGAIN` reader - Analog gain correction"]
pub type AGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGAIN` writer - Analog gain correction"]
pub type AGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:7 - Analog offset correction"]
    #[inline(always)]
    pub fn aoffset(&self) -> AOFFSET_R {
        AOFFSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Analog gain correction"]
    #[inline(always)]
    pub fn again(&self) -> AGAIN_R {
        AGAIN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog offset correction"]
    #[inline(always)]
    #[must_use]
    pub fn aoffset(&mut self) -> AOFFSET_W<0> {
        AOFFSET_W::new(self)
    }
    #[doc = "Bits 16:20 - Analog gain correction"]
    #[inline(always)]
    #[must_use]
    pub fn again(&mut self) -> AGAIN_W<16> {
        AGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current analog calibration values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_cal](index.html) module"]
pub struct ANA_CAL_SPEC;
impl crate::RegisterSpec for ANA_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_cal::R](R) reader structure"]
impl crate::Readable for ANA_CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_cal::W](W) writer structure"]
impl crate::Writable for ANA_CAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CAL to value 0"]
impl crate::Resettable for ANA_CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
