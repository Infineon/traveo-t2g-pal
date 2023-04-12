#[doc = "Register `TR_CTL` reader"]
pub struct R(crate::R<TR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL` writer"]
pub struct W(crate::W<TR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL_SPEC>;
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
impl From<crate::W<TR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Trigger select"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Use for channels in group, except the first channel"]
    OFF = 0,
    #[doc = "1: Trigger from corresponding TCPWM channel"]
    TCPWM = 1,
    #[doc = "2: Generic trigger input 0"]
    GENERIC0 = 2,
    #[doc = "3: N/A"]
    GENERIC1 = 3,
    #[doc = "4: N/A"]
    GENERIC2 = 4,
    #[doc = "5: N/A"]
    GENERIC3 = 5,
    #[doc = "6: N/A"]
    GENERIC4 = 6,
    #[doc = "7: Always triggered (also called idle), can only be used for at most 1 channel"]
    CONTINUOUS = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::OFF,
            1 => SEL_A::TCPWM,
            2 => SEL_A::GENERIC0,
            3 => SEL_A::GENERIC1,
            4 => SEL_A::GENERIC2,
            5 => SEL_A::GENERIC3,
            6 => SEL_A::GENERIC4,
            7 => SEL_A::CONTINUOUS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `TCPWM`"]
    #[inline(always)]
    pub fn is_tcpwm(&self) -> bool {
        *self == SEL_A::TCPWM
    }
    #[doc = "Checks if the value of the field is `GENERIC0`"]
    #[inline(always)]
    pub fn is_generic0(&self) -> bool {
        *self == SEL_A::GENERIC0
    }
    #[doc = "Checks if the value of the field is `GENERIC1`"]
    #[inline(always)]
    pub fn is_generic1(&self) -> bool {
        *self == SEL_A::GENERIC1
    }
    #[doc = "Checks if the value of the field is `GENERIC2`"]
    #[inline(always)]
    pub fn is_generic2(&self) -> bool {
        *self == SEL_A::GENERIC2
    }
    #[doc = "Checks if the value of the field is `GENERIC3`"]
    #[inline(always)]
    pub fn is_generic3(&self) -> bool {
        *self == SEL_A::GENERIC3
    }
    #[doc = "Checks if the value of the field is `GENERIC4`"]
    #[inline(always)]
    pub fn is_generic4(&self) -> bool {
        *self == SEL_A::GENERIC4
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == SEL_A::CONTINUOUS
    }
}
#[doc = "Field `SEL` writer - Trigger select"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TR_CTL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Use for channels in group, except the first channel"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SEL_A::OFF)
    }
    #[doc = "Trigger from corresponding TCPWM channel"]
    #[inline(always)]
    pub fn tcpwm(self) -> &'a mut W {
        self.variant(SEL_A::TCPWM)
    }
    #[doc = "Generic trigger input 0"]
    #[inline(always)]
    pub fn generic0(self) -> &'a mut W {
        self.variant(SEL_A::GENERIC0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn generic1(self) -> &'a mut W {
        self.variant(SEL_A::GENERIC1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn generic2(self) -> &'a mut W {
        self.variant(SEL_A::GENERIC2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn generic3(self) -> &'a mut W {
        self.variant(SEL_A::GENERIC3)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn generic4(self) -> &'a mut W {
        self.variant(SEL_A::GENERIC4)
    }
    #[doc = "Always triggered (also called idle), can only be used for at most 1 channel"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(SEL_A::CONTINUOUS)
    }
}
#[doc = "Field `PRIO` reader - Channel priority: '0': highest priority. '1' ... '6' '7': lowest priority. Channels with the same priority constitute a priority level. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority level with pending channels is identified. Second, within this priority level, round robin arbitration is applied. Round robin arbitration (within a priority level) gives the highest priority to the lower channel indices (within the priority level)."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Channel priority: '0': highest priority. '1' ... '6' '7': lowest priority. Channels with the same priority constitute a priority level. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority level with pending channels is identified. Second, within this priority level, round robin arbitration is applied. Round robin arbitration (within a priority level) gives the highest priority to the lower channel indices (within the priority level)."]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PREEMPT_TYPE` reader - Preemption type allow for this group"]
pub type PREEMPT_TYPE_R = crate::FieldReader<u8, PREEMPT_TYPE_A>;
#[doc = "Preemption type allow for this group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREEMPT_TYPE_A {
    #[doc = "0: Abort ongoing acquisition, do not return Clear pending trigger for aborted group and set Cancelled interrupt. Also 'Abort' whenever this group (do not pend the trigger) is not immediately scheduled for acquisition after a new trigger arrives. For this preemption type only, only a positive edge on the trigger can trigger the channel, i.e. CONTINUOUS or level high operation is not supported (to avoid continuous Cancelled interrupts). In case CTL.IDLE_PWRDWN is used and the analog is powered down, the group cannot be immediately scheduled for acquisition and therefore a trigger for a group with this preemption type will power up the analog, but the group will ABORT and set the Cancelled interrupt"]
    ABORT_CANCEL = 0,
    #[doc = "1: Abort ongoing acquisition, up on return Restart group from first channel."]
    ABORT_RESTART = 1,
    #[doc = "2: Abort ongoing acquisition, up on return Resume group from aborted channel If averaging, discard averaging results so far and restart averaging."]
    ABORT_RESUME = 2,
    #[doc = "3: Complete ongoing acquisition (including averaging), up on return Resume group from next channel"]
    FINISH_RESUME = 3,
}
impl From<PREEMPT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PREEMPT_TYPE_A) -> Self {
        variant as _
    }
}
impl PREEMPT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEMPT_TYPE_A {
        match self.bits {
            0 => PREEMPT_TYPE_A::ABORT_CANCEL,
            1 => PREEMPT_TYPE_A::ABORT_RESTART,
            2 => PREEMPT_TYPE_A::ABORT_RESUME,
            3 => PREEMPT_TYPE_A::FINISH_RESUME,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABORT_CANCEL`"]
    #[inline(always)]
    pub fn is_abort_cancel(&self) -> bool {
        *self == PREEMPT_TYPE_A::ABORT_CANCEL
    }
    #[doc = "Checks if the value of the field is `ABORT_RESTART`"]
    #[inline(always)]
    pub fn is_abort_restart(&self) -> bool {
        *self == PREEMPT_TYPE_A::ABORT_RESTART
    }
    #[doc = "Checks if the value of the field is `ABORT_RESUME`"]
    #[inline(always)]
    pub fn is_abort_resume(&self) -> bool {
        *self == PREEMPT_TYPE_A::ABORT_RESUME
    }
    #[doc = "Checks if the value of the field is `FINISH_RESUME`"]
    #[inline(always)]
    pub fn is_finish_resume(&self) -> bool {
        *self == PREEMPT_TYPE_A::FINISH_RESUME
    }
}
#[doc = "Field `PREEMPT_TYPE` writer - Preemption type allow for this group"]
pub type PREEMPT_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTL_SPEC, u8, PREEMPT_TYPE_A, 2, O>;
impl<'a, const O: u8> PREEMPT_TYPE_W<'a, O> {
    #[doc = "Abort ongoing acquisition, do not return Clear pending trigger for aborted group and set Cancelled interrupt. Also 'Abort' whenever this group (do not pend the trigger) is not immediately scheduled for acquisition after a new trigger arrives. For this preemption type only, only a positive edge on the trigger can trigger the channel, i.e. CONTINUOUS or level high operation is not supported (to avoid continuous Cancelled interrupts). In case CTL.IDLE_PWRDWN is used and the analog is powered down, the group cannot be immediately scheduled for acquisition and therefore a trigger for a group with this preemption type will power up the analog, but the group will ABORT and set the Cancelled interrupt"]
    #[inline(always)]
    pub fn abort_cancel(self) -> &'a mut W {
        self.variant(PREEMPT_TYPE_A::ABORT_CANCEL)
    }
    #[doc = "Abort ongoing acquisition, up on return Restart group from first channel."]
    #[inline(always)]
    pub fn abort_restart(self) -> &'a mut W {
        self.variant(PREEMPT_TYPE_A::ABORT_RESTART)
    }
    #[doc = "Abort ongoing acquisition, up on return Resume group from aborted channel If averaging, discard averaging results so far and restart averaging."]
    #[inline(always)]
    pub fn abort_resume(self) -> &'a mut W {
        self.variant(PREEMPT_TYPE_A::ABORT_RESUME)
    }
    #[doc = "Complete ongoing acquisition (including averaging), up on return Resume group from next channel"]
    #[inline(always)]
    pub fn finish_resume(self) -> &'a mut W {
        self.variant(PREEMPT_TYPE_A::FINISH_RESUME)
    }
}
#[doc = "Field `GROUP_END` reader - 0: continue group with next channel 1: last channel of a group. Note that for the channel with the highest index (SAR_CH_NR) this always needs to be set"]
pub type GROUP_END_R = crate::BitReader<bool>;
#[doc = "Field `GROUP_END` writer - 0: continue group with next channel 1: last channel of a group. Note that for the channel with the highest index (SAR_CH_NR) this always needs to be set"]
pub type GROUP_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL_SPEC, bool, O>;
#[doc = "Field `DONE_LEVEL` reader - select level or pulse for 'tr_ch_done' trigger output Also see POST_CTL.TR_DONE_GRP_VIO"]
pub type DONE_LEVEL_R = crate::BitReader<DONE_LEVEL_A>;
#[doc = "select level or pulse for 'tr_ch_done' trigger output Also see POST_CTL.TR_DONE_GRP_VIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_LEVEL_A {
    #[doc = "0: tr_ch_done generates a 2 cycle pulse (clk_sys), no need to read the result to clear (also no ch_overflow detection)"]
    PULSE = 0,
    #[doc = "1: tr_ch_done is a level output until the result register is read (typical for DW usage, this also enables ch_overflow detection when DW is too slow)"]
    LEVEL = 1,
}
impl From<DONE_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_LEVEL_A {
        match self.bits {
            false => DONE_LEVEL_A::PULSE,
            true => DONE_LEVEL_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == DONE_LEVEL_A::PULSE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DONE_LEVEL_A::LEVEL
    }
}
#[doc = "Field `DONE_LEVEL` writer - select level or pulse for 'tr_ch_done' trigger output Also see POST_CTL.TR_DONE_GRP_VIO"]
pub type DONE_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL_SPEC, DONE_LEVEL_A, O>;
impl<'a, const O: u8> DONE_LEVEL_W<'a, O> {
    #[doc = "tr_ch_done generates a 2 cycle pulse (clk_sys), no need to read the result to clear (also no ch_overflow detection)"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(DONE_LEVEL_A::PULSE)
    }
    #[doc = "tr_ch_done is a level output until the result register is read (typical for DW usage, this also enables ch_overflow detection when DW is too slow)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DONE_LEVEL_A::LEVEL)
    }
}
impl R {
    #[doc = "Bits 0:2 - Trigger select"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Channel priority: '0': highest priority. '1' ... '6' '7': lowest priority. Channels with the same priority constitute a priority level. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority level with pending channels is identified. Second, within this priority level, round robin arbitration is applied. Round robin arbitration (within a priority level) gives the highest priority to the lower channel indices (within the priority level)."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Preemption type allow for this group"]
    #[inline(always)]
    pub fn preempt_type(&self) -> PREEMPT_TYPE_R {
        PREEMPT_TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - 0: continue group with next channel 1: last channel of a group. Note that for the channel with the highest index (SAR_CH_NR) this always needs to be set"]
    #[inline(always)]
    pub fn group_end(&self) -> GROUP_END_R {
        GROUP_END_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - select level or pulse for 'tr_ch_done' trigger output Also see POST_CTL.TR_DONE_GRP_VIO"]
    #[inline(always)]
    pub fn done_level(&self) -> DONE_LEVEL_R {
        DONE_LEVEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel priority: '0': highest priority. '1' ... '6' '7': lowest priority. Channels with the same priority constitute a priority level. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority level with pending channels is identified. Second, within this priority level, round robin arbitration is applied. Round robin arbitration (within a priority level) gives the highest priority to the lower channel indices (within the priority level)."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<4> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 8:9 - Preemption type allow for this group"]
    #[inline(always)]
    #[must_use]
    pub fn preempt_type(&mut self) -> PREEMPT_TYPE_W<8> {
        PREEMPT_TYPE_W::new(self)
    }
    #[doc = "Bit 11 - 0: continue group with next channel 1: last channel of a group. Note that for the channel with the highest index (SAR_CH_NR) this always needs to be set"]
    #[inline(always)]
    #[must_use]
    pub fn group_end(&mut self) -> GROUP_END_W<11> {
        GROUP_END_W::new(self)
    }
    #[doc = "Bit 31 - select level or pulse for 'tr_ch_done' trigger output Also see POST_CTL.TR_DONE_GRP_VIO"]
    #[inline(always)]
    #[must_use]
    pub fn done_level(&mut self) -> DONE_LEVEL_W<31> {
        DONE_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](index.html) module"]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl::R](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL to value 0x0800"]
impl crate::Resettable for TR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
