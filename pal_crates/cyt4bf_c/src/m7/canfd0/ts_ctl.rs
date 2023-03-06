#[doc = "Register `TS_CTL` reader"]
pub struct R(crate::R<TS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS_CTL` writer"]
pub struct W(crate::W<TS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS_CTL_SPEC>;
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
impl From<crate::W<TS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALE` reader - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
pub type PRESCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESCALE` writer - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
pub type PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TS_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `ENABLED` reader - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, TS_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Stamp counter prescale value. When enabled divide the Host clock (HCLK) by PRESCALE+1 to create Time Stamp clock ticks."]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<0> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bit 31 - Counter enable bit 0 = Count disabled. Stop counting up and keep the counter value 1 = Count enabled. Start counting up from the current value"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_ctl](index.html) module"]
pub struct TS_CTL_SPEC;
impl crate::RegisterSpec for TS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts_ctl::R](R) reader structure"]
impl crate::Readable for TS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts_ctl::W](W) writer structure"]
impl crate::Writable for TS_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TS_CTL to value 0"]
impl crate::Resettable for TS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
