#[doc = "Register `RSDSINVCTRL` reader"]
pub struct R(crate::R<RSDSINVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSDSINVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSDSINVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSDSINVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSDSINVCTRL` writer"]
pub struct W(crate::W<RSDSINVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSDSINVCTRL_SPEC>;
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
impl From<crate::W<RSDSINVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSDSINVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSDS_INV` reader - Inversion vector for 1st channel. For i in \\[ 0 .. 11 \\]; if RSDS_Inv \\[ i \\]
== 0 => NON-Inversion of RSDS \\[ i \\]
else Inversion of RSDS \\[ i \\]"]
pub type RSDS_INV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSDS_INV` writer - Inversion vector for 1st channel. For i in \\[ 0 .. 11 \\]; if RSDS_Inv \\[ i \\]
== 0 => NON-Inversion of RSDS \\[ i \\]
else Inversion of RSDS \\[ i \\]"]
pub type RSDS_INV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RSDSINVCTRL_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSDS_INV_DUAL` reader - Same as RSDS_inv for 2nd channel"]
pub type RSDS_INV_DUAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSDS_INV_DUAL` writer - Same as RSDS_inv for 2nd channel"]
pub type RSDS_INV_DUAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RSDSINVCTRL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Inversion vector for 1st channel. For i in \\[ 0 .. 11 \\]; if RSDS_Inv \\[ i \\]
== 0 => NON-Inversion of RSDS \\[ i \\]
else Inversion of RSDS \\[ i \\]"]
    #[inline(always)]
    pub fn rsds_inv(&self) -> RSDS_INV_R {
        RSDS_INV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Same as RSDS_inv for 2nd channel"]
    #[inline(always)]
    pub fn rsds_inv_dual(&self) -> RSDS_INV_DUAL_R {
        RSDS_INV_DUAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Inversion vector for 1st channel. For i in \\[ 0 .. 11 \\]; if RSDS_Inv \\[ i \\]
== 0 => NON-Inversion of RSDS \\[ i \\]
else Inversion of RSDS \\[ i \\]"]
    #[inline(always)]
    #[must_use]
    pub fn rsds_inv(&mut self) -> RSDS_INV_W<0> {
        RSDS_INV_W::new(self)
    }
    #[doc = "Bits 16:27 - Same as RSDS_inv for 2nd channel"]
    #[inline(always)]
    #[must_use]
    pub fn rsds_inv_dual(&mut self) -> RSDS_INV_DUAL_W<16> {
        RSDS_INV_DUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls inversion of output polarity when connected IO cells operate in RSDS mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsdsinvctrl](index.html) module"]
pub struct RSDSINVCTRL_SPEC;
impl crate::RegisterSpec for RSDSINVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsdsinvctrl::R](R) reader structure"]
impl crate::Readable for RSDSINVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsdsinvctrl::W](W) writer structure"]
impl crate::Writable for RSDSINVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSDSINVCTRL to value 0"]
impl crate::Resettable for RSDSINVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
