#[doc = "Register `INSTR_FF_WR` writer"]
pub struct W(crate::W<INSTR_FF_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTR_FF_WR_SPEC>;
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
impl From<crate::W<INSTR_FF_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTR_FF_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA32` writer - Instruction or instruction operand data that is written to the instruction FIFO."]
pub type DATA32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INSTR_FF_WR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Instruction or instruction operand data that is written to the instruction FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> DATA32_W<0> {
        DATA32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction FIFO write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instr_ff_wr](index.html) module"]
pub struct INSTR_FF_WR_SPEC;
impl crate::RegisterSpec for INSTR_FF_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [instr_ff_wr::W](W) writer structure"]
impl crate::Writable for INSTR_FF_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTR_FF_WR to value 0"]
impl crate::Resettable for INSTR_FF_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
