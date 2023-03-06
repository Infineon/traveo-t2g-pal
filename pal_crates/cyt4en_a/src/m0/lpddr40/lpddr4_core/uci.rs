#[doc = "Register `UCI` reader"]
pub struct R(crate::R<UCI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCI` writer"]
pub struct W(crate::W<UCI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCI_SPEC>;
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
impl From<crate::W<UCI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_OP` reader - User Command Opcode. Valid values are: USER_CMD_STOP = 4'b0000 Stop Normal Operation USER_CMD_RUN = 4'b0001 Start Normal Operation USER_CMD_SRE = 4'b0010 Self-Refresh Mode Entry USER_CMD_SRX = 4'b0011 Self-Refresh Mode Exit USER_CMD_PDE = 4'b0100 Power-Down Mode Entry USER_CMD_PDX = 4'b0101 Power-Down Mode Exit USER_CMD_ZQRS = 4'b0110 ZQ Calibration Reset USER_CMD_MRR = 4'b0111 Mode Register Read USER_CMD_MRW = 4'b1000 Mode Register Write USER_CMD_BIST = 4'b1001 BIST USER_CMD_ZQSTART = 4'b1010 ZQSTART USER_CMD_ZQLAT = 4'b1011 ZQLAT USER_CMD_DQSOSC_START = 4'b1100 Start DQS Oscillator USER_CMD_DQSOSC_STOP = 4'b1101 Stop DQS Oscillator In case an invalid command or an invalid command sequence is detected the controller will enter STOP mode and raise a fault signal if dmcfg.int_gc_fsm_en is enabled."]
pub type CMD_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_OP` writer - User Command Opcode. Valid values are: USER_CMD_STOP = 4'b0000 Stop Normal Operation USER_CMD_RUN = 4'b0001 Start Normal Operation USER_CMD_SRE = 4'b0010 Self-Refresh Mode Entry USER_CMD_SRX = 4'b0011 Self-Refresh Mode Exit USER_CMD_PDE = 4'b0100 Power-Down Mode Entry USER_CMD_PDX = 4'b0101 Power-Down Mode Exit USER_CMD_ZQRS = 4'b0110 ZQ Calibration Reset USER_CMD_MRR = 4'b0111 Mode Register Read USER_CMD_MRW = 4'b1000 Mode Register Write USER_CMD_BIST = 4'b1001 BIST USER_CMD_ZQSTART = 4'b1010 ZQSTART USER_CMD_ZQLAT = 4'b1011 ZQLAT USER_CMD_DQSOSC_START = 4'b1100 Start DQS Oscillator USER_CMD_DQSOSC_STOP = 4'b1101 Stop DQS Oscillator In case an invalid command or an invalid command sequence is detected the controller will enter STOP mode and raise a fault signal if dmcfg.int_gc_fsm_en is enabled."]
pub type CMD_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UCI_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMD_CHAN` reader - N/A"]
pub type CMD_CHAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_CHAN` writer - N/A"]
pub type CMD_CHAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UCI_SPEC, u8, u8, 2, O>;
#[doc = "Field `MR_SEL` reader - Register Select Argument for MRS/MRW/MRR Command"]
pub type MR_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MR_SEL` writer - Register Select Argument for MRS/MRW/MRR Command"]
pub type MR_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UCI_SPEC, u8, u8, 6, O>;
#[doc = "Field `MR_OP` reader - Register Opcode (MRW, MPC) See LPDDR4 memory datasheet for definition of opcodes."]
pub type MR_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MR_OP` writer - Register Opcode (MRW, MPC) See LPDDR4 memory datasheet for definition of opcodes."]
pub type MR_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UCI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - User Command Opcode. Valid values are: USER_CMD_STOP = 4'b0000 Stop Normal Operation USER_CMD_RUN = 4'b0001 Start Normal Operation USER_CMD_SRE = 4'b0010 Self-Refresh Mode Entry USER_CMD_SRX = 4'b0011 Self-Refresh Mode Exit USER_CMD_PDE = 4'b0100 Power-Down Mode Entry USER_CMD_PDX = 4'b0101 Power-Down Mode Exit USER_CMD_ZQRS = 4'b0110 ZQ Calibration Reset USER_CMD_MRR = 4'b0111 Mode Register Read USER_CMD_MRW = 4'b1000 Mode Register Write USER_CMD_BIST = 4'b1001 BIST USER_CMD_ZQSTART = 4'b1010 ZQSTART USER_CMD_ZQLAT = 4'b1011 ZQLAT USER_CMD_DQSOSC_START = 4'b1100 Start DQS Oscillator USER_CMD_DQSOSC_STOP = 4'b1101 Stop DQS Oscillator In case an invalid command or an invalid command sequence is detected the controller will enter STOP mode and raise a fault signal if dmcfg.int_gc_fsm_en is enabled."]
    #[inline(always)]
    pub fn cmd_op(&self) -> CMD_OP_R {
        CMD_OP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    pub fn cmd_chan(&self) -> CMD_CHAN_R {
        CMD_CHAN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:11 - Register Select Argument for MRS/MRW/MRR Command"]
    #[inline(always)]
    pub fn mr_sel(&self) -> MR_SEL_R {
        MR_SEL_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - Register Opcode (MRW, MPC) See LPDDR4 memory datasheet for definition of opcodes."]
    #[inline(always)]
    pub fn mr_op(&self) -> MR_OP_R {
        MR_OP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Command Opcode. Valid values are: USER_CMD_STOP = 4'b0000 Stop Normal Operation USER_CMD_RUN = 4'b0001 Start Normal Operation USER_CMD_SRE = 4'b0010 Self-Refresh Mode Entry USER_CMD_SRX = 4'b0011 Self-Refresh Mode Exit USER_CMD_PDE = 4'b0100 Power-Down Mode Entry USER_CMD_PDX = 4'b0101 Power-Down Mode Exit USER_CMD_ZQRS = 4'b0110 ZQ Calibration Reset USER_CMD_MRR = 4'b0111 Mode Register Read USER_CMD_MRW = 4'b1000 Mode Register Write USER_CMD_BIST = 4'b1001 BIST USER_CMD_ZQSTART = 4'b1010 ZQSTART USER_CMD_ZQLAT = 4'b1011 ZQLAT USER_CMD_DQSOSC_START = 4'b1100 Start DQS Oscillator USER_CMD_DQSOSC_STOP = 4'b1101 Stop DQS Oscillator In case an invalid command or an invalid command sequence is detected the controller will enter STOP mode and raise a fault signal if dmcfg.int_gc_fsm_en is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_op(&mut self) -> CMD_OP_W<0> {
        CMD_OP_W::new(self)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_chan(&mut self) -> CMD_CHAN_W<4> {
        CMD_CHAN_W::new(self)
    }
    #[doc = "Bits 6:11 - Register Select Argument for MRS/MRW/MRR Command"]
    #[inline(always)]
    #[must_use]
    pub fn mr_sel(&mut self) -> MR_SEL_W<6> {
        MR_SEL_W::new(self)
    }
    #[doc = "Bits 12:19 - Register Opcode (MRW, MPC) See LPDDR4 memory datasheet for definition of opcodes."]
    #[inline(always)]
    #[must_use]
    pub fn mr_op(&mut self) -> MR_OP_W<12> {
        MR_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Command Interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uci](index.html) module"]
pub struct UCI_SPEC;
impl crate::RegisterSpec for UCI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uci::R](R) reader structure"]
impl crate::Readable for UCI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uci::W](W) writer structure"]
impl crate::Writable for UCI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCI to value 0"]
impl crate::Resettable for UCI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
