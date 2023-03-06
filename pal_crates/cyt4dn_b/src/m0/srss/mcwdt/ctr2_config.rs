#[doc = "Register `CTR2_CONFIG` reader"]
pub struct R(crate::R<CTR2_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR2_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR2_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR2_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR2_CONFIG` writer"]
pub struct W(crate::W<CTR2_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR2_CONFIG_SPEC>;
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
impl From<crate::W<CTR2_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR2_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTION` reader - Action taken when the specified BIT toggles. Action will be triggered on the same edge where BITS to observe toggle."]
pub type ACTION_R = crate::BitReader<ACTION_A>;
#[doc = "Action taken when the specified BIT toggles. Action will be triggered on the same edge where BITS to observe toggle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTION_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Trigger an interrupt"]
    INT = 1,
}
impl From<ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTION_A {
        match self.bits {
            false => ACTION_A::NOTHING,
            true => ACTION_A::INT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == ACTION_A::INT
    }
}
#[doc = "Field `ACTION` writer - Action taken when the specified BIT toggles. Action will be triggered on the same edge where BITS to observe toggle."]
pub type ACTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR2_CONFIG_SPEC, ACTION_A, O>;
impl<'a, const O: u8> ACTION_W<'a, O> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(ACTION_A::NOTHING)
    }
    #[doc = "Trigger an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(ACTION_A::INT)
    }
}
#[doc = "Field `BITS` reader - Bit to observe for a toggle: 0: Do ACTION after CNT\\[0\\]
toggles (i.e. every tick) . 31: Do ACTION after CNT\\[31\\]
toggles (i.e. every 2^31 ticks)"]
pub type BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITS` writer - Bit to observe for a toggle: 0: Do ACTION after CNT\\[0\\]
toggles (i.e. every tick) . 31: Do ACTION after CNT\\[31\\]
toggles (i.e. every 2^31 ticks)"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTR2_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_TRIGGER_EN` reader - Enables the trigger input for this MCWDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this MCWDT, and then set this bit. It takes up to two clk_lf cycles for the trigger signal to be processed. Triggers that are less than two clk_lf cycles may be missed. Synchronization errors can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
pub type DEBUG_TRIGGER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_TRIGGER_EN` writer - Enables the trigger input for this MCWDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this MCWDT, and then set this bit. It takes up to two clk_lf cycles for the trigger signal to be processed. Triggers that are less than two clk_lf cycles may be missed. Synchronization errors can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
pub type DEBUG_TRIGGER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR2_CONFIG_SPEC, bool, O>;
#[doc = "Field `SLEEPDEEP_PAUSE` reader - Pauses/runs this counter when the corresponding processor is in SLEEPDEEP. Note it may take up to two clk_lf cycles for the counter to pause and up to two clk_lf cycles for it to unpause, due to internal synchronization. 0: Counter runs normally regardless of processor mode. 1: Counter pauses when corresponding processor is in SLEEPDEEP."]
pub type SLEEPDEEP_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEP_PAUSE` writer - Pauses/runs this counter when the corresponding processor is in SLEEPDEEP. Note it may take up to two clk_lf cycles for the counter to pause and up to two clk_lf cycles for it to unpause, due to internal synchronization. 0: Counter runs normally regardless of processor mode. 1: Counter pauses when corresponding processor is in SLEEPDEEP."]
pub type SLEEPDEEP_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR2_CONFIG_SPEC, bool, O>;
#[doc = "Field `DEBUG_RUN` reader - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_lf cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
pub type DEBUG_RUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_RUN` writer - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_lf cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
pub type DEBUG_RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR2_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Action taken when the specified BIT toggles. Action will be triggered on the same edge where BITS to observe toggle."]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bit to observe for a toggle: 0: Do ACTION after CNT\\[0\\]
toggles (i.e. every tick) . 31: Do ACTION after CNT\\[31\\]
toggles (i.e. every 2^31 ticks)"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Enables the trigger input for this MCWDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this MCWDT, and then set this bit. It takes up to two clk_lf cycles for the trigger signal to be processed. Triggers that are less than two clk_lf cycles may be missed. Synchronization errors can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
    #[inline(always)]
    pub fn debug_trigger_en(&self) -> DEBUG_TRIGGER_EN_R {
        DEBUG_TRIGGER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Pauses/runs this counter when the corresponding processor is in SLEEPDEEP. Note it may take up to two clk_lf cycles for the counter to pause and up to two clk_lf cycles for it to unpause, due to internal synchronization. 0: Counter runs normally regardless of processor mode. 1: Counter pauses when corresponding processor is in SLEEPDEEP."]
    #[inline(always)]
    pub fn sleepdeep_pause(&self) -> SLEEPDEEP_PAUSE_R {
        SLEEPDEEP_PAUSE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_lf cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
    #[inline(always)]
    pub fn debug_run(&self) -> DEBUG_RUN_R {
        DEBUG_RUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Action taken when the specified BIT toggles. Action will be triggered on the same edge where BITS to observe toggle."]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<0> {
        ACTION_W::new(self)
    }
    #[doc = "Bits 16:20 - Bit to observe for a toggle: 0: Do ACTION after CNT\\[0\\]
toggles (i.e. every tick) . 31: Do ACTION after CNT\\[31\\]
toggles (i.e. every 2^31 ticks)"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<16> {
        BITS_W::new(self)
    }
    #[doc = "Bit 28 - Enables the trigger input for this MCWDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this MCWDT, and then set this bit. It takes up to two clk_lf cycles for the trigger signal to be processed. Triggers that are less than two clk_lf cycles may be missed. Synchronization errors can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
    #[inline(always)]
    #[must_use]
    pub fn debug_trigger_en(&mut self) -> DEBUG_TRIGGER_EN_W<28> {
        DEBUG_TRIGGER_EN_W::new(self)
    }
    #[doc = "Bit 30 - Pauses/runs this counter when the corresponding processor is in SLEEPDEEP. Note it may take up to two clk_lf cycles for the counter to pause and up to two clk_lf cycles for it to unpause, due to internal synchronization. 0: Counter runs normally regardless of processor mode. 1: Counter pauses when corresponding processor is in SLEEPDEEP."]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep_pause(&mut self) -> SLEEPDEEP_PAUSE_W<30> {
        SLEEPDEEP_PAUSE_W::new(self)
    }
    #[doc = "Bit 31 - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_lf cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
    #[inline(always)]
    #[must_use]
    pub fn debug_run(&mut self) -> DEBUG_RUN_W<31> {
        DEBUG_RUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCWDT Subcounter 2 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr2_config](index.html) module"]
pub struct CTR2_CONFIG_SPEC;
impl crate::RegisterSpec for CTR2_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr2_config::R](R) reader structure"]
impl crate::Readable for CTR2_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr2_config::W](W) writer structure"]
impl crate::Writable for CTR2_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR2_CONFIG to value 0"]
impl crate::Resettable for CTR2_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
