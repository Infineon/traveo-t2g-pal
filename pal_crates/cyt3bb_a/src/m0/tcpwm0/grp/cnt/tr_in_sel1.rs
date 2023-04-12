#[doc = "Register `TR_IN_SEL1` reader"]
pub struct R(crate::R<TR_IN_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_IN_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_IN_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_IN_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_IN_SEL1` writer"]
pub struct W(crate::W<TR_IN_SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_IN_SEL1_SPEC>;
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
impl From<crate::W<TR_IN_SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_IN_SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_SEL` reader - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub type START_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `START_SEL` writer - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub type START_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_IN_SEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CAPTURE1_SEL` reader - Selects one of the 256 input triggers as a capture 1 trigger."]
pub type CAPTURE1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPTURE1_SEL` writer - Selects one of the 256 input triggers as a capture 1 trigger."]
pub type CAPTURE1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_IN_SEL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&self) -> START_SEL_R {
        START_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a capture 1 trigger."]
    #[inline(always)]
    pub fn capture1_sel(&self) -> CAPTURE1_SEL_R {
        CAPTURE1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects one of the 256 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    #[must_use]
    pub fn start_sel(&mut self) -> START_SEL_W<0> {
        START_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Selects one of the 256 input triggers as a capture 1 trigger."]
    #[inline(always)]
    #[must_use]
    pub fn capture1_sel(&mut self) -> CAPTURE1_SEL_W<8> {
        CAPTURE1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter input trigger selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_sel1](index.html) module"]
pub struct TR_IN_SEL1_SPEC;
impl crate::RegisterSpec for TR_IN_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_in_sel1::R](R) reader structure"]
impl crate::Readable for TR_IN_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_in_sel1::W](W) writer structure"]
impl crate::Writable for TR_IN_SEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_IN_SEL1 to value 0"]
impl crate::Resettable for TR_IN_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
