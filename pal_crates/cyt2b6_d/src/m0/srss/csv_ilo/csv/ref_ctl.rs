#[doc = "Register `REF_CTL` reader"]
pub struct R(crate::R<REF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REF_CTL` writer"]
pub struct W(crate::W<REF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_CTL_SPEC>;
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
impl From<crate::W<REF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTUP` reader - Startup delay time -1 (in reference clock cycles), after enable, from reference clock start to monitored clock start. At a minimum (both clocks running): STARTUP >= (PERIOD +3) * FREQ_RATIO - UPPER, with FREQ_RATIO = (Reference frequency / Monitored frequency) On top of that the actual clock startup delay and the margin for accuracy of both clocks must be added."]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Startup delay time -1 (in reference clock cycles), after enable, from reference clock start to monitored clock start. At a minimum (both clocks running): STARTUP >= (PERIOD +3) * FREQ_RATIO - UPPER, with FREQ_RATIO = (Reference frequency / Monitored frequency) On top of that the actual clock startup delay and the margin for accuracy of both clocks must be added."]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REF_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CSV_EN` reader - Enables clock supervision, both frequency and loss. CSV in Active domain: Clock supervision is reset during DeepSleep and Hibernate modes. When enabled it begins operating automatically after a DeepSleep wakeup, but it must be reconfigured after Hibernate wakeup. CSV in DeepSleep domain: Clock supervision is reset during Hibernate mode. It must be reconfigured after Hibernate wakeup. CSV in Backup domain: Clock supervision operates during Hibernate mode, can be configured to wake from Hibernate, and continues operating during reboot. A CSV error detection is reported to the Fault structure."]
pub type CSV_EN_R = crate::BitReader<bool>;
#[doc = "Field `CSV_EN` writer - Enables clock supervision, both frequency and loss. CSV in Active domain: Clock supervision is reset during DeepSleep and Hibernate modes. When enabled it begins operating automatically after a DeepSleep wakeup, but it must be reconfigured after Hibernate wakeup. CSV in DeepSleep domain: Clock supervision is reset during Hibernate mode. It must be reconfigured after Hibernate wakeup. CSV in Backup domain: Clock supervision operates during Hibernate mode, can be configured to wake from Hibernate, and continues operating during reboot. A CSV error detection is reported to the Fault structure."]
pub type CSV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REF_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Startup delay time -1 (in reference clock cycles), after enable, from reference clock start to monitored clock start. At a minimum (both clocks running): STARTUP >= (PERIOD +3) * FREQ_RATIO - UPPER, with FREQ_RATIO = (Reference frequency / Monitored frequency) On top of that the actual clock startup delay and the margin for accuracy of both clocks must be added."]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enables clock supervision, both frequency and loss. CSV in Active domain: Clock supervision is reset during DeepSleep and Hibernate modes. When enabled it begins operating automatically after a DeepSleep wakeup, but it must be reconfigured after Hibernate wakeup. CSV in DeepSleep domain: Clock supervision is reset during Hibernate mode. It must be reconfigured after Hibernate wakeup. CSV in Backup domain: Clock supervision operates during Hibernate mode, can be configured to wake from Hibernate, and continues operating during reboot. A CSV error detection is reported to the Fault structure."]
    #[inline(always)]
    pub fn csv_en(&self) -> CSV_EN_R {
        CSV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Startup delay time -1 (in reference clock cycles), after enable, from reference clock start to monitored clock start. At a minimum (both clocks running): STARTUP >= (PERIOD +3) * FREQ_RATIO - UPPER, with FREQ_RATIO = (Reference frequency / Monitored frequency) On top of that the actual clock startup delay and the margin for accuracy of both clocks must be added."]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<0> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 31 - Enables clock supervision, both frequency and loss. CSV in Active domain: Clock supervision is reset during DeepSleep and Hibernate modes. When enabled it begins operating automatically after a DeepSleep wakeup, but it must be reconfigured after Hibernate wakeup. CSV in DeepSleep domain: Clock supervision is reset during Hibernate mode. It must be reconfigured after Hibernate wakeup. CSV in Backup domain: Clock supervision operates during Hibernate mode, can be configured to wake from Hibernate, and continues operating during reboot. A CSV error detection is reported to the Fault structure."]
    #[inline(always)]
    #[must_use]
    pub fn csv_en(&mut self) -> CSV_EN_W<31> {
        CSV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Supervision Reference Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ctl](index.html) module"]
pub struct REF_CTL_SPEC;
impl crate::RegisterSpec for REF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_ctl::R](R) reader structure"]
impl crate::Readable for REF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ref_ctl::W](W) writer structure"]
impl crate::Writable for REF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CTL to value 0"]
impl crate::Resettable for REF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
