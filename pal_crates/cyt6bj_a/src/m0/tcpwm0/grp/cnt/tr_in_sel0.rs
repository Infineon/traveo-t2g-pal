#[doc = "Register `TR_IN_SEL0` reader"]
pub struct R(crate::R<TR_IN_SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_IN_SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_IN_SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_IN_SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_IN_SEL0` writer"]
pub struct W(crate::W<TR_IN_SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_IN_SEL0_SPEC>;
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
impl From<crate::W<TR_IN_SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_IN_SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE0_SEL` reader - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub type CAPTURE0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPTURE0_SEL` writer - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub type CAPTURE0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_IN_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COUNT_SEL` reader - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'. Note: In the modes: TIMER, CAPTURE, PWM, PWM_DT, and SR, If the counter is externally triggered ( COUNT_SEL > 1), an external trigger will be required for each TR_CMD to execute. For example, a write to TR_CMD.START will not start the counter until the trigger selected by COUNT_SEL asserts. The next trigger will increment the counter since the counter is now running. This goes for all TR_CMD fields."]
pub type COUNT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNT_SEL` writer - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'. Note: In the modes: TIMER, CAPTURE, PWM, PWM_DT, and SR, If the counter is externally triggered ( COUNT_SEL > 1), an external trigger will be required for each TR_CMD to execute. For example, a write to TR_CMD.START will not start the counter until the trigger selected by COUNT_SEL asserts. The next trigger will increment the counter since the counter is now running. This goes for all TR_CMD fields."]
pub type COUNT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_IN_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RELOAD_SEL` reader - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
pub type RELOAD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RELOAD_SEL` writer - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
pub type RELOAD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_IN_SEL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `STOP_SEL` reader - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub type STOP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_SEL` writer - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub type STOP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_IN_SEL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture0_sel(&self) -> CAPTURE0_SEL_R {
        CAPTURE0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'. Note: In the modes: TIMER, CAPTURE, PWM, PWM_DT, and SR, If the counter is externally triggered ( COUNT_SEL > 1), an external trigger will be required for each TR_CMD to execute. For example, a write to TR_CMD.START will not start the counter until the trigger selected by COUNT_SEL asserts. The next trigger will increment the counter since the counter is now running. This goes for all TR_CMD fields."]
    #[inline(always)]
    pub fn count_sel(&self) -> COUNT_SEL_R {
        COUNT_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&self) -> STOP_SEL_R {
        STOP_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects one of the up to 256 input triggers as a capture0 trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. If existing, the one-to-one trigger inputs 'tr_one_cnt_in' (different to each counter) are selected by setting 2 and above. The settings above are used for the general purpose trigger inputs 'tr_all_cnt_in' connected to all counters selected. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    #[must_use]
    pub fn capture0_sel(&mut self) -> CAPTURE0_SEL_W<0> {
        CAPTURE0_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'. Note: In the modes: TIMER, CAPTURE, PWM, PWM_DT, and SR, If the counter is externally triggered ( COUNT_SEL > 1), an external trigger will be required for each TR_CMD to execute. For example, a write to TR_CMD.START will not start the counter until the trigger selected by COUNT_SEL asserts. The next trigger will increment the counter since the counter is now running. This goes for all TR_CMD fields."]
    #[inline(always)]
    #[must_use]
    pub fn count_sel(&mut self) -> COUNT_SEL_W<8> {
        COUNT_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Selects one of the 256 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In these modes, it will update the counter with 0x8000 (counter midpoint) or 0x0000 depending on the QUAD_RANGE_MODE."]
    #[inline(always)]
    #[must_use]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W<16> {
        RELOAD_SEL_W::new(self)
    }
    #[doc = "Bits 24:31 - Selects one of the 256 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    #[must_use]
    pub fn stop_sel(&mut self) -> STOP_SEL_W<24> {
        STOP_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter input trigger selection register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_sel0](index.html) module"]
pub struct TR_IN_SEL0_SPEC;
impl crate::RegisterSpec for TR_IN_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_in_sel0::R](R) reader structure"]
impl crate::Readable for TR_IN_SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_in_sel0::W](W) writer structure"]
impl crate::Writable for TR_IN_SEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_IN_SEL0 to value 0x0100"]
impl crate::Resettable for TR_IN_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
