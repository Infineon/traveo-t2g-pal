#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVALIDATE` writer - Triggers invalidation of the cache."]
pub type INVALIDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `SAVE_AND_RESET_MEASUREMENTS` writer - All performance and utilization counters are implemented internally. Writing one to this field provides all measured values to corresponding configurations registers and resets the internal counters."]
pub type SAVE_AND_RESET_MEASUREMENTS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CH0_INVALIDATE` writer - Triggers invalidation of the cache for channel 0 if CH0_ENABLED is enabled."]
pub type CH0_INVALIDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CH1_INVALIDATE` writer - Triggers invalidation of the cache for channel 1 if CH1_ENABLED is enabled."]
pub type CH1_INVALIDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CH2_INVALIDATE` writer - Triggers invalidation of the cache for channel 2 if CH2_ENABLED is enabled."]
pub type CH2_INVALIDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CH3_INVALIDATE` writer - Triggers invalidation of the cache for channel 3 if CH3_ENABLED is enabled."]
pub type CH3_INVALIDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Triggers invalidation of the cache."]
    #[inline(always)]
    #[must_use]
    pub fn invalidate(&mut self) -> INVALIDATE_W<0> {
        INVALIDATE_W::new(self)
    }
    #[doc = "Bit 1 - All performance and utilization counters are implemented internally. Writing one to this field provides all measured values to corresponding configurations registers and resets the internal counters."]
    #[inline(always)]
    #[must_use]
    pub fn save_and_reset_measurements(&mut self) -> SAVE_AND_RESET_MEASUREMENTS_W<1> {
        SAVE_AND_RESET_MEASUREMENTS_W::new(self)
    }
    #[doc = "Bit 8 - Triggers invalidation of the cache for channel 0 if CH0_ENABLED is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_invalidate(&mut self) -> CH0_INVALIDATE_W<8> {
        CH0_INVALIDATE_W::new(self)
    }
    #[doc = "Bit 9 - Triggers invalidation of the cache for channel 1 if CH1_ENABLED is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_invalidate(&mut self) -> CH1_INVALIDATE_W<9> {
        CH1_INVALIDATE_W::new(self)
    }
    #[doc = "Bit 10 - Triggers invalidation of the cache for channel 2 if CH2_ENABLED is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_invalidate(&mut self) -> CH2_INVALIDATE_W<10> {
        CH2_INVALIDATE_W::new(self)
    }
    #[doc = "Bit 11 - Triggers invalidation of the cache for channel 3 if CH3_ENABLED is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_invalidate(&mut self) -> CH3_INVALIDATE_W<11> {
        CH3_INVALIDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control of the GfxCache.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
