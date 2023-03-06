#[doc = "Register `MON_CTL` reader"]
pub struct R(crate::R<MON_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MON_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MON_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MON_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MON_CTL` writer"]
pub struct W(crate::W<MON_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MON_CTL_SPEC>;
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
impl From<crate::W<MON_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MON_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Period time. Set the Period -1, in monitored clock cycles, before the next monitored clock event happens. PERIOD &lt;= (UPPER+1) / FREQ_RATIO -1, with FREQ_RATIO = (Reference frequency / Monitored frequency) In case the clocks are asynchronous: PERIOD &lt;= UPPER / FREQ_RATIO -1 Additionally margin must be added for accuracy of both clocks."]
pub type PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD` writer - Period time. Set the Period -1, in monitored clock cycles, before the next monitored clock event happens. PERIOD &lt;= (UPPER+1) / FREQ_RATIO -1, with FREQ_RATIO = (Reference frequency / Monitored frequency) In case the clocks are asynchronous: PERIOD &lt;= UPPER / FREQ_RATIO -1 Additionally margin must be added for accuracy of both clocks."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MON_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Period time. Set the Period -1, in monitored clock cycles, before the next monitored clock event happens. PERIOD &lt;= (UPPER+1) / FREQ_RATIO -1, with FREQ_RATIO = (Reference frequency / Monitored frequency) In case the clocks are asynchronous: PERIOD &lt;= UPPER / FREQ_RATIO -1 Additionally margin must be added for accuracy of both clocks."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period time. Set the Period -1, in monitored clock cycles, before the next monitored clock event happens. PERIOD &lt;= (UPPER+1) / FREQ_RATIO -1, with FREQ_RATIO = (Reference frequency / Monitored frequency) In case the clocks are asynchronous: PERIOD &lt;= UPPER / FREQ_RATIO -1 Additionally margin must be added for accuracy of both clocks."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Supervision Monitor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mon_ctl](index.html) module"]
pub struct MON_CTL_SPEC;
impl crate::RegisterSpec for MON_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mon_ctl::R](R) reader structure"]
impl crate::Readable for MON_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mon_ctl::W](W) writer structure"]
impl crate::Writable for MON_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MON_CTL to value 0"]
impl crate::Resettable for MON_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
