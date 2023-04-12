#[doc = "Register `LPECO_PRESCALE` reader"]
pub struct R(crate::R<LPECO_PRESCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPECO_PRESCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPECO_PRESCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPECO_PRESCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPECO_PRESCALE` writer"]
pub struct W(crate::W<LPECO_PRESCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPECO_PRESCALE_SPEC>;
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
impl From<crate::W<LPECO_PRESCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPECO_PRESCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPECO_DIV_ENABLED` reader - LPECO prescaler enabled. HW sets this field to '1' as a result of an BACKUP_LPECO_CTL.LPECO_DIV_ENABLE. HW sets this field to '0' as a result of writing CLK_LPECO_CONFIG.LPECO_EN==0. This field does not update unless LPECO clock is toggling."]
pub type LPECO_DIV_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_FRAC_DIV` reader - Fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when LPECO Prescaler is enabled."]
pub type LPECO_FRAC_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPECO_FRAC_DIV` writer - Fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when LPECO Prescaler is enabled."]
pub type LPECO_FRAC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPECO_PRESCALE_SPEC, u8, u8, 8, O>;
#[doc = "Field `LPECO_INT_DIV` reader - Integer divide value allows for LPECO frequencies up to 8MHz to generate 32768 Hz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write LPECO_INT_DIV=0. Do not change this setting when LPECO Prescaler is enabled."]
pub type LPECO_INT_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LPECO_INT_DIV` writer - Integer divide value allows for LPECO frequencies up to 8MHz to generate 32768 Hz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write LPECO_INT_DIV=0. Do not change this setting when LPECO Prescaler is enabled."]
pub type LPECO_INT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPECO_PRESCALE_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - LPECO prescaler enabled. HW sets this field to '1' as a result of an BACKUP_LPECO_CTL.LPECO_DIV_ENABLE. HW sets this field to '0' as a result of writing CLK_LPECO_CONFIG.LPECO_EN==0. This field does not update unless LPECO clock is toggling."]
    #[inline(always)]
    pub fn lpeco_div_enabled(&self) -> LPECO_DIV_ENABLED_R {
        LPECO_DIV_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when LPECO Prescaler is enabled."]
    #[inline(always)]
    pub fn lpeco_frac_div(&self) -> LPECO_FRAC_DIV_R {
        LPECO_FRAC_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Integer divide value allows for LPECO frequencies up to 8MHz to generate 32768 Hz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write LPECO_INT_DIV=0. Do not change this setting when LPECO Prescaler is enabled."]
    #[inline(always)]
    pub fn lpeco_int_div(&self) -> LPECO_INT_DIV_R {
        LPECO_INT_DIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - Fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when LPECO Prescaler is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_frac_div(&mut self) -> LPECO_FRAC_DIV_W<8> {
        LPECO_FRAC_DIV_W::new(self)
    }
    #[doc = "Bits 16:25 - Integer divide value allows for LPECO frequencies up to 8MHz to generate 32768 Hz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write LPECO_INT_DIV=0. Do not change this setting when LPECO Prescaler is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_int_div(&mut self) -> LPECO_INT_DIV_W<16> {
        LPECO_INT_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low-power external crystal oscillator prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpeco_prescale](index.html) module"]
pub struct LPECO_PRESCALE_SPEC;
impl crate::RegisterSpec for LPECO_PRESCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpeco_prescale::R](R) reader structure"]
impl crate::Readable for LPECO_PRESCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpeco_prescale::W](W) writer structure"]
impl crate::Writable for LPECO_PRESCALE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPECO_PRESCALE to value 0"]
impl crate::Resettable for LPECO_PRESCALE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
