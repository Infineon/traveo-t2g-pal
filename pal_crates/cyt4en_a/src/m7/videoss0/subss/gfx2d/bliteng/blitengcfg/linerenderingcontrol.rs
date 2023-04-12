#[doc = "Register `LINERENDERINGCONTROL` writer"]
pub struct W(crate::W<LINERENDERINGCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINERENDERINGCONTROL_SPEC>;
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
impl From<crate::W<LINERENDERINGCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINERENDERINGCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRSTART` writer - Start next source to linebuffer operation."]
pub type LRSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINERENDERINGCONTROL_SPEC, bool, O>;
#[doc = "Field `CLEARTASK` writer - Clear related registers (ypos, perf) of corresponding TB. Each bit corresponds to one TB."]
pub type CLEARTASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LINERENDERINGCONTROL_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bit 0 - Start next source to linebuffer operation."]
    #[inline(always)]
    #[must_use]
    pub fn lrstart(&mut self) -> LRSTART_W<0> {
        LRSTART_W::new(self)
    }
    #[doc = "Bits 8:15 - Clear related registers (ypos, perf) of corresponding TB. Each bit corresponds to one TB."]
    #[inline(always)]
    #[must_use]
    pub fn cleartask(&mut self) -> CLEARTASK_W<8> {
        CLEARTASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line rendering operation control register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linerenderingcontrol](index.html) module"]
pub struct LINERENDERINGCONTROL_SPEC;
impl crate::RegisterSpec for LINERENDERINGCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [linerenderingcontrol::W](W) writer structure"]
impl crate::Writable for LINERENDERINGCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINERENDERINGCONTROL to value 0"]
impl crate::Resettable for LINERENDERINGCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
