#[doc = "Register `FGCHSTATCLR` writer"]
pub struct W(crate::W<FGCHSTATCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCHSTATCLR_SPEC>;
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
impl From<crate::W<FGCHSTATCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCHSTATCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRPRIMSTAT` writer - Clears PFifoEmpty in FgChStat register."]
pub type CLRPRIMSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGCHSTATCLR_SPEC, bool, O>;
#[doc = "Field `CLRSECSTAT` writer - Clears SFifoEmpty and SkewRangeErr in FgChStat register."]
pub type CLRSECSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGCHSTATCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clears PFifoEmpty in FgChStat register."]
    #[inline(always)]
    #[must_use]
    pub fn clrprimstat(&mut self) -> CLRPRIMSTAT_W<0> {
        CLRPRIMSTAT_W::new(self)
    }
    #[doc = "Bit 16 - Clears SFifoEmpty and SkewRangeErr in FgChStat register."]
    #[inline(always)]
    #[must_use]
    pub fn clrsecstat(&mut self) -> CLRSECSTAT_W<16> {
        CLRSECSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Channel Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgchstatclr](index.html) module"]
pub struct FGCHSTATCLR_SPEC;
impl crate::RegisterSpec for FGCHSTATCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fgchstatclr::W](W) writer structure"]
impl crate::Writable for FGCHSTATCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGCHSTATCLR to value 0"]
impl crate::Resettable for FGCHSTATCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
