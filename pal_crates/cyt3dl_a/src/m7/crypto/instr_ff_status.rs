#[doc = "Register `INSTR_FF_STATUS` reader"]
pub struct R(crate::R<INSTR_FF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTR_FF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTR_FF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTR_FF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of instructions in the instruction FIFO. The value of this field ranges from 0 to 8."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVENT` reader - Instruction FIFO event."]
pub type EVENT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Number of instructions in the instruction FIFO. The value of this field ranges from 0 to 8."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Instruction FIFO event."]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Instruction FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_ff_status](index.html) module"]
pub struct INSTR_FF_STATUS_SPEC;
impl crate::RegisterSpec for INSTR_FF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instr_ff_status::R](R) reader structure"]
impl crate::Readable for INSTR_FF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INSTR_FF_STATUS to value 0"]
impl crate::Resettable for INSTR_FF_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
