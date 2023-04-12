#[doc = "Register `LOCKUNLOCKCONTROL` reader"]
pub struct R(crate::R<LOCKUNLOCKCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKUNLOCKCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKUNLOCKCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKUNLOCKCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKUNLOCKCONTROL` writer"]
pub struct W(crate::W<LOCKUNLOCKCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKUNLOCKCONTROL_SPEC>;
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
impl From<crate::W<LOCKUNLOCKCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKUNLOCKCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUNLOCKCONTROL` reader - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKCONTROL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCKUNLOCKCONTROL` writer - The protection status is changed by writing one of the following key values to this field:"]
pub type LOCKUNLOCKCONTROL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCKUNLOCKCONTROL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    pub fn lockunlockcontrol(&self) -> LOCKUNLOCKCONTROL_R {
        LOCKUNLOCKCONTROL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The protection status is changed by writing one of the following key values to this field:"]
    #[inline(always)]
    #[must_use]
    pub fn lockunlockcontrol(&mut self) -> LOCKUNLOCKCONTROL_W<0> {
        LOCKUNLOCKCONTROL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to change the protection status of this address block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockunlockcontrol](index.html) module"]
pub struct LOCKUNLOCKCONTROL_SPEC;
impl crate::RegisterSpec for LOCKUNLOCKCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockunlockcontrol::R](R) reader structure"]
impl crate::Readable for LOCKUNLOCKCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockunlockcontrol::W](W) writer structure"]
impl crate::Writable for LOCKUNLOCKCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKUNLOCKCONTROL to value 0"]
impl crate::Resettable for LOCKUNLOCKCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
