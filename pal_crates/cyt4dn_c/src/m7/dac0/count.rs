#[doc = "Register `COUNT` reader"]
pub struct R(crate::R<COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT` writer"]
pub struct W(crate::W<COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_SPEC>;
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
impl From<crate::W<COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT_1MS` reader - Count for 1ms with clk_timer. Specifies the number of counts to achieve 1ms based on clk_timer (1 count represent 1 clk_timer period). This field needs to be programmed before IF_CTL.DAC_EN=1."]
pub type COUNT_1MS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT_1MS` writer - Count for 1ms with clk_timer. Specifies the number of counts to achieve 1ms based on clk_timer (1 count represent 1 clk_timer period). This field needs to be programmed before IF_CTL.DAC_EN=1."]
pub type COUNT_1MS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `FAST_RAMP_COUNT_IN_MS` reader - Fast Ramp count in ms. Specifies the number of counts in ms for the fast ramp circuit. HW will enable the fast ramp circuit when counting is in progress. HW will disable the fast ramp circuit when counting is complete. E.g. FAST_RAMP_COUNT_IN_MS=10 means that HW enables the fast ramp circuit when it starts counting and disable the fast ramp circuit after 10ms. HW sets STATUS.FAST_RAMP_DONE=1 after counting is complete."]
pub type FAST_RAMP_COUNT_IN_MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAST_RAMP_COUNT_IN_MS` writer - Fast Ramp count in ms. Specifies the number of counts in ms for the fast ramp circuit. HW will enable the fast ramp circuit when counting is in progress. HW will disable the fast ramp circuit when counting is complete. E.g. FAST_RAMP_COUNT_IN_MS=10 means that HW enables the fast ramp circuit when it starts counting and disable the fast ramp circuit after 10ms. HW sets STATUS.FAST_RAMP_DONE=1 after counting is complete."]
pub type FAST_RAMP_COUNT_IN_MS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COUNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `COMP_RAMP_COUNT_IN_MS` reader - Complete Ramp count in ms. Specifies the number of counts in ms for the complete ramp circuit. The count takes place after the fast ramp count is complete. E.g. COMP_RAMP_COUNT_IN_MS=60 means that HW starts counting after disabling fast ramp circuit and stops counting after 60ms. HW sets STATUS.COMP_RAMP_DONE=1 after counting is complete."]
pub type COMP_RAMP_COUNT_IN_MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP_RAMP_COUNT_IN_MS` writer - Complete Ramp count in ms. Specifies the number of counts in ms for the complete ramp circuit. The count takes place after the fast ramp count is complete. E.g. COMP_RAMP_COUNT_IN_MS=60 means that HW starts counting after disabling fast ramp circuit and stops counting after 60ms. HW sets STATUS.COMP_RAMP_DONE=1 after counting is complete."]
pub type COMP_RAMP_COUNT_IN_MS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COUNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Count for 1ms with clk_timer. Specifies the number of counts to achieve 1ms based on clk_timer (1 count represent 1 clk_timer period). This field needs to be programmed before IF_CTL.DAC_EN=1."]
    #[inline(always)]
    pub fn count_1ms(&self) -> COUNT_1MS_R {
        COUNT_1MS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fast Ramp count in ms. Specifies the number of counts in ms for the fast ramp circuit. HW will enable the fast ramp circuit when counting is in progress. HW will disable the fast ramp circuit when counting is complete. E.g. FAST_RAMP_COUNT_IN_MS=10 means that HW enables the fast ramp circuit when it starts counting and disable the fast ramp circuit after 10ms. HW sets STATUS.FAST_RAMP_DONE=1 after counting is complete."]
    #[inline(always)]
    pub fn fast_ramp_count_in_ms(&self) -> FAST_RAMP_COUNT_IN_MS_R {
        FAST_RAMP_COUNT_IN_MS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Complete Ramp count in ms. Specifies the number of counts in ms for the complete ramp circuit. The count takes place after the fast ramp count is complete. E.g. COMP_RAMP_COUNT_IN_MS=60 means that HW starts counting after disabling fast ramp circuit and stops counting after 60ms. HW sets STATUS.COMP_RAMP_DONE=1 after counting is complete."]
    #[inline(always)]
    pub fn comp_ramp_count_in_ms(&self) -> COMP_RAMP_COUNT_IN_MS_R {
        COMP_RAMP_COUNT_IN_MS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for 1ms with clk_timer. Specifies the number of counts to achieve 1ms based on clk_timer (1 count represent 1 clk_timer period). This field needs to be programmed before IF_CTL.DAC_EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn count_1ms(&mut self) -> COUNT_1MS_W<0> {
        COUNT_1MS_W::new(self)
    }
    #[doc = "Bits 16:23 - Fast Ramp count in ms. Specifies the number of counts in ms for the fast ramp circuit. HW will enable the fast ramp circuit when counting is in progress. HW will disable the fast ramp circuit when counting is complete. E.g. FAST_RAMP_COUNT_IN_MS=10 means that HW enables the fast ramp circuit when it starts counting and disable the fast ramp circuit after 10ms. HW sets STATUS.FAST_RAMP_DONE=1 after counting is complete."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ramp_count_in_ms(&mut self) -> FAST_RAMP_COUNT_IN_MS_W<16> {
        FAST_RAMP_COUNT_IN_MS_W::new(self)
    }
    #[doc = "Bits 24:31 - Complete Ramp count in ms. Specifies the number of counts in ms for the complete ramp circuit. The count takes place after the fast ramp count is complete. E.g. COMP_RAMP_COUNT_IN_MS=60 means that HW starts counting after disabling fast ramp circuit and stops counting after 60ms. HW sets STATUS.COMP_RAMP_DONE=1 after counting is complete."]
    #[inline(always)]
    #[must_use]
    pub fn comp_ramp_count_in_ms(&mut self) -> COMP_RAMP_COUNT_IN_MS_W<24> {
        COMP_RAMP_COUNT_IN_MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](index.html) module"]
pub struct COUNT_SPEC;
impl crate::RegisterSpec for COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count::R](R) reader structure"]
impl crate::Readable for COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count::W](W) writer structure"]
impl crate::Writable for COUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT to value 0x3c0a_1f40"]
impl crate::Resettable for COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c0a_1f40;
}
