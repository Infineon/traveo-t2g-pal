#[doc = "Register `PERFORMANCEMEASUREMENTCONTROL` writer"]
pub struct W(crate::W<PERFORMANCEMEASUREMENTCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFORMANCEMEASUREMENTCONTROL_SPEC>;
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
impl From<crate::W<PERFORMANCEMEASUREMENTCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFORMANCEMEASUREMENTCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAVEANDRESETMEASUREMENTS` writer - All performance and utilization counters are implemented internally. Writing one to this field provides all measured values to corresponding configurations registers and resets the internal counters."]
pub type SAVEANDRESETMEASUREMENTS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERFORMANCEMEASUREMENTCONTROL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - All performance and utilization counters are implemented internally. Writing one to this field provides all measured values to corresponding configurations registers and resets the internal counters."]
    #[inline(always)]
    #[must_use]
    pub fn saveandresetmeasurements(&mut self) -> SAVEANDRESETMEASUREMENTS_W<0> {
        SAVEANDRESETMEASUREMENTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control of the performance and utilitzation measurements.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [performancemeasurementcontrol](index.html) module"]
pub struct PERFORMANCEMEASUREMENTCONTROL_SPEC;
impl crate::RegisterSpec for PERFORMANCEMEASUREMENTCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [performancemeasurementcontrol::W](W) writer structure"]
impl crate::Writable for PERFORMANCEMEASUREMENTCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERFORMANCEMEASUREMENTCONTROL to value 0"]
impl crate::Resettable for PERFORMANCEMEASUREMENTCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
