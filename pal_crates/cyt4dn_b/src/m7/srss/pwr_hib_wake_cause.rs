#[doc = "Register `PWR_HIB_WAKE_CAUSE` reader"]
pub struct R(crate::R<PWR_HIB_WAKE_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_HIB_WAKE_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_HIB_WAKE_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_HIB_WAKE_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_HIB_WAKE_CAUSE` writer"]
pub struct W(crate::W<PWR_HIB_WAKE_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_HIB_WAKE_CAUSE_SPEC>;
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
impl From<crate::W<PWR_HIB_WAKE_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_HIB_WAKE_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_WAKE_SRC` reader - Each bit indicates a HIBERNATE wakeup cause. The number and assigment of wakeup sources are product-specific. For each bit, writing a 1 clears the cause flag."]
pub type HIB_WAKE_SRC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_WAKE_SRC` writer - Each bit indicates a HIBERNATE wakeup cause. The number and assigment of wakeup sources are product-specific. For each bit, writing a 1 clears the cause flag."]
pub type HIB_WAKE_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_HIB_WAKE_CAUSE_SPEC, u32, u32, 24, O>;
#[doc = "Field `HIB_WAKE_CSV_BAK` reader - Indicates CSV_BAK wakeup cause. The related fault must be handled before this bit can be cleared."]
pub type HIB_WAKE_CSV_BAK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_WAKE_CSV_BAK` writer - Indicates CSV_BAK wakeup cause. The related fault must be handled before this bit can be cleared."]
pub type HIB_WAKE_CSV_BAK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_HIB_WAKE_CAUSE_SPEC, bool, O>;
#[doc = "Field `HIB_WAKE_RTC` reader - Indicates RTC wakeup cause. The RTC interrupt must be cleared before this bit can be cleared."]
pub type HIB_WAKE_RTC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_WAKE_RTC` writer - Indicates RTC wakeup cause. The RTC interrupt must be cleared before this bit can be cleared."]
pub type HIB_WAKE_RTC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_HIB_WAKE_CAUSE_SPEC, bool, O>;
#[doc = "Field `HIB_WAKE_WDT` reader - Indicates WDT wakeup cause. The WDT interrupt must be cleared before this bit can be cleared."]
pub type HIB_WAKE_WDT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_WAKE_WDT` writer - Indicates WDT wakeup cause. The WDT interrupt must be cleared before this bit can be cleared."]
pub type HIB_WAKE_WDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_HIB_WAKE_CAUSE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Each bit indicates a HIBERNATE wakeup cause. The number and assigment of wakeup sources are product-specific. For each bit, writing a 1 clears the cause flag."]
    #[inline(always)]
    pub fn hib_wake_src(&self) -> HIB_WAKE_SRC_R {
        HIB_WAKE_SRC_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 29 - Indicates CSV_BAK wakeup cause. The related fault must be handled before this bit can be cleared."]
    #[inline(always)]
    pub fn hib_wake_csv_bak(&self) -> HIB_WAKE_CSV_BAK_R {
        HIB_WAKE_CSV_BAK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates RTC wakeup cause. The RTC interrupt must be cleared before this bit can be cleared."]
    #[inline(always)]
    pub fn hib_wake_rtc(&self) -> HIB_WAKE_RTC_R {
        HIB_WAKE_RTC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates WDT wakeup cause. The WDT interrupt must be cleared before this bit can be cleared."]
    #[inline(always)]
    pub fn hib_wake_wdt(&self) -> HIB_WAKE_WDT_R {
        HIB_WAKE_WDT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Each bit indicates a HIBERNATE wakeup cause. The number and assigment of wakeup sources are product-specific. For each bit, writing a 1 clears the cause flag."]
    #[inline(always)]
    #[must_use]
    pub fn hib_wake_src(&mut self) -> HIB_WAKE_SRC_W<0> {
        HIB_WAKE_SRC_W::new(self)
    }
    #[doc = "Bit 29 - Indicates CSV_BAK wakeup cause. The related fault must be handled before this bit can be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn hib_wake_csv_bak(&mut self) -> HIB_WAKE_CSV_BAK_W<29> {
        HIB_WAKE_CSV_BAK_W::new(self)
    }
    #[doc = "Bit 30 - Indicates RTC wakeup cause. The RTC interrupt must be cleared before this bit can be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn hib_wake_rtc(&mut self) -> HIB_WAKE_RTC_W<30> {
        HIB_WAKE_RTC_W::new(self)
    }
    #[doc = "Bit 31 - Indicates WDT wakeup cause. The WDT interrupt must be cleared before this bit can be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn hib_wake_wdt(&mut self) -> HIB_WAKE_WDT_W<31> {
        HIB_WAKE_WDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Wakeup Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hib_wake_cause](index.html) module"]
pub struct PWR_HIB_WAKE_CAUSE_SPEC;
impl crate::RegisterSpec for PWR_HIB_WAKE_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_hib_wake_cause::R](R) reader structure"]
impl crate::Readable for PWR_HIB_WAKE_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_hib_wake_cause::W](W) writer structure"]
impl crate::Writable for PWR_HIB_WAKE_CAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_HIB_WAKE_CAUSE to value 0"]
impl crate::Resettable for PWR_HIB_WAKE_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
