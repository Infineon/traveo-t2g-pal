#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT` reader - WDT Interrupt Request. This bit is set as configured by WDT action and limits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to update after a W1C or reading this register and during this time AHB bus is stalled."]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - WDT Interrupt Request. This bit is set as configured by WDT action and limits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to update after a W1C or reading this register and during this time AHB bus is stalled."]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set as configured by WDT action and limits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to update after a W1C or reading this register and during this time AHB bus is stalled."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set as configured by WDT action and limits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to update after a W1C or reading this register and during this time AHB bus is stalled."]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<0> {
        WDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
