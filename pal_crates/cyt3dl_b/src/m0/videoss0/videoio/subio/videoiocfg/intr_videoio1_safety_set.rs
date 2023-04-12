#[doc = "Register `INTR_VIDEOIO1_SAFETY_SET` writer"]
pub struct W(crate::W<INTR_VIDEOIO1_SAFETY_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_VIDEOIO1_SAFETY_SET_SPEC>;
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
impl From<crate::W<INTR_VIDEOIO1_SAFETY_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_VIDEOIO1_SAFETY_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_VIDEOIO1_SAFETY_SET` writer - Interrupt set vector. Writing a 1 to this fields sets status in INTR_VIDEOIO1_SAFETY."]
pub type INTR_VIDEOIO1_SAFETY_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_VIDEOIO1_SAFETY_SET_SPEC, u32, u32, 28, O>;
impl W {
    #[doc = "Bits 0:27 - Interrupt set vector. Writing a 1 to this fields sets status in INTR_VIDEOIO1_SAFETY."]
    #[inline(always)]
    #[must_use]
    pub fn intr_videoio1_safety_set(&mut self) -> INTR_VIDEOIO1_SAFETY_SET_W<0> {
        INTR_VIDEOIO1_SAFETY_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio1_safety_set](index.html) module"]
pub struct INTR_VIDEOIO1_SAFETY_SET_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO1_SAFETY_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intr_videoio1_safety_set::W](W) writer structure"]
impl crate::Writable for INTR_VIDEOIO1_SAFETY_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_VIDEOIO1_SAFETY_SET to value 0"]
impl crate::Resettable for INTR_VIDEOIO1_SAFETY_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
