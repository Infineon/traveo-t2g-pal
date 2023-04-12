#[doc = "Register `INT_CLOCK_DELAY_TAP_SEL1` reader"]
pub struct R(crate::R<INT_CLOCK_DELAY_TAP_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CLOCK_DELAY_TAP_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CLOCK_DELAY_TAP_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CLOCK_DELAY_TAP_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CLOCK_DELAY_TAP_SEL1` writer"]
pub struct W(crate::W<INT_CLOCK_DELAY_TAP_SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLOCK_DELAY_TAP_SEL1_SPEC>;
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
impl From<crate::W<INT_CLOCK_DELAY_TAP_SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLOCK_DELAY_TAP_SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BIT4` reader - Delay line tap selection for data bit 4. See DATA_BIT0."]
pub type DATA_BIT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT4` writer - Delay line tap selection for data bit 4. See DATA_BIT0."]
pub type DATA_BIT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT5` reader - Delay line tap selection for data bit 5. See DATA_BIT0."]
pub type DATA_BIT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT5` writer - Delay line tap selection for data bit 5. See DATA_BIT0."]
pub type DATA_BIT5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT6` reader - Delay line tap selection for data bit 6. See DATA_BIT0."]
pub type DATA_BIT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT6` writer - Delay line tap selection for data bit 6. See DATA_BIT0."]
pub type DATA_BIT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA_BIT7` reader - Delay line tap selection for data bit 7. See DATA_BIT0."]
pub type DATA_BIT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT7` writer - Delay line tap selection for data bit 7. See DATA_BIT0."]
pub type DATA_BIT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CLOCK_DELAY_TAP_SEL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Delay line tap selection for data bit 4. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit4(&self) -> DATA_BIT4_R {
        DATA_BIT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for data bit 5. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit5(&self) -> DATA_BIT5_R {
        DATA_BIT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for data bit 6. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit6(&self) -> DATA_BIT6_R {
        DATA_BIT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for data bit 7. See DATA_BIT0."]
    #[inline(always)]
    pub fn data_bit7(&self) -> DATA_BIT7_R {
        DATA_BIT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay line tap selection for data bit 4. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit4(&mut self) -> DATA_BIT4_W<0> {
        DATA_BIT4_W::new(self)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for data bit 5. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit5(&mut self) -> DATA_BIT5_W<8> {
        DATA_BIT5_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for data bit 6. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit6(&mut self) -> DATA_BIT6_W<16> {
        DATA_BIT6_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for data bit 7. See DATA_BIT0."]
    #[inline(always)]
    #[must_use]
    pub fn data_bit7(&mut self) -> DATA_BIT7_W<24> {
        DATA_BIT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Clocking Delay Tap Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clock_delay_tap_sel1](index.html) module"]
pub struct INT_CLOCK_DELAY_TAP_SEL1_SPEC;
impl crate::RegisterSpec for INT_CLOCK_DELAY_TAP_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_clock_delay_tap_sel1::R](R) reader structure"]
impl crate::Readable for INT_CLOCK_DELAY_TAP_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_clock_delay_tap_sel1::W](W) writer structure"]
impl crate::Writable for INT_CLOCK_DELAY_TAP_SEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLOCK_DELAY_TAP_SEL1 to value 0"]
impl crate::Resettable for INT_CLOCK_DELAY_TAP_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
