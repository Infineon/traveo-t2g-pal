#[doc = "Register `PWR_LVD_CTL2` reader"]
pub struct R(crate::R<PWR_LVD_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_LVD_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_LVD_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_LVD_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_LVD_CTL2` writer"]
pub struct W(crate::W<PWR_LVD_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_LVD_CTL2_SPEC>;
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
impl From<crate::W<PWR_LVD_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_LVD_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVLVD2_TRIPSEL_HT` reader - N/A"]
pub type HVLVD2_TRIPSEL_HT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLVD2_TRIPSEL_HT` writer - N/A"]
pub type HVLVD2_TRIPSEL_HT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_LVD_CTL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `HVLVD2_DPSLP_EN_HT` reader - Keep HVLVD2 voltage monitor enabled during DEEPSLEEP mode. This field is only used when HVLVD1_EN_HT==1."]
pub type HVLVD2_DPSLP_EN_HT_R = crate::BitReader<bool>;
#[doc = "Field `HVLVD2_DPSLP_EN_HT` writer - Keep HVLVD2 voltage monitor enabled during DEEPSLEEP mode. This field is only used when HVLVD1_EN_HT==1."]
pub type HVLVD2_DPSLP_EN_HT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_LVD_CTL2_SPEC, bool, O>;
#[doc = "Field `HVLVD2_EN_HT` reader - Enable HVLVD2 voltage monitor. This detector monitors vddd only. Do not change other HVLVD2 settings when enabled."]
pub type HVLVD2_EN_HT_R = crate::BitReader<bool>;
#[doc = "Field `HVLVD2_EN_HT` writer - Enable HVLVD2 voltage monitor. This detector monitors vddd only. Do not change other HVLVD2 settings when enabled."]
pub type HVLVD2_EN_HT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_LVD_CTL2_SPEC, bool, O>;
#[doc = "Field `HVLVD2_EDGE_SEL` reader - Sets which edge(s) will trigger an action when the threshold is crossed."]
pub type HVLVD2_EDGE_SEL_R = crate::FieldReader<u8, HVLVD2_EDGE_SEL_A>;
#[doc = "Sets which edge(s) will trigger an action when the threshold is crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HVLVD2_EDGE_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<HVLVD2_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVLVD2_EDGE_SEL_A) -> Self {
        variant as _
    }
}
impl HVLVD2_EDGE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVLVD2_EDGE_SEL_A {
        match self.bits {
            0 => HVLVD2_EDGE_SEL_A::DISABLE,
            1 => HVLVD2_EDGE_SEL_A::RISING,
            2 => HVLVD2_EDGE_SEL_A::FALLING,
            3 => HVLVD2_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVLVD2_EDGE_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == HVLVD2_EDGE_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == HVLVD2_EDGE_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == HVLVD2_EDGE_SEL_A::BOTH
    }
}
#[doc = "Field `HVLVD2_EDGE_SEL` writer - Sets which edge(s) will trigger an action when the threshold is crossed."]
pub type HVLVD2_EDGE_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PWR_LVD_CTL2_SPEC, u8, HVLVD2_EDGE_SEL_A, 2, O>;
impl<'a, const O: u8> HVLVD2_EDGE_SEL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVLVD2_EDGE_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(HVLVD2_EDGE_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(HVLVD2_EDGE_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(HVLVD2_EDGE_SEL_A::BOTH)
    }
}
#[doc = "Field `HVLVD2_ACTION` reader - Action taken when the threshold is crossed in the programmed directions(s)"]
pub type HVLVD2_ACTION_R = crate::BitReader<HVLVD2_ACTION_A>;
#[doc = "Action taken when the threshold is crossed in the programmed directions(s)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVLVD2_ACTION_A {
    #[doc = "0: Generate an interrupt"]
    INTERRUPT = 0,
    #[doc = "1: Generate a fault"]
    FAULT = 1,
}
impl From<HVLVD2_ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: HVLVD2_ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl HVLVD2_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVLVD2_ACTION_A {
        match self.bits {
            false => HVLVD2_ACTION_A::INTERRUPT,
            true => HVLVD2_ACTION_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HVLVD2_ACTION_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == HVLVD2_ACTION_A::FAULT
    }
}
#[doc = "Field `HVLVD2_ACTION` writer - Action taken when the threshold is crossed in the programmed directions(s)"]
pub type HVLVD2_ACTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_LVD_CTL2_SPEC, HVLVD2_ACTION_A, O>;
impl<'a, const O: u8> HVLVD2_ACTION_W<'a, O> {
    #[doc = "Generate an interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(HVLVD2_ACTION_A::INTERRUPT)
    }
    #[doc = "Generate a fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(HVLVD2_ACTION_A::FAULT)
    }
}
impl R {
    #[doc = "Bits 8:12 - N/A"]
    #[inline(always)]
    pub fn hvlvd2_tripsel_ht(&self) -> HVLVD2_TRIPSEL_HT_R {
        HVLVD2_TRIPSEL_HT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Keep HVLVD2 voltage monitor enabled during DEEPSLEEP mode. This field is only used when HVLVD1_EN_HT==1."]
    #[inline(always)]
    pub fn hvlvd2_dpslp_en_ht(&self) -> HVLVD2_DPSLP_EN_HT_R {
        HVLVD2_DPSLP_EN_HT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable HVLVD2 voltage monitor. This detector monitors vddd only. Do not change other HVLVD2 settings when enabled."]
    #[inline(always)]
    pub fn hvlvd2_en_ht(&self) -> HVLVD2_EN_HT_R {
        HVLVD2_EN_HT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets which edge(s) will trigger an action when the threshold is crossed."]
    #[inline(always)]
    pub fn hvlvd2_edge_sel(&self) -> HVLVD2_EDGE_SEL_R {
        HVLVD2_EDGE_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Action taken when the threshold is crossed in the programmed directions(s)"]
    #[inline(always)]
    pub fn hvlvd2_action(&self) -> HVLVD2_ACTION_R {
        HVLVD2_ACTION_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd2_tripsel_ht(&mut self) -> HVLVD2_TRIPSEL_HT_W<8> {
        HVLVD2_TRIPSEL_HT_W::new(self)
    }
    #[doc = "Bit 14 - Keep HVLVD2 voltage monitor enabled during DEEPSLEEP mode. This field is only used when HVLVD1_EN_HT==1."]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd2_dpslp_en_ht(&mut self) -> HVLVD2_DPSLP_EN_HT_W<14> {
        HVLVD2_DPSLP_EN_HT_W::new(self)
    }
    #[doc = "Bit 15 - Enable HVLVD2 voltage monitor. This detector monitors vddd only. Do not change other HVLVD2 settings when enabled."]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd2_en_ht(&mut self) -> HVLVD2_EN_HT_W<15> {
        HVLVD2_EN_HT_W::new(self)
    }
    #[doc = "Bits 16:17 - Sets which edge(s) will trigger an action when the threshold is crossed."]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd2_edge_sel(&mut self) -> HVLVD2_EDGE_SEL_W<16> {
        HVLVD2_EDGE_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Action taken when the threshold is crossed in the programmed directions(s)"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd2_action(&mut self) -> HVLVD2_ACTION_W<18> {
        HVLVD2_ACTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Configuration Register #2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_ctl2](index.html) module"]
pub struct PWR_LVD_CTL2_SPEC;
impl crate::RegisterSpec for PWR_LVD_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_lvd_ctl2::R](R) reader structure"]
impl crate::Readable for PWR_LVD_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_lvd_ctl2::W](W) writer structure"]
impl crate::Writable for PWR_LVD_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_LVD_CTL2 to value 0"]
impl crate::Resettable for PWR_LVD_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
