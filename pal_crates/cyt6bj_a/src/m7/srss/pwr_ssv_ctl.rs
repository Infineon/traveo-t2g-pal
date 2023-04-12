#[doc = "Register `PWR_SSV_CTL` reader"]
pub struct R(crate::R<PWR_SSV_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SSV_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SSV_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SSV_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_SSV_CTL` writer"]
pub struct W(crate::W<PWR_SSV_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SSV_CTL_SPEC>;
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
impl From<crate::W<PWR_SSV_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SSV_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODVDDD_VSEL` reader - Selects the voltage threshold for BOD on vddd. The BOD does not reliably monitor the supply during the transition. 0: vddd&lt;2.7V 1: vddd&lt;3.0V"]
pub type BODVDDD_VSEL_R = crate::BitReader<bool>;
#[doc = "Field `BODVDDD_VSEL` writer - Selects the voltage threshold for BOD on vddd. The BOD does not reliably monitor the supply during the transition. 0: vddd&lt;2.7V 1: vddd&lt;3.0V"]
pub type BODVDDD_VSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `BODVDDD_ENABLE` reader - Enable for BOD on vddd. This cannot be disabled during normal operation."]
pub type BODVDDD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BODVDDD_ENABLE` writer - Enable for BOD on vddd. This cannot be disabled during normal operation."]
pub type BODVDDD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `BODVDDA_VSEL` reader - Selects the voltage threshold for BOD on vdda. Ensure BODVDDA_ENABLE==0 before changing this setting to prevent false triggers. 0: vdda&lt;2.7V 1: vdda&lt;3.0V"]
pub type BODVDDA_VSEL_R = crate::BitReader<bool>;
#[doc = "Field `BODVDDA_VSEL` writer - Selects the voltage threshold for BOD on vdda. Ensure BODVDDA_ENABLE==0 before changing this setting to prevent false triggers. 0: vdda&lt;2.7V 1: vdda&lt;3.0V"]
pub type BODVDDA_VSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `BODVDDA_ACTION` reader - Action taken when the BOD on vdda triggers."]
pub type BODVDDA_ACTION_R = crate::FieldReader<u8, BODVDDA_ACTION_A>;
#[doc = "Action taken when the BOD on vdda triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODVDDA_ACTION_A {
    #[doc = "0: No action"]
    NOTHING = 0,
    #[doc = "1: Generate a fault"]
    FAULT = 1,
    #[doc = "2: Reset the chip"]
    RESET = 2,
}
impl From<BODVDDA_ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: BODVDDA_ACTION_A) -> Self {
        variant as _
    }
}
impl BODVDDA_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODVDDA_ACTION_A> {
        match self.bits {
            0 => Some(BODVDDA_ACTION_A::NOTHING),
            1 => Some(BODVDDA_ACTION_A::FAULT),
            2 => Some(BODVDDA_ACTION_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == BODVDDA_ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == BODVDDA_ACTION_A::FAULT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BODVDDA_ACTION_A::RESET
    }
}
#[doc = "Field `BODVDDA_ACTION` writer - Action taken when the BOD on vdda triggers."]
pub type BODVDDA_ACTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_SSV_CTL_SPEC, u8, BODVDDA_ACTION_A, 2, O>;
impl<'a, const O: u8> BODVDDA_ACTION_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(BODVDDA_ACTION_A::NOTHING)
    }
    #[doc = "Generate a fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(BODVDDA_ACTION_A::FAULT)
    }
    #[doc = "Reset the chip"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BODVDDA_ACTION_A::RESET)
    }
}
#[doc = "Field `BODVDDA_ENABLE` reader - Enable for BOD on vdda. BODVDDA_ACTION will be triggered when the BOD is disabled. If no action is desired when disabling, firmware must first write BODVDDA_ACTION=NOTHING in a separate write cycle."]
pub type BODVDDA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BODVDDA_ENABLE` writer - Enable for BOD on vdda. BODVDDA_ACTION will be triggered when the BOD is disabled. If no action is desired when disabling, firmware must first write BODVDDA_ACTION=NOTHING in a separate write cycle."]
pub type BODVDDA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `BODVCCD_ENABLE` reader - Enable for BOD on vccd. This cannot be disabled during normal operation."]
pub type BODVCCD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BODVCCD_ENABLE` writer - Enable for BOD on vccd. This cannot be disabled during normal operation."]
pub type BODVCCD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `OVDVDDD_VSEL` reader - Selects the voltage threshold for OVD on vddd. The OVD does not reliably monitor the supply during the transition. 0: vddd>5.5V 1: vddd>5.0V"]
pub type OVDVDDD_VSEL_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDD_VSEL` writer - Selects the voltage threshold for OVD on vddd. The OVD does not reliably monitor the supply during the transition. 0: vddd>5.5V 1: vddd>5.0V"]
pub type OVDVDDD_VSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `OVDVDDD_ENABLE` reader - Enable for OVD on vddd. This cannot be disabled during normal operation."]
pub type OVDVDDD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDD_ENABLE` writer - Enable for OVD on vddd. This cannot be disabled during normal operation."]
pub type OVDVDDD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `OVDVDDA_VSEL` reader - Selects the voltage threshold for OVD on vdda. Ensure OVDVDDA_ENABLE==0 before changing this setting to prevent false triggers 0: vddd>5.5V 1: vddd>5.0V"]
pub type OVDVDDA_VSEL_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDA_VSEL` writer - Selects the voltage threshold for OVD on vdda. Ensure OVDVDDA_ENABLE==0 before changing this setting to prevent false triggers 0: vddd>5.5V 1: vddd>5.0V"]
pub type OVDVDDA_VSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `OVDVDDA_ACTION` reader - Action taken when the OVD on vdda triggers."]
pub type OVDVDDA_ACTION_R = crate::FieldReader<u8, OVDVDDA_ACTION_A>;
#[doc = "Action taken when the OVD on vdda triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVDVDDA_ACTION_A {
    #[doc = "0: No action"]
    NOTHING = 0,
    #[doc = "1: Generate a fault"]
    FAULT = 1,
    #[doc = "2: Reset the chip"]
    RESET = 2,
}
impl From<OVDVDDA_ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: OVDVDDA_ACTION_A) -> Self {
        variant as _
    }
}
impl OVDVDDA_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVDVDDA_ACTION_A> {
        match self.bits {
            0 => Some(OVDVDDA_ACTION_A::NOTHING),
            1 => Some(OVDVDDA_ACTION_A::FAULT),
            2 => Some(OVDVDDA_ACTION_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == OVDVDDA_ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == OVDVDDA_ACTION_A::FAULT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OVDVDDA_ACTION_A::RESET
    }
}
#[doc = "Field `OVDVDDA_ACTION` writer - Action taken when the OVD on vdda triggers."]
pub type OVDVDDA_ACTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_SSV_CTL_SPEC, u8, OVDVDDA_ACTION_A, 2, O>;
impl<'a, const O: u8> OVDVDDA_ACTION_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(OVDVDDA_ACTION_A::NOTHING)
    }
    #[doc = "Generate a fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(OVDVDDA_ACTION_A::FAULT)
    }
    #[doc = "Reset the chip"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OVDVDDA_ACTION_A::RESET)
    }
}
#[doc = "Field `OVDVDDA_ENABLE` reader - Enable for OVD on vdda."]
pub type OVDVDDA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDA_ENABLE` writer - Enable for OVD on vdda."]
pub type OVDVDDA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
#[doc = "Field `OVDVCCD_ENABLE` reader - Enable for OVD on vccd. This cannot be disabled during normal operation."]
pub type OVDVCCD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `OVDVCCD_ENABLE` writer - Enable for OVD on vccd. This cannot be disabled during normal operation."]
pub type OVDVCCD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SSV_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects the voltage threshold for BOD on vddd. The BOD does not reliably monitor the supply during the transition. 0: vddd&lt;2.7V 1: vddd&lt;3.0V"]
    #[inline(always)]
    pub fn bodvddd_vsel(&self) -> BODVDDD_VSEL_R {
        BODVDDD_VSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for BOD on vddd. This cannot be disabled during normal operation."]
    #[inline(always)]
    pub fn bodvddd_enable(&self) -> BODVDDD_ENABLE_R {
        BODVDDD_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the voltage threshold for BOD on vdda. Ensure BODVDDA_ENABLE==0 before changing this setting to prevent false triggers. 0: vdda&lt;2.7V 1: vdda&lt;3.0V"]
    #[inline(always)]
    pub fn bodvdda_vsel(&self) -> BODVDDA_VSEL_R {
        BODVDDA_VSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Action taken when the BOD on vdda triggers."]
    #[inline(always)]
    pub fn bodvdda_action(&self) -> BODVDDA_ACTION_R {
        BODVDDA_ACTION_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable for BOD on vdda. BODVDDA_ACTION will be triggered when the BOD is disabled. If no action is desired when disabling, firmware must first write BODVDDA_ACTION=NOTHING in a separate write cycle."]
    #[inline(always)]
    pub fn bodvdda_enable(&self) -> BODVDDA_ENABLE_R {
        BODVDDA_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable for BOD on vccd. This cannot be disabled during normal operation."]
    #[inline(always)]
    pub fn bodvccd_enable(&self) -> BODVCCD_ENABLE_R {
        BODVCCD_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects the voltage threshold for OVD on vddd. The OVD does not reliably monitor the supply during the transition. 0: vddd>5.5V 1: vddd>5.0V"]
    #[inline(always)]
    pub fn ovdvddd_vsel(&self) -> OVDVDDD_VSEL_R {
        OVDVDDD_VSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable for OVD on vddd. This cannot be disabled during normal operation."]
    #[inline(always)]
    pub fn ovdvddd_enable(&self) -> OVDVDDD_ENABLE_R {
        OVDVDDD_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects the voltage threshold for OVD on vdda. Ensure OVDVDDA_ENABLE==0 before changing this setting to prevent false triggers 0: vddd>5.5V 1: vddd>5.0V"]
    #[inline(always)]
    pub fn ovdvdda_vsel(&self) -> OVDVDDA_VSEL_R {
        OVDVDDA_VSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Action taken when the OVD on vdda triggers."]
    #[inline(always)]
    pub fn ovdvdda_action(&self) -> OVDVDDA_ACTION_R {
        OVDVDDA_ACTION_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable for OVD on vdda."]
    #[inline(always)]
    pub fn ovdvdda_enable(&self) -> OVDVDDA_ENABLE_R {
        OVDVDDA_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable for OVD on vccd. This cannot be disabled during normal operation."]
    #[inline(always)]
    pub fn ovdvccd_enable(&self) -> OVDVCCD_ENABLE_R {
        OVDVCCD_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the voltage threshold for BOD on vddd. The BOD does not reliably monitor the supply during the transition. 0: vddd&lt;2.7V 1: vddd&lt;3.0V"]
    #[inline(always)]
    #[must_use]
    pub fn bodvddd_vsel(&mut self) -> BODVDDD_VSEL_W<0> {
        BODVDDD_VSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable for BOD on vddd. This cannot be disabled during normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn bodvddd_enable(&mut self) -> BODVDDD_ENABLE_W<3> {
        BODVDDD_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Selects the voltage threshold for BOD on vdda. Ensure BODVDDA_ENABLE==0 before changing this setting to prevent false triggers. 0: vdda&lt;2.7V 1: vdda&lt;3.0V"]
    #[inline(always)]
    #[must_use]
    pub fn bodvdda_vsel(&mut self) -> BODVDDA_VSEL_W<4> {
        BODVDDA_VSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Action taken when the BOD on vdda triggers."]
    #[inline(always)]
    #[must_use]
    pub fn bodvdda_action(&mut self) -> BODVDDA_ACTION_W<6> {
        BODVDDA_ACTION_W::new(self)
    }
    #[doc = "Bit 8 - Enable for BOD on vdda. BODVDDA_ACTION will be triggered when the BOD is disabled. If no action is desired when disabling, firmware must first write BODVDDA_ACTION=NOTHING in a separate write cycle."]
    #[inline(always)]
    #[must_use]
    pub fn bodvdda_enable(&mut self) -> BODVDDA_ENABLE_W<8> {
        BODVDDA_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - Enable for BOD on vccd. This cannot be disabled during normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn bodvccd_enable(&mut self) -> BODVCCD_ENABLE_W<11> {
        BODVCCD_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Selects the voltage threshold for OVD on vddd. The OVD does not reliably monitor the supply during the transition. 0: vddd>5.5V 1: vddd>5.0V"]
    #[inline(always)]
    #[must_use]
    pub fn ovdvddd_vsel(&mut self) -> OVDVDDD_VSEL_W<16> {
        OVDVDDD_VSEL_W::new(self)
    }
    #[doc = "Bit 19 - Enable for OVD on vddd. This cannot be disabled during normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn ovdvddd_enable(&mut self) -> OVDVDDD_ENABLE_W<19> {
        OVDVDDD_ENABLE_W::new(self)
    }
    #[doc = "Bit 20 - Selects the voltage threshold for OVD on vdda. Ensure OVDVDDA_ENABLE==0 before changing this setting to prevent false triggers 0: vddd>5.5V 1: vddd>5.0V"]
    #[inline(always)]
    #[must_use]
    pub fn ovdvdda_vsel(&mut self) -> OVDVDDA_VSEL_W<20> {
        OVDVDDA_VSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Action taken when the OVD on vdda triggers."]
    #[inline(always)]
    #[must_use]
    pub fn ovdvdda_action(&mut self) -> OVDVDDA_ACTION_W<22> {
        OVDVDDA_ACTION_W::new(self)
    }
    #[doc = "Bit 24 - Enable for OVD on vdda."]
    #[inline(always)]
    #[must_use]
    pub fn ovdvdda_enable(&mut self) -> OVDVDDA_ENABLE_W<24> {
        OVDVDDA_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - Enable for OVD on vccd. This cannot be disabled during normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn ovdvccd_enable(&mut self) -> OVDVCCD_ENABLE_W<27> {
        OVDVCCD_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Supervision Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ssv_ctl](index.html) module"]
pub struct PWR_SSV_CTL_SPEC;
impl crate::RegisterSpec for PWR_SSV_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ssv_ctl::R](R) reader structure"]
impl crate::Readable for PWR_SSV_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ssv_ctl::W](W) writer structure"]
impl crate::Writable for PWR_SSV_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_SSV_CTL to value 0x0808_0808"]
impl crate::Resettable for PWR_SSV_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808_0808;
}
