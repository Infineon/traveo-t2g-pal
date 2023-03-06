#[doc = "Register `TTOCN` reader"]
pub struct R(crate::R<TTOCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOCN` writer"]
pub struct W(crate::W<TTOCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOCN_SPEC>;
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
impl From<crate::W<TTOCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGT` reader - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
pub type SGT_R = crate::BitReader<bool>;
#[doc = "Field `SGT` writer - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
pub type SGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `ECS` reader - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
pub type ECS_R = crate::BitReader<bool>;
#[doc = "Field `ECS` writer - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
pub type ECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `SWP` reader - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
pub type SWP_R = crate::BitReader<bool>;
#[doc = "Field `SWP` writer - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
pub type SWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `SWS` reader - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWS` writer - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCN_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTIE` reader - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
pub type RTIE_R = crate::BitReader<bool>;
#[doc = "Field `RTIE` writer - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
pub type RTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `TMC` reader - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
pub type TMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMC` writer - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
pub type TMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOCN_SPEC, u8, u8, 2, O>;
#[doc = "Field `TTIE` reader - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
pub type TTIE_R = crate::BitReader<bool>;
#[doc = "Field `TTIE` writer - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
pub type TTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `GCS` reader - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
pub type GCS_R = crate::BitReader<bool>;
#[doc = "Field `GCS` writer - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
pub type GCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `FGP` reader - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
pub type FGP_R = crate::BitReader<bool>;
#[doc = "Field `FGP` writer - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
pub type FGP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `TMG` reader - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
pub type TMG_R = crate::BitReader<bool>;
#[doc = "Field `TMG` writer - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
pub type TMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `NIG` reader - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
pub type NIG_R = crate::BitReader<bool>;
#[doc = "Field `NIG` writer - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
pub type NIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `ESCN` reader - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
pub type ESCN_R = crate::BitReader<bool>;
#[doc = "Field `ESCN` writer - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
pub type ESCN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOCN_SPEC, bool, O>;
#[doc = "Field `LCKC` reader - TT Operation Control Register Locked Set by a write access to register TTOCN. Reset when the updated configuration has been synchronized into the CAN clock domain. 0= Write access to TTOCN enabled 1= Write access to TTOCN locked"]
pub type LCKC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked Set by a write access to register TTOCN. Reset when the updated configuration has been synchronized into the CAN clock domain. 0= Write access to TTOCN enabled 1= Write access to TTOCN locked"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Global time Writing a '1' to SGT sets TTOST.WGDT if the node is the actual Time Master. SGT is reset after one Host clock period. The global time preset takes effect when the node transmits the next reference message with the Master_Ref_Mark modified by the preset value written to TTGTP."]
    #[inline(always)]
    #[must_use]
    pub fn sgt(&mut self) -> SGT_W<0> {
        SGT_W::new(self)
    }
    #[doc = "Bit 1 - External Clock Synchronization Writing a '1' to ECS sets TTOST.WECS if the node is the actual Time Master. ECS is reset after one Host clock period. The external clock synchronization takes effect at the start of the next basic cycle."]
    #[inline(always)]
    #[must_use]
    pub fn ecs(&mut self) -> ECS_W<1> {
        ECS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Watch Polarity 0= Rising edge trigger 1= Falling edge trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swp(&mut self) -> SWP_W<2> {
        SWP_W::new(self)
    }
    #[doc = "Bits 3:4 - Stop Watch Source 00= Stop Watch disabled 01= Actual value of cycle time is copied to TTCPT.SWV 10= Actual value of local time is copied to TTCPT.SWV 11= Actual value of global time is copied to TTCPT.SWV"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<3> {
        SWS_W::new(self)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable Register time mark interrupts are configured by register TTTMK. A register time mark interrupt pulse with the length of one NTU is generated when the time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Register Time Mark Interrupt output m_ttcan_rtp disabled 1= Register Time Mark Interrupt output m_ttcan_rtp enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<5> {
        RTIE_W::new(self)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare 00= No Register Time Mark Interrupt generated 01= Register Time Mark Interrupt if Time Mark = cycle time 10= Register Time Mark Interrupt if Time Mark = local time 11= Register Time Mark Interrupt if Time Mark = global time"]
    #[inline(always)]
    #[must_use]
    pub fn tmc(&mut self) -> TMC_W<6> {
        TMC_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable External time mark events are configured by trigger memory element TMEX (see Section 2.4.7). A trigger time mark interrupt pulse is generated when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Schedule or In_Gap. 0= Trigger Time Mark Interrupt output m_ttcan_tmp disabled 1= Trigger Time Mark Interrupt output m_ttcan_tmp enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ttie(&mut self) -> TTIE_W<8> {
        TTIE_W::new(self)
    }
    #[doc = "Bit 9 - Gap Control Select 0= Gap control independent from m_ttcan_evt 1= Gap control by input pin m_ttcan_evt"]
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GCS_W<9> {
        GCS_W::new(self)
    }
    #[doc = "Bit 10 - Finish Gap Set by the CPU, reset by each reference message 0= No reference message requested 1= Application requested start of reference message"]
    #[inline(always)]
    #[must_use]
    pub fn fgp(&mut self) -> FGP_W<10> {
        FGP_W::new(self)
    }
    #[doc = "Bit 11 - Time Mark Gap 0= Reset by each reference message 1= Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
    #[inline(always)]
    #[must_use]
    pub fn tmg(&mut self) -> TMG_W<11> {
        TMG_W::new(self)
    }
    #[doc = "Bit 12 - Next is Gap This bit can only be set when the M_TTCAN is the actual Time Master and when it is configured for external event-synchronized time-triggered operation (TTOCF.GEN = '1') 0= No action, reset by reception of any reference message 1= Transmit next reference message with Next_is_Gap = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn nig(&mut self) -> NIG_W<12> {
        NIG_W::new(self)
    }
    #[doc = "Bit 13 - External Synchronization Control If enabled the M_TTCAN synchronizes its cycle time phase to an external event signaled by a rising edge at pin m_ttcan_evt (see Section 4.11). 0= External synchronization disabled 1= External synchronization enabled"]
    #[inline(always)]
    #[must_use]
    pub fn escn(&mut self) -> ESCN_W<13> {
        ESCN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TT Operation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocn](index.html) module"]
pub struct TTOCN_SPEC;
impl crate::RegisterSpec for TTOCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttocn::R](R) reader structure"]
impl crate::Readable for TTOCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttocn::W](W) writer structure"]
impl crate::Writable for TTOCN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTOCN to value 0"]
impl crate::Resettable for TTOCN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
