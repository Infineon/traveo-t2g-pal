#[doc = "Register `FGSRCR3` reader"]
pub struct R(crate::R<FGSRCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR3` writer"]
pub struct W(crate::W<FGSRCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR3_SPEC>;
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
impl From<crate::W<FGSRCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTOTALMIN` reader - Minimum value of vtotal when vertical regulation is enabled. (Set value +1 is the minimum value of vtotal, must be greater or equal Vsbp + Vactive + 1)"]
pub type VTOTALMIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTOTALMIN` writer - Minimum value of vtotal when vertical regulation is enabled. (Set value +1 is the minimum value of vtotal, must be greater or equal Vsbp + Vactive + 1)"]
pub type VTOTALMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGSRCR3_SPEC, u16, u16, 14, O>;
#[doc = "Field `VTOTALMAX` reader - Maximum value of vtotal when vertical regulation is enabled. (Set value +1 is the maximum value of vtotal)"]
pub type VTOTALMAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTOTALMAX` writer - Maximum value of vtotal when vertical regulation is enabled. (Set value +1 is the maximum value of vtotal)"]
pub type VTOTALMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGSRCR3_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Minimum value of vtotal when vertical regulation is enabled. (Set value +1 is the minimum value of vtotal, must be greater or equal Vsbp + Vactive + 1)"]
    #[inline(always)]
    pub fn vtotalmin(&self) -> VTOTALMIN_R {
        VTOTALMIN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Maximum value of vtotal when vertical regulation is enabled. (Set value +1 is the maximum value of vtotal)"]
    #[inline(always)]
    pub fn vtotalmax(&self) -> VTOTALMAX_R {
        VTOTALMAX_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Minimum value of vtotal when vertical regulation is enabled. (Set value +1 is the minimum value of vtotal, must be greater or equal Vsbp + Vactive + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn vtotalmin(&mut self) -> VTOTALMIN_W<0> {
        VTOTALMIN_W::new(self)
    }
    #[doc = "Bits 16:29 - Maximum value of vtotal when vertical regulation is enabled. (Set value +1 is the maximum value of vtotal)"]
    #[inline(always)]
    #[must_use]
    pub fn vtotalmax(&mut self) -> VTOTALMAX_W<16> {
        VTOTALMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr3](index.html) module"]
pub struct FGSRCR3_SPEC;
impl crate::RegisterSpec for FGSRCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr3::R](R) reader structure"]
impl crate::Readable for FGSRCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr3::W](W) writer structure"]
impl crate::Writable for FGSRCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR3 to value 0x0115_00fb"]
impl crate::Resettable for FGSRCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0115_00fb;
}
