#[doc = "Register `SOFTWARERESET` writer"]
pub struct W(crate::W<SOFTWARERESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTWARERESET_SPEC>;
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
impl From<crate::W<SOFTWARERESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTWARERESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTWARERESET` writer - Writting a 1 to this field will reset all fifos and control logic of LCDBusIF."]
pub type SOFTWARERESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTWARERESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writting a 1 to this field will reset all fifos and control logic of LCDBusIF."]
    #[inline(always)]
    #[must_use]
    pub fn softwarereset(&mut self) -> SOFTWARERESET_W<0> {
        SOFTWARERESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset Register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softwarereset](index.html) module"]
pub struct SOFTWARERESET_SPEC;
impl crate::RegisterSpec for SOFTWARERESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [softwarereset::W](W) writer structure"]
impl crate::Writable for SOFTWARERESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFTWARERESET to value 0"]
impl crate::Resettable for SOFTWARERESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
