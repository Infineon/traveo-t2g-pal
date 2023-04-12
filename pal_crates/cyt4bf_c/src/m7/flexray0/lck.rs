#[doc = "Register `LCK` writer"]
pub struct W(crate::W<LCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCK_SPEC>;
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
impl From<crate::W<LCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK` writer - Configuration Lock Key To leave CONFIG state by writing SUCC1.CMD\\[3:0\\]
(commands READY, MONITOR_MODE, ATM, LOOP_BACK), the write operation has to be directly preceded by two write accesses to the Configuration Lock Key (unlock sequence). If the write sequence below is interrupted by other write accesses between the second write to the Configuration Lock Key and the write access to the SUCC1 register, the CC remains in CONFIG state and the sequence has to be repeated. First write: LCK.CLK\\[7:0\\]
= '1100 1110' (0xCE) Second write: LCK.CLK\\[7:0\\]
= '0011 0001' (0x31) Third write: SUCC1.CMD\\[3:0\\]"]
pub type CLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `TMK` writer - Test Mode Key To write bit TEST1.WRTEN, the write operation has to be directly preceded by two consecutive write accesses to the Test Mode Key (unlock sequence). If the write sequence below is interrupted by other write accesses between the second write to the Test Mode Key and the write access to the TEST1 register, TEST1.WRTEN is not set to '1' and the sequence has to be repeated. First write: LCK.TMK\\[7:0\\]
= '0111 0101' (0x75) Second write: LCK.TMK\\[7:0\\]
= '1000 1010' (0x8A) Third write: TEST1.WRTEN = '1'"]
pub type TMK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCK_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Configuration Lock Key To leave CONFIG state by writing SUCC1.CMD\\[3:0\\]
(commands READY, MONITOR_MODE, ATM, LOOP_BACK), the write operation has to be directly preceded by two write accesses to the Configuration Lock Key (unlock sequence). If the write sequence below is interrupted by other write accesses between the second write to the Configuration Lock Key and the write access to the SUCC1 register, the CC remains in CONFIG state and the sequence has to be repeated. First write: LCK.CLK\\[7:0\\]
= '1100 1110' (0xCE) Second write: LCK.CLK\\[7:0\\]
= '0011 0001' (0x31) Third write: SUCC1.CMD\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<0> {
        CLK_W::new(self)
    }
    #[doc = "Bits 8:15 - Test Mode Key To write bit TEST1.WRTEN, the write operation has to be directly preceded by two consecutive write accesses to the Test Mode Key (unlock sequence). If the write sequence below is interrupted by other write accesses between the second write to the Test Mode Key and the write access to the TEST1 register, TEST1.WRTEN is not set to '1' and the sequence has to be repeated. First write: LCK.TMK\\[7:0\\]
= '0111 0101' (0x75) Second write: LCK.TMK\\[7:0\\]
= '1000 1010' (0x8A) Third write: TEST1.WRTEN = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn tmk(&mut self) -> TMK_W<8> {
        TMK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lck](index.html) module"]
pub struct LCK_SPEC;
impl crate::RegisterSpec for LCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lck::W](W) writer structure"]
impl crate::Writable for LCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCK to value 0"]
impl crate::Resettable for LCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
