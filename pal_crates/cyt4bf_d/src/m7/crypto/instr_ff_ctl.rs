#[doc = "Register `INSTR_FF_CTL` reader"]
pub struct R(crate::R<INSTR_FF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTR_FF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTR_FF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTR_FF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSTR_FF_CTL` writer"]
pub struct W(crate::W<INSTR_FF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTR_FF_CTL_SPEC>;
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
impl From<crate::W<INSTR_FF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTR_FF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT_LEVEL` reader - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
pub type EVENT_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVENT_LEVEL` writer - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
pub type EVENT_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INSTR_FF_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLEAR` reader - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSTR_FF_CTL_SPEC, bool, O>;
#[doc = "Field `BLOCK` reader - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
pub type BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK` writer - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
pub type BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSTR_FF_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
    #[inline(always)]
    pub fn event_level(&self) -> EVENT_LEVEL_R {
        EVENT_LEVEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
    #[inline(always)]
    #[must_use]
    pub fn event_level(&mut self) -> EVENT_LEVEL_W<0> {
        EVENT_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<17> {
        BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_ff_ctl](index.html) module"]
pub struct INSTR_FF_CTL_SPEC;
impl crate::RegisterSpec for INSTR_FF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instr_ff_ctl::R](R) reader structure"]
impl crate::Readable for INSTR_FF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [instr_ff_ctl::W](W) writer structure"]
impl crate::Writable for INSTR_FF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTR_FF_CTL to value 0x0002_0000"]
impl crate::Resettable for INSTR_FF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
