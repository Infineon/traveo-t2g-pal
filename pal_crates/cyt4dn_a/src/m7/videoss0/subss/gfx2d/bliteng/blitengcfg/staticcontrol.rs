#[doc = "Register `STATICCONTROL` reader"]
pub struct R(crate::R<STATICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONTROL` writer"]
pub struct W(crate::W<STATICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONTROL_SPEC>;
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
impl From<crate::W<STATICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `PERFMEASUREMENTEN` reader - Enable global performance measument in blit engine."]
pub type PERFMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `PERFMEASUREMENTEN` writer - Enable global performance measument in blit engine."]
pub type PERFMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `SLICEPERFMEASUREMENTEN` reader - Enable performance measument per slice in blit engine."]
pub type SLICEPERFMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `SLICEPERFMEASUREMENTEN` writer - Enable performance measument per slice in blit engine."]
pub type SLICEPERFMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `LRSTATUSSELECT` reader - Select pipeline from which to access status register information in line-rendering mode."]
pub type LRSTATUSSELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LRSTATUSSELECT` writer - Select pipeline from which to access status register information in line-rendering mode."]
pub type LRSTATUSSELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable global performance measument in blit engine."]
    #[inline(always)]
    pub fn perfmeasurementen(&self) -> PERFMEASUREMENTEN_R {
        PERFMEASUREMENTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable performance measument per slice in blit engine."]
    #[inline(always)]
    pub fn sliceperfmeasurementen(&self) -> SLICEPERFMEASUREMENTEN_R {
        SLICEPERFMEASUREMENTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select pipeline from which to access status register information in line-rendering mode."]
    #[inline(always)]
    pub fn lrstatusselect(&self) -> LRSTATUSSELECT_R {
        LRSTATUSSELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable global performance measument in blit engine."]
    #[inline(always)]
    #[must_use]
    pub fn perfmeasurementen(&mut self) -> PERFMEASUREMENTEN_W<1> {
        PERFMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable performance measument per slice in blit engine."]
    #[inline(always)]
    #[must_use]
    pub fn sliceperfmeasurementen(&mut self) -> SLICEPERFMEASUREMENTEN_W<2> {
        SLICEPERFMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Select pipeline from which to access status register information in line-rendering mode."]
    #[inline(always)]
    #[must_use]
    pub fn lrstatusselect(&mut self) -> LRSTATUSSELECT_W<4> {
        LRSTATUSSELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
pub struct STATICCONTROL_SPEC;
impl crate::RegisterSpec for STATICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcontrol::R](R) reader structure"]
impl crate::Readable for STATICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcontrol::W](W) writer structure"]
impl crate::Writable for STATICCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCONTROL to value 0"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
