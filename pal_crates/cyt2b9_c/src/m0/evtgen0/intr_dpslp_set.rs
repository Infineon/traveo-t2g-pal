#[doc = "Register `INTR_DPSLP_SET` reader"]
pub struct R(crate::R<INTR_DPSLP_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DPSLP_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DPSLP_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DPSLP_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_DPSLP_SET` writer"]
pub struct W(crate::W<INTR_DPSLP_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_DPSLP_SET_SPEC>;
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
impl From<crate::W<INTR_DPSLP_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_DPSLP_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type COMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMP1` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type COMP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_DPSLP_SET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<0> {
        COMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeepSleep interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dpslp_set](index.html) module"]
pub struct INTR_DPSLP_SET_SPEC;
impl crate::RegisterSpec for INTR_DPSLP_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dpslp_set::R](R) reader structure"]
impl crate::Readable for INTR_DPSLP_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_dpslp_set::W](W) writer structure"]
impl crate::Writable for INTR_DPSLP_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_DPSLP_SET to value 0"]
impl crate::Resettable for INTR_DPSLP_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
