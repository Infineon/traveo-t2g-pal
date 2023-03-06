#[doc = "Register `TTIR` reader"]
pub struct R(crate::R<TTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTIR` writer"]
pub struct W(crate::W<TTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTIR_SPEC>;
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
impl From<crate::W<TTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBC` reader - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
pub type SBC_R = crate::BitReader<bool>;
#[doc = "Field `SBC` writer - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `SMC` reader - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
pub type SMC_R = crate::BitReader<bool>;
#[doc = "Field `SMC` writer - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
pub type SMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `CSM_` reader - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
pub type CSM__R = crate::BitReader<bool>;
#[doc = "Field `CSM_` writer - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
pub type CSM__W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `SOG` reader - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
pub type SOG_R = crate::BitReader<bool>;
#[doc = "Field `SOG` writer - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
pub type SOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `RTMI` reader - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
pub type RTMI_R = crate::BitReader<bool>;
#[doc = "Field `RTMI` writer - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
pub type RTMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `TTMI` reader - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
pub type TTMI_R = crate::BitReader<bool>;
#[doc = "Field `TTMI` writer - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
pub type TTMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `SWE` reader - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
pub type SWE_R = crate::BitReader<bool>;
#[doc = "Field `SWE` writer - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
pub type SWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `GTW` reader - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
pub type GTW_R = crate::BitReader<bool>;
#[doc = "Field `GTW` writer - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
pub type GTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `GTD` reader - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
pub type GTD_R = crate::BitReader<bool>;
#[doc = "Field `GTD` writer - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
pub type GTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `GTE` reader - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
pub type GTE_R = crate::BitReader<bool>;
#[doc = "Field `GTE` writer - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
pub type GTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `TXU` reader - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
pub type TXU_R = crate::BitReader<bool>;
#[doc = "Field `TXU` writer - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
pub type TXU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `TXO` reader - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
pub type TXO_R = crate::BitReader<bool>;
#[doc = "Field `TXO` writer - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
pub type TXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `SE1` reader - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
pub type SE1_R = crate::BitReader<bool>;
#[doc = "Field `SE1` writer - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
pub type SE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `SE2` reader - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
pub type SE2_R = crate::BitReader<bool>;
#[doc = "Field `SE2` writer - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
pub type SE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `ELC` reader - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
pub type ELC_R = crate::BitReader<bool>;
#[doc = "Field `ELC` writer - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
pub type ELC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `IWT` reader - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
pub type IWT_R = crate::BitReader<bool>;
#[doc = "Field `IWT` writer - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
pub type IWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `WT` reader - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
pub type WT_R = crate::BitReader<bool>;
#[doc = "Field `WT` writer - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
pub type WT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `AW` reader - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
pub type AW_R = crate::BitReader<bool>;
#[doc = "Field `AW` writer - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
pub type AW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
#[doc = "Field `CER` reader - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
pub type CER_R = crate::BitReader<bool>;
#[doc = "Field `CER` writer - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
pub type CER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    pub fn csm_(&self) -> CSM__R {
        CSM__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    pub fn iwt(&self) -> IWT_R {
        IWT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle 0= No Basic Cycle started since bit has been reset 1= Basic Cycle started"]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<0> {
        SBC_W::new(self)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle 0= No Matrix Cycle started since bit has been reset 1= Matrix Cycle started"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<1> {
        SMC_W::new(self)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode 0= No change in master to slave relation or schedule synchronization 1= Master to slave relation or schedule synchronization changed, also set when TTOST.SPL is reset"]
    #[inline(always)]
    #[must_use]
    pub fn csm_(&mut self) -> CSM__W<2> {
        CSM__W::new(self)
    }
    #[doc = "Bit 3 - Start of Gap 0= No reference message seen with Next_is_Gap bit set 1= Reference message with Next_is_Gap bit set becomes valid"]
    #[inline(always)]
    #[must_use]
    pub fn sog(&mut self) -> SOG_W<3> {
        SOG_W::new(self)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Set when time referenced by TTOCN.TMC (cycle, local, or global) equals TTTMK.TM, independent of the synchronization state. 0= Time mark not reached 1= Time mark reached"]
    #[inline(always)]
    #[must_use]
    pub fn rtmi(&mut self) -> RTMI_W<4> {
        RTMI_W::new(self)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Internal time mark events are configured by trigger memory element TMIN (see Section 2.4.7). Set when the trigger memory element becomes active, and the M_TTCAN is in synchronization state In_Gap or In_Schedule. 0= Time mark not reached 1= Time mark reached (Level 0: cycle time TTOCF.IRTO * 0x200)"]
    #[inline(always)]
    #[must_use]
    pub fn ttmi(&mut self) -> TTMI_W<5> {
        TTMI_W::new(self)
    }
    #[doc = "Bit 6 - Stop Watch Event 0= No rising/falling edge at stop watch trigger pin m_ttcan_swt detected 1= Rising/falling edge at stop watch trigger pin m_ttcan_swt detected"]
    #[inline(always)]
    #[must_use]
    pub fn swe(&mut self) -> SWE_W<6> {
        SWE_W::new(self)
    }
    #[doc = "Bit 7 - Global Time Wrap 0= No global time wrap occurred 1= Global time wrap from 0xFFFF to 0x0000 occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gtw(&mut self) -> GTW_W<7> {
        GTW_W::new(self)
    }
    #[doc = "Bit 8 - Global Time Discontinuity 0= No discontinuity of global time 1= Discontinuity of global time"]
    #[inline(always)]
    #[must_use]
    pub fn gtd(&mut self) -> GTD_W<8> {
        GTD_W::new(self)
    }
    #[doc = "Bit 9 - Global Time Error Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL, TTCAN Level 0,2 only. 0= Synchronization deviation within limit 1= Synchronization deviation exceeded limit"]
    #[inline(always)]
    #[must_use]
    pub fn gte(&mut self) -> GTE_W<9> {
        GTE_W::new(self)
    }
    #[doc = "Bit 10 - Tx Count Underflow 0= Number of Tx Trigger as expected 1= Less Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    #[must_use]
    pub fn txu(&mut self) -> TXU_W<10> {
        TXU_W::new(self)
    }
    #[doc = "Bit 11 - Tx Count Overflow 0= Number of Tx Trigger as expected 1= More Tx trigger than expected in one matrix cycle"]
    #[inline(always)]
    #[must_use]
    pub fn txo(&mut self) -> TXO_W<11> {
        TXO_W::new(self)
    }
    #[doc = "Bit 12 - Scheduling Error 1 0= No scheduling error 1 1= Scheduling error 1 occurred"]
    #[inline(always)]
    #[must_use]
    pub fn se1(&mut self) -> SE1_W<12> {
        SE1_W::new(self)
    }
    #[doc = "Bit 13 - Scheduling Error 2 0= No scheduling error 2 1= Scheduling error 2 occurred"]
    #[inline(always)]
    #[must_use]
    pub fn se2(&mut self) -> SE2_W<13> {
        SE2_W::new(self)
    }
    #[doc = "Bit 14 - Error Level Changed Not set when error level changed during initialization. 0= No change in error level 1= Error level changed"]
    #[inline(always)]
    #[must_use]
    pub fn elc(&mut self) -> ELC_W<14> {
        ELC_W::new(self)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger The initialization is restarted by resetting IWT. 0= No missing reference message during system startup 1= No system startup due to missing reference message"]
    #[inline(always)]
    #[must_use]
    pub fn iwt(&mut self) -> IWT_W<15> {
        IWT_W::new(self)
    }
    #[doc = "Bit 16 - Watch Trigger 0= No missing reference message 1= Missing reference message (Level 0: cycle time 0xFF00)"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<16> {
        WT_W::new(self)
    }
    #[doc = "Bit 17 - Application Watchdog 0= Application watchdog served in time 1= Application watchdog not served in time"]
    #[inline(always)]
    #[must_use]
    pub fn aw(&mut self) -> AW_W<17> {
        AW_W::new(self)
    }
    #[doc = "Bit 18 - Configuration Error Trigger out of order. 0= No error found in trigger list 1= Error found in trigger list"]
    #[inline(always)]
    #[must_use]
    pub fn cer(&mut self) -> CER_W<18> {
        CER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttir](index.html) module"]
pub struct TTIR_SPEC;
impl crate::RegisterSpec for TTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttir::R](R) reader structure"]
impl crate::Readable for TTIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttir::W](W) writer structure"]
impl crate::Writable for TTIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTIR to value 0"]
impl crate::Resettable for TTIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
