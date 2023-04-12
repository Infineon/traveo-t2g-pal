#[doc = "Register `SAR_TR_IN_SEL[%s]` reader"]
pub struct R(crate::R<SAR_TR_IN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TR_IN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TR_IN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TR_IN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TR_IN_SEL[%s]` writer"]
pub struct W(crate::W<SAR_TR_IN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TR_IN_SEL_SPEC>;
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
impl From<crate::W<SAR_TR_IN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TR_IN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN0_SEL` reader - Select generic trigger for SAR generic trigger input 0"]
pub type IN0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN0_SEL` writer - Select generic trigger for SAR generic trigger input 0"]
pub type IN0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_TR_IN_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IN1_SEL` reader - Select generic trigger for SAR generic trigger input 1"]
pub type IN1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN1_SEL` writer - Select generic trigger for SAR generic trigger input 1"]
pub type IN1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_TR_IN_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IN2_SEL` reader - Select generic trigger for SAR generic trigger input 2"]
pub type IN2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN2_SEL` writer - Select generic trigger for SAR generic trigger input 2"]
pub type IN2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_TR_IN_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IN3_SEL` reader - Select generic trigger for SAR generic trigger input 3"]
pub type IN3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN3_SEL` writer - Select generic trigger for SAR generic trigger input 3"]
pub type IN3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_TR_IN_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IN4_SEL` reader - Select generic trigger for SAR generic trigger input 4"]
pub type IN4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN4_SEL` writer - Select generic trigger for SAR generic trigger input 4"]
pub type IN4_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAR_TR_IN_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Select generic trigger for SAR generic trigger input 0"]
    #[inline(always)]
    pub fn in0_sel(&self) -> IN0_SEL_R {
        IN0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select generic trigger for SAR generic trigger input 1"]
    #[inline(always)]
    pub fn in1_sel(&self) -> IN1_SEL_R {
        IN1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select generic trigger for SAR generic trigger input 2"]
    #[inline(always)]
    pub fn in2_sel(&self) -> IN2_SEL_R {
        IN2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select generic trigger for SAR generic trigger input 3"]
    #[inline(always)]
    pub fn in3_sel(&self) -> IN3_SEL_R {
        IN3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select generic trigger for SAR generic trigger input 4"]
    #[inline(always)]
    pub fn in4_sel(&self) -> IN4_SEL_R {
        IN4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select generic trigger for SAR generic trigger input 0"]
    #[inline(always)]
    #[must_use]
    pub fn in0_sel(&mut self) -> IN0_SEL_W<0> {
        IN0_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Select generic trigger for SAR generic trigger input 1"]
    #[inline(always)]
    #[must_use]
    pub fn in1_sel(&mut self) -> IN1_SEL_W<4> {
        IN1_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Select generic trigger for SAR generic trigger input 2"]
    #[inline(always)]
    #[must_use]
    pub fn in2_sel(&mut self) -> IN2_SEL_W<8> {
        IN2_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Select generic trigger for SAR generic trigger input 3"]
    #[inline(always)]
    #[must_use]
    pub fn in3_sel(&mut self) -> IN3_SEL_W<12> {
        IN3_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Select generic trigger for SAR generic trigger input 4"]
    #[inline(always)]
    #[must_use]
    pub fn in4_sel(&mut self) -> IN4_SEL_W<16> {
        IN4_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "per SAR generic input trigger select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tr_in_sel](index.html) module"]
pub struct SAR_TR_IN_SEL_SPEC;
impl crate::RegisterSpec for SAR_TR_IN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tr_in_sel::R](R) reader structure"]
impl crate::Readable for SAR_TR_IN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tr_in_sel::W](W) writer structure"]
impl crate::Writable for SAR_TR_IN_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TR_IN_SEL[%s]
to value 0x0004_3210"]
impl crate::Resettable for SAR_TR_IN_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_3210;
}
