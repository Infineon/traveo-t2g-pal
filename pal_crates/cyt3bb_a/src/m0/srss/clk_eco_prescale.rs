#[doc = "Register `CLK_ECO_PRESCALE` reader"]
pub struct R(crate::R<CLK_ECO_PRESCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ECO_PRESCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ECO_PRESCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ECO_PRESCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ECO_PRESCALE` writer"]
pub struct W(crate::W<CLK_ECO_PRESCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ECO_PRESCALE_SPEC>;
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
impl From<crate::W<CLK_ECO_PRESCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ECO_PRESCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECO_DIV_ENABLED` reader - ECO prescaler enabled. HW sets this field to '1' as a result of an CLK_ECO_CONFIG.ECO_DIV_ENABLE command. HW sets this field to '0' as a result on a CLK_ECO_CONFIG.ECO_DIV_DISABLE command."]
pub type ECO_DIV_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ECO_FRAC_DIV` reader - 8-bit fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when ECO Prescaler is enabled."]
pub type ECO_FRAC_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECO_FRAC_DIV` writer - 8-bit fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when ECO Prescaler is enabled."]
pub type ECO_FRAC_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_ECO_PRESCALE_SPEC, u8, u8, 8, O>;
#[doc = "Field `ECO_INT_DIV` reader - 10-bit integer value allows for ECO frequencies up to 33.55MHz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write ECO_INT_DIV=0. Do not change this setting when ECO Prescaler is enabled."]
pub type ECO_INT_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ECO_INT_DIV` writer - 10-bit integer value allows for ECO frequencies up to 33.55MHz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write ECO_INT_DIV=0. Do not change this setting when ECO Prescaler is enabled."]
pub type ECO_INT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_ECO_PRESCALE_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - ECO prescaler enabled. HW sets this field to '1' as a result of an CLK_ECO_CONFIG.ECO_DIV_ENABLE command. HW sets this field to '0' as a result on a CLK_ECO_CONFIG.ECO_DIV_DISABLE command."]
    #[inline(always)]
    pub fn eco_div_enabled(&self) -> ECO_DIV_ENABLED_R {
        ECO_DIV_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 8-bit fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when ECO Prescaler is enabled."]
    #[inline(always)]
    pub fn eco_frac_div(&self) -> ECO_FRAC_DIV_R {
        ECO_FRAC_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - 10-bit integer value allows for ECO frequencies up to 33.55MHz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write ECO_INT_DIV=0. Do not change this setting when ECO Prescaler is enabled."]
    #[inline(always)]
    pub fn eco_int_div(&self) -> ECO_INT_DIV_R {
        ECO_INT_DIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - 8-bit fractional value, sufficient to get prescaler output within the +/-65ppm calibration range. Do not change this setting when ECO Prescaler is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn eco_frac_div(&mut self) -> ECO_FRAC_DIV_W<8> {
        ECO_FRAC_DIV_W::new(self)
    }
    #[doc = "Bits 16:25 - 10-bit integer value allows for ECO frequencies up to 33.55MHz. Subtract one from the desired divide value when writing this field. For example, to divide by 1, write ECO_INT_DIV=0. Do not change this setting when ECO Prescaler is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn eco_int_div(&mut self) -> ECO_INT_DIV_W<16> {
        ECO_INT_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECO Prescaler Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_eco_prescale](index.html) module"]
pub struct CLK_ECO_PRESCALE_SPEC;
impl crate::RegisterSpec for CLK_ECO_PRESCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_eco_prescale::R](R) reader structure"]
impl crate::Readable for CLK_ECO_PRESCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_eco_prescale::W](W) writer structure"]
impl crate::Writable for CLK_ECO_PRESCALE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ECO_PRESCALE to value 0"]
impl crate::Resettable for CLK_ECO_PRESCALE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
