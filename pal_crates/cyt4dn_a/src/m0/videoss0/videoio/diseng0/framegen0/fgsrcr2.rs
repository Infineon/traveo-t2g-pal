#[doc = "Register `FGSRCR2` reader"]
pub struct R(crate::R<FGSRCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR2` writer"]
pub struct W(crate::W<FGSRCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR2_SPEC>;
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
impl From<crate::W<FGSRCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTOTALMIN` reader - Minimum value of htotal when horizontal regulation is enabled. (Set value +1 is the minimum value of htotal, must be greater or equal Hsbp + Hactive) HTotalMin must be smaller or equal Htotal. Note: If SRAdj==1 then HtotalMin\\[0\\]
shall be equal to the value of SREven."]
pub type HTOTALMIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HTOTALMIN` writer - Minimum value of htotal when horizontal regulation is enabled. (Set value +1 is the minimum value of htotal, must be greater or equal Hsbp + Hactive) HTotalMin must be smaller or equal Htotal. Note: If SRAdj==1 then HtotalMin\\[0\\]
shall be equal to the value of SREven."]
pub type HTOTALMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGSRCR2_SPEC, u16, u16, 14, O>;
#[doc = "Field `HTOTALMAX` reader - Maximum value of htotal when horizontal regulation is enabled. (Set value +1 is the maximum value of htotal) HTotalMax must be greater or equal Htotal. Note: If SRAdj==1 then HtotalMax\\[0\\]
shall be equal to the value of SREven."]
pub type HTOTALMAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HTOTALMAX` writer - Maximum value of htotal when horizontal regulation is enabled. (Set value +1 is the maximum value of htotal) HTotalMax must be greater or equal Htotal. Note: If SRAdj==1 then HtotalMax\\[0\\]
shall be equal to the value of SREven."]
pub type HTOTALMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGSRCR2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Minimum value of htotal when horizontal regulation is enabled. (Set value +1 is the minimum value of htotal, must be greater or equal Hsbp + Hactive) HTotalMin must be smaller or equal Htotal. Note: If SRAdj==1 then HtotalMin\\[0\\]
shall be equal to the value of SREven."]
    #[inline(always)]
    pub fn htotalmin(&self) -> HTOTALMIN_R {
        HTOTALMIN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Maximum value of htotal when horizontal regulation is enabled. (Set value +1 is the maximum value of htotal) HTotalMax must be greater or equal Htotal. Note: If SRAdj==1 then HtotalMax\\[0\\]
shall be equal to the value of SREven."]
    #[inline(always)]
    pub fn htotalmax(&self) -> HTOTALMAX_R {
        HTOTALMAX_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Minimum value of htotal when horizontal regulation is enabled. (Set value +1 is the minimum value of htotal, must be greater or equal Hsbp + Hactive) HTotalMin must be smaller or equal Htotal. Note: If SRAdj==1 then HtotalMin\\[0\\]
shall be equal to the value of SREven."]
    #[inline(always)]
    #[must_use]
    pub fn htotalmin(&mut self) -> HTOTALMIN_W<0> {
        HTOTALMIN_W::new(self)
    }
    #[doc = "Bits 16:29 - Maximum value of htotal when horizontal regulation is enabled. (Set value +1 is the maximum value of htotal) HTotalMax must be greater or equal Htotal. Note: If SRAdj==1 then HtotalMax\\[0\\]
shall be equal to the value of SREven."]
    #[inline(always)]
    #[must_use]
    pub fn htotalmax(&mut self) -> HTOTALMAX_W<16> {
        HTOTALMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr2](index.html) module"]
pub struct FGSRCR2_SPEC;
impl crate::RegisterSpec for FGSRCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr2::R](R) reader structure"]
impl crate::Readable for FGSRCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr2::W](W) writer structure"]
impl crate::Writable for FGSRCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR2 to value 0x01b7_0188"]
impl crate::Resettable for FGSRCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01b7_0188;
}
