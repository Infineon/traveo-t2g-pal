#[doc = "Register `READCHANNELCONTROL` writer"]
pub struct W(crate::W<READCHANNELCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCHANNELCONTROL_SPEC>;
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
impl From<crate::W<READCHANNELCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCHANNELCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READCHANNELTRIGGER` writer - Initiate read of data via read channel interface."]
pub type READCHANNELTRIGGER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, READCHANNELCONTROL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Initiate read of data via read channel interface."]
    #[inline(always)]
    #[must_use]
    pub fn readchanneltrigger(&mut self) -> READCHANNELTRIGGER_W<0> {
        READCHANNELTRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Channel Control Register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readchannelcontrol](index.html) module"]
pub struct READCHANNELCONTROL_SPEC;
impl crate::RegisterSpec for READCHANNELCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [readchannelcontrol::W](W) writer structure"]
impl crate::Writable for READCHANNELCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCHANNELCONTROL to value 0"]
impl crate::Resettable for READCHANNELCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
