#[doc = "Register `PERIOD_BUFF` reader"]
pub struct R(crate::R<PERIOD_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIOD_BUFF` writer"]
pub struct W(crate::W<PERIOD_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_BUFF_SPEC>;
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
impl From<crate::W<PERIOD_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
pub type PERIOD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERIOD` writer - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERIOD_BUFF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register. In PWM_PR mode PEROD_BUFF defines the LFSR polynomial. Each bit represents a tap of the shift register which can be feed back to the MSB via an XOR tree. Examples for GRP_CNT_WIDTH = 16: - Maximum length 16bit LFSR - polynomial x^16 + x^14 + x^13 + x^11 + 1 - taps 0,2,3,5 -> PERIOD = 0x002d - period is 2^16-1 = 65535 cycles - Maximum length 8bit LFSR: - polynomial x^8 + x^6 + x^5 + x^4 + 1 - taps 8,10,11,12 (realized in 8 MSBs of 16bit LFSR) - period is 2^8-1 = 255 cycles In SR mode PERIOD_BUFF defines which tap of the shift register generates the PWM output signals. For a delay of n cycles (from capture event to PWM output) the bit CNT_WIDTH-n should be set to '1'. For a shift register function only one tap should be use, i.e. a one-hot value must be written to PERIOD_BUFF. If multiple bits in PERIOD_BUFF are set then the taps are XOR combined."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter buffered period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_buff](index.html) module"]
pub struct PERIOD_BUFF_SPEC;
impl crate::RegisterSpec for PERIOD_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period_buff::R](R) reader structure"]
impl crate::Readable for PERIOD_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period_buff::W](W) writer structure"]
impl crate::Writable for PERIOD_BUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIOD_BUFF to value 0xffff_ffff"]
impl crate::Resettable for PERIOD_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
