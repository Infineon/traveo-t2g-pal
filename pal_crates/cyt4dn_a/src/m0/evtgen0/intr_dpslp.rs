#[doc = "Register `INTR_DPSLP` reader"]
pub struct R(crate::R<INTR_DPSLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DPSLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DPSLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DPSLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_DPSLP` writer"]
pub struct W(crate::W<INTR_DPSLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_DPSLP_SPEC>;
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
impl From<crate::W<INTR_DPSLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_DPSLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1` reader - This interrupt cause field is activated (HW sets the field to '1') when a comparator 1 event is generated (DeepSleep counter 'counter_int_lf\\[31:0\\]' becomes greater or equal to COMP1.INT\\[31:0\\])."]
pub type COMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMP1` writer - This interrupt cause field is activated (HW sets the field to '1') when a comparator 1 event is generated (DeepSleep counter 'counter_int_lf\\[31:0\\]' becomes greater or equal to COMP1.INT\\[31:0\\])."]
pub type COMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_DPSLP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This interrupt cause field is activated (HW sets the field to '1') when a comparator 1 event is generated (DeepSleep counter 'counter_int_lf\\[31:0\\]' becomes greater or equal to COMP1.INT\\[31:0\\])."]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This interrupt cause field is activated (HW sets the field to '1') when a comparator 1 event is generated (DeepSleep counter 'counter_int_lf\\[31:0\\]' becomes greater or equal to COMP1.INT\\[31:0\\])."]
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
#[doc = "DeepSleep interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dpslp](index.html) module"]
pub struct INTR_DPSLP_SPEC;
impl crate::RegisterSpec for INTR_DPSLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dpslp::R](R) reader structure"]
impl crate::Readable for INTR_DPSLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_dpslp::W](W) writer structure"]
impl crate::Writable for INTR_DPSLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_DPSLP to value 0"]
impl crate::Resettable for INTR_DPSLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
