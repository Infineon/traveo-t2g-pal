#[doc = "Register `LINE_SEL_BUFF` reader"]
pub struct R(crate::R<LINE_SEL_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE_SEL_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE_SEL_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE_SEL_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE_SEL_BUFF` writer"]
pub struct W(crate::W<LINE_SEL_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE_SEL_BUFF_SPEC>;
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
impl From<crate::W<LINE_SEL_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE_SEL_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SEL` reader - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
pub type OUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_SEL` writer - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
pub type OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LINE_SEL_BUFF_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMPL_OUT_SEL` reader - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
pub type COMPL_OUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPL_OUT_SEL` writer - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
pub type COMPL_OUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LINE_SEL_BUFF_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    pub fn compl_out_sel(&self) -> COMPL_OUT_SEL_R {
        COMPL_OUT_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Buffer for LINE_SEL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<0> {
        OUT_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Buffer for LINE_SEL.COMPL.OUT_SEL. Can be exchanged with LINE_SEL.LINE_COMPL_OUT_SEL on a terminal count event with an actively pending switch event. This field has a function in PWM and PWM_PR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn compl_out_sel(&mut self) -> COMPL_OUT_SEL_W<4> {
        COMPL_OUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter buffered line selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line_sel_buff](index.html) module"]
pub struct LINE_SEL_BUFF_SPEC;
impl crate::RegisterSpec for LINE_SEL_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line_sel_buff::R](R) reader structure"]
impl crate::Readable for LINE_SEL_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line_sel_buff::W](W) writer structure"]
impl crate::Writable for LINE_SEL_BUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINE_SEL_BUFF to value 0x32"]
impl crate::Resettable for LINE_SEL_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0x32;
}
