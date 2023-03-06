#[doc = "Register `SERVICE` reader"]
pub struct R(crate::R<SERVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERVICE` writer"]
pub struct W(crate::W<SERVICE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERVICE_SPEC>;
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
impl From<crate::W<SERVICE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERVICE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERVICE` reader - Services the watchdog. This resets the count value to zero. This may take up to three clk_ilo0 cycle to take effect. Hardware clears this bit, after necessary synchronization. To ensure a pending SERVICE write is reflected, firmware should wait until this bit reads low before attempting to write SERVICE=1. If WDT is disabled, SERVICE will not trigger a LOWER_ACTION and will not clear a preloaded count value."]
pub type SERVICE_R = crate::BitReader<bool>;
#[doc = "Field `SERVICE` writer - Services the watchdog. This resets the count value to zero. This may take up to three clk_ilo0 cycle to take effect. Hardware clears this bit, after necessary synchronization. To ensure a pending SERVICE write is reflected, firmware should wait until this bit reads low before attempting to write SERVICE=1. If WDT is disabled, SERVICE will not trigger a LOWER_ACTION and will not clear a preloaded count value."]
pub type SERVICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SERVICE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Services the watchdog. This resets the count value to zero. This may take up to three clk_ilo0 cycle to take effect. Hardware clears this bit, after necessary synchronization. To ensure a pending SERVICE write is reflected, firmware should wait until this bit reads low before attempting to write SERVICE=1. If WDT is disabled, SERVICE will not trigger a LOWER_ACTION and will not clear a preloaded count value."]
    #[inline(always)]
    pub fn service(&self) -> SERVICE_R {
        SERVICE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Services the watchdog. This resets the count value to zero. This may take up to three clk_ilo0 cycle to take effect. Hardware clears this bit, after necessary synchronization. To ensure a pending SERVICE write is reflected, firmware should wait until this bit reads low before attempting to write SERVICE=1. If WDT is disabled, SERVICE will not trigger a LOWER_ACTION and will not clear a preloaded count value."]
    #[inline(always)]
    #[must_use]
    pub fn service(&mut self) -> SERVICE_W<0> {
        SERVICE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Service register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [service](index.html) module"]
pub struct SERVICE_SPEC;
impl crate::RegisterSpec for SERVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [service::R](R) reader structure"]
impl crate::Readable for SERVICE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [service::W](W) writer structure"]
impl crate::Writable for SERVICE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SERVICE to value 0"]
impl crate::Resettable for SERVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
