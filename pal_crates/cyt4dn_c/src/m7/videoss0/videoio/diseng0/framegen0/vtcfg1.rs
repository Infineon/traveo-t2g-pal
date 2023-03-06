#[doc = "Register `VTCFG1` reader"]
pub struct R(crate::R<VTCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTCFG1` writer"]
pub struct W(crate::W<VTCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTCFG1_SPEC>;
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
impl From<crate::W<VTCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VACT` reader - Vertical size of active display area in lines. Vact shall be greater or equal than 4. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VACT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VACT` writer - Vertical size of active display area in lines. Vact shall be greater or equal than 4. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTCFG1_SPEC, u16, u16, 14, O>;
#[doc = "Field `VTOTAL` reader - Total vertical size of frame in lines. (Set value +1 is the total vertical size of frame) Vtotal shall be greater or equal than Vact + Vsbp. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VTOTAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VTOTAL` writer - Total vertical size of frame in lines. (Set value +1 is the total vertical size of frame) Vtotal shall be greater or equal than Vact + Vsbp. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
pub type VTOTAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTCFG1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Vertical size of active display area in lines. Vact shall be greater or equal than 4. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    pub fn vact(&self) -> VACT_R {
        VACT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Total vertical size of frame in lines. (Set value +1 is the total vertical size of frame) Vtotal shall be greater or equal than Vact + Vsbp. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    pub fn vtotal(&self) -> VTOTAL_R {
        VTOTAL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Vertical size of active display area in lines. Vact shall be greater or equal than 4. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    #[must_use]
    pub fn vact(&mut self) -> VACT_W<0> {
        VACT_W::new(self)
    }
    #[doc = "Bits 16:29 - Total vertical size of frame in lines. (Set value +1 is the total vertical size of frame) Vtotal shall be greater or equal than Vact + Vsbp. Note: Valid setup for vertical video timing contains parameters Vact, Vtotal, Vsbp and, if enabled, Vsync."]
    #[inline(always)]
    #[must_use]
    pub fn vtotal(&mut self) -> VTOTAL_W<16> {
        VTOTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Vertical Timing Config Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtcfg1](index.html) module"]
pub struct VTCFG1_SPEC;
impl crate::RegisterSpec for VTCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vtcfg1::R](R) reader structure"]
impl crate::Readable for VTCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtcfg1::W](W) writer structure"]
impl crate::Writable for VTCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTCFG1 to value 0x00fc_00f0"]
impl crate::Resettable for VTCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00fc_00f0;
}
