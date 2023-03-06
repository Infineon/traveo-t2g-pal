#[doc = "Register `HTCFG1` reader"]
pub struct R(crate::R<HTCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCFG1` writer"]
pub struct W(crate::W<HTCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCFG1_SPEC>;
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
impl From<crate::W<HTCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HACT` reader - Horizontal size of active display area in pixels. Hact shall be greater or equal than 4. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HACT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HACT` writer - Horizontal size of active display area in pixels. Hact shall be greater or equal than 4. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
pub type HACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCFG1_SPEC, u16, u16, 14, O>;
#[doc = "Field `HTOTAL` reader - Total horizontal size of frame in pixels. (Set value +1 is the total horizontal size of frame) Htotal shall be greater or equal than Hact + Hsbp. Note1: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync. Note2: If SRAdj==1 then Htotal\\[0\\]
shall be equal the value of SREven (for sec. channel only)."]
pub type HTOTAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HTOTAL` writer - Total horizontal size of frame in pixels. (Set value +1 is the total horizontal size of frame) Htotal shall be greater or equal than Hact + Hsbp. Note1: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync. Note2: If SRAdj==1 then Htotal\\[0\\]
shall be equal the value of SREven (for sec. channel only)."]
pub type HTOTAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCFG1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Horizontal size of active display area in pixels. Hact shall be greater or equal than 4. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    pub fn hact(&self) -> HACT_R {
        HACT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Total horizontal size of frame in pixels. (Set value +1 is the total horizontal size of frame) Htotal shall be greater or equal than Hact + Hsbp. Note1: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync. Note2: If SRAdj==1 then Htotal\\[0\\]
shall be equal the value of SREven (for sec. channel only)."]
    #[inline(always)]
    pub fn htotal(&self) -> HTOTAL_R {
        HTOTAL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Horizontal size of active display area in pixels. Hact shall be greater or equal than 4. Note: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync."]
    #[inline(always)]
    #[must_use]
    pub fn hact(&mut self) -> HACT_W<0> {
        HACT_W::new(self)
    }
    #[doc = "Bits 16:29 - Total horizontal size of frame in pixels. (Set value +1 is the total horizontal size of frame) Htotal shall be greater or equal than Hact + Hsbp. Note1: Valid setup for horizontal video timing contains parameters Hact, Htotal, Hsbp and, if enabled, Hsync. Note2: If SRAdj==1 then Htotal\\[0\\]
shall be equal the value of SREven (for sec. channel only)."]
    #[inline(always)]
    #[must_use]
    pub fn htotal(&mut self) -> HTOTAL_W<16> {
        HTOTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Horizontal Timing Config Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcfg1](index.html) module"]
pub struct HTCFG1_SPEC;
impl crate::RegisterSpec for HTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcfg1::R](R) reader structure"]
impl crate::Readable for HTCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcfg1::W](W) writer structure"]
impl crate::Writable for HTCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTCFG1 to value 0x018f_0140"]
impl crate::Resettable for HTCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x018f_0140;
}
