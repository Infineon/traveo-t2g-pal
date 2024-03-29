#[doc = "Register `TS_CNT` reader"]
pub struct R(crate::R<TS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS_CNT` writer"]
pub struct W(crate::W<TS_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS_CNT_SPEC>;
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
impl From<crate::W<TS_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS_CNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The counter value of the Time Stamp Counter. When enabled this counter will count Time Stamp clock ticks from the pre-scaler. When written this counter and the pre-scaler will reset to 0 (write data is ignored)."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Stamp counter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_cnt](index.html) module"]
pub struct TS_CNT_SPEC;
impl crate::RegisterSpec for TS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts_cnt::R](R) reader structure"]
impl crate::Readable for TS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts_cnt::W](W) writer structure"]
impl crate::Writable for TS_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS_CNT to value 0"]
impl crate::Resettable for TS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
