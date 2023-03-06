#[doc = "Register `RES_PXRES_CTL` writer"]
pub struct W(crate::W<RES_PXRES_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_PXRES_CTL_SPEC>;
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
impl From<crate::W<RES_PXRES_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_PXRES_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PXRES_TRIGGER` writer - Triggers PXRES. This causes a full-scope reset and reboot."]
pub type PXRES_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_PXRES_CTL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Triggers PXRES. This causes a full-scope reset and reboot."]
    #[inline(always)]
    #[must_use]
    pub fn pxres_trigger(&mut self) -> PXRES_TRIGGER_W<0> {
        PXRES_TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable XRES Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_pxres_ctl](index.html) module"]
pub struct RES_PXRES_CTL_SPEC;
impl crate::RegisterSpec for RES_PXRES_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [res_pxres_ctl::W](W) writer structure"]
impl crate::Writable for RES_PXRES_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES_PXRES_CTL to value 0"]
impl crate::Resettable for RES_PXRES_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
