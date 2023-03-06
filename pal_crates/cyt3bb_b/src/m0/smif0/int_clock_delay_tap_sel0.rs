#[doc = "Register `INT_CLOCK_DELAY_TAP_SEL0` reader"]
pub struct R(crate::R<INT_CLOCK_DELAY_TAP_SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CLOCK_DELAY_TAP_SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CLOCK_DELAY_TAP_SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CLOCK_DELAY_TAP_SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CLOCK_DELAY_TAP_SEL0` writer"]
pub struct W(crate::W<INT_CLOCK_DELAY_TAP_SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLOCK_DELAY_TAP_SEL0_SPEC>;
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
impl From<crate::W<INT_CLOCK_DELAY_TAP_SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLOCK_DELAY_TAP_SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BIT0` reader - Delay line tap selection for data bit 0. A value of 0 selects the receive capture clock source specified by CTL.CLOCK_IF_RX_SEL. All other values select a tap of the clock delay line. When the data learning pattern is enabled this value is set by HW."]
pub type DATA_BIT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT0` writer - Delay line tap selection for data bit 0. A value of 0 selects the receive capture clock source specified by CTL.CLOCK_IF_RX_SEL. All other values select a tap of the clock delay line. When the data learning pattern is enabled this value is set by HW."]
pub type DATA_BIT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT1` reader - Delay line tap selection for data bit 1. See DATA_BIT0."]
pub type DATA_BIT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT1` writer - Delay line tap selection for data bit 1. See DATA_BIT0."]
pub type DATA_BIT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT2` reader - Delay line tap selection for data bit 2. See DATA_BIT0."]
pub type DATA_BIT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT2` writer - Delay line tap selection for data bit 2. See DATA_BIT0."]
pub type DATA_BIT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT3` reader - Delay line tap selection for data bit 3. See DATA_BIT0."]
pub type DATA_BIT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT3` writer - Delay line tap selection for data bit 3. See DATA_BIT0."]
pub type DATA_BIT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Delay line tap selection for data bit 0. A value of 0 selects the receive capture clock source specified by CTL.CLOCK_IF_RX_SEL. All other values select a tap of the clock delay line. When the data learning pattern is enabled this value is set by HW."]
    #[inline(always)]
    pub fn data_bit0(&self) -> DATA_BIT0_R {
        DATA_BIT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for data bit 1. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit1(&self) -> DATA_BIT1_R {
        DATA_BIT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for data bit 2. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit2(&self) -> DATA_BIT2_R {
        DATA_BIT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for data bit 3. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit3(&self) -> DATA_BIT3_R {
        DATA_BIT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay line tap selection for data bit 0. A value of 0 selects the receive capture clock source specified by CTL.CLOCK_IF_RX_SEL. All other values select a tap of the clock delay line. When the data learning pattern is enabled this value is set by HW."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit0(&mut self) -> DATA_BIT0_W<0> {
        DATA_BIT0_W::new(self)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for data bit 1. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit1(&mut self) -> DATA_BIT1_W<8> {
        DATA_BIT1_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for data bit 2. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit2(&mut self) -> DATA_BIT2_W<16> {
        DATA_BIT2_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for data bit 3. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit3(&mut self) -> DATA_BIT3_W<24> {
        DATA_BIT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Clocking Delay Tap Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clock_delay_tap_sel0](index.html) module"]
pub struct INT_CLOCK_DELAY_TAP_SEL0_SPEC;
impl crate::RegisterSpec for INT_CLOCK_DELAY_TAP_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_clock_delay_tap_sel0::R](R) reader structure"]
impl crate::Readable for INT_CLOCK_DELAY_TAP_SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_clock_delay_tap_sel0::W](W) writer structure"]
impl crate::Writable for INT_CLOCK_DELAY_TAP_SEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLOCK_DELAY_TAP_SEL0 to value 0"]
impl crate::Resettable for INT_CLOCK_DELAY_TAP_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
