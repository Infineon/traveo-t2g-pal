#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_LOW1` reader - Low count for logic 1. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '1'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks. .. 399: means 400 clocks. Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
pub type T_LOW1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_LOW1` writer - Low count for logic 1. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '1'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks. .. 399: means 400 clocks. Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
pub type T_LOW1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 9, O>;
#[doc = "Field `T_LOW0` reader - Low count for logic 0. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '0'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks .. 399: means 400 clocks Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
pub type T_LOW0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_LOW0` writer - Low count for logic 0. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '0'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks .. 399: means 400 clocks Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
pub type T_LOW0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 9, O>;
#[doc = "Field `T_OFFSET` reader - The value of offset that is used for sampling the 'rx'. The value of this counter is used in HW as below. - 0 : means 1 clock after detecting falling edge of 'rx' - 1 : means 2 clocks after detecting falling edge of 'rx' .. - 7 : means 8 clocks after detecting falling edge of 'rx' .. - 15 : means 16 clocks after detecting falling edge of 'rx' .. - 399 : means 400 clocks after detecting falling edge of 'rx' Any value above 399 is invalid."]
pub type T_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_OFFSET` writer - The value of offset that is used for sampling the 'rx'. The value of this counter is used in HW as below. - 0 : means 1 clock after detecting falling edge of 'rx' - 1 : means 2 clocks after detecting falling edge of 'rx' .. - 7 : means 8 clocks after detecting falling edge of 'rx' .. - 15 : means 16 clocks after detecting falling edge of 'rx' .. - 399 : means 400 clocks after detecting falling edge of 'rx' Any value above 399 is invalid."]
pub type T_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Low count for logic 1. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '1'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks. .. 399: means 400 clocks. Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
    #[inline(always)]
    pub fn t_low1(&self) -> T_LOW1_R {
        T_LOW1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - Low count for logic 0. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '0'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks .. 399: means 400 clocks Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
    #[inline(always)]
    pub fn t_low0(&self) -> T_LOW0_R {
        T_LOW0_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:30 - The value of offset that is used for sampling the 'rx'. The value of this counter is used in HW as below. - 0 : means 1 clock after detecting falling edge of 'rx' - 1 : means 2 clocks after detecting falling edge of 'rx' .. - 7 : means 8 clocks after detecting falling edge of 'rx' .. - 15 : means 16 clocks after detecting falling edge of 'rx' .. - 399 : means 400 clocks after detecting falling edge of 'rx' Any value above 399 is invalid."]
    #[inline(always)]
    pub fn t_offset(&self) -> T_OFFSET_R {
        T_OFFSET_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Low count for logic 1. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '1'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks. .. 399: means 400 clocks. Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
    #[inline(always)]
    #[must_use]
    pub fn t_low1(&mut self) -> T_LOW1_W<0> {
        T_LOW1_W::new(self)
    }
    #[doc = "Bits 12:20 - Low count for logic 0. This is valid only for PWM mode. The count value here indicates the number of clocks per clk_cxpi_ch to drive a '0' at CXPI bus before releasing it to indicate a logical '0'. 0: means 1 clock. 1: means 2 clocks .. 15: means 16 clocks .. 399: means 400 clocks Any value above 399 is invalid. Note that for NRZ mode, this field is ignored. Note that this field is used for TX."]
    #[inline(always)]
    #[must_use]
    pub fn t_low0(&mut self) -> T_LOW0_W<12> {
        T_LOW0_W::new(self)
    }
    #[doc = "Bits 22:30 - The value of offset that is used for sampling the 'rx'. The value of this counter is used in HW as below. - 0 : means 1 clock after detecting falling edge of 'rx' - 1 : means 2 clocks after detecting falling edge of 'rx' .. - 7 : means 8 clocks after detecting falling edge of 'rx' .. - 15 : means 16 clocks after detecting falling edge of 'rx' .. - 399 : means 400 clocks after detecting falling edge of 'rx' Any value above 399 is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn t_offset(&mut self) -> T_OFFSET_W<22> {
        T_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
