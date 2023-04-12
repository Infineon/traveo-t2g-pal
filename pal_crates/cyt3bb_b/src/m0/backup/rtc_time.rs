#[doc = "Register `RTC_TIME` reader"]
pub struct R(crate::R<RTC_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME` writer"]
pub struct W(crate::W<RTC_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_SPEC>;
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
impl From<crate::W<RTC_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SEC` reader - Calendar seconds, 0-59"]
pub type RTC_SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_SEC` writer - Calendar seconds, 0-59"]
pub type RTC_SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_TIME_SPEC, u8, u8, 6, O>;
#[doc = "Field `RTC_MIN` reader - Calendar minutes, 0-59"]
pub type RTC_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_MIN` writer - Calendar minutes, 0-59"]
pub type RTC_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_TIME_SPEC, u8, u8, 6, O>;
#[doc = "Field `RTC_HOUR` reader - Calendar hours, value depending on 12/24HR mode 0=24HR: \\[20:16\\]=0-23 1=12HR: \\[20\\]:0=AM, 1=PM, \\[19:16\\]=1-12"]
pub type RTC_HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_HOUR` writer - Calendar hours, value depending on 12/24HR mode 0=24HR: \\[20:16\\]=0-23 1=12HR: \\[20\\]:0=AM, 1=PM, \\[19:16\\]=1-12"]
pub type RTC_HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_TIME_SPEC, u8, u8, 5, O>;
#[doc = "Field `CTRL_12HR` reader - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub type CTRL_12HR_R = crate::BitReader<bool>;
#[doc = "Field `CTRL_12HR` writer - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub type CTRL_12HR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_TIME_SPEC, bool, O>;
#[doc = "Field `RTC_DAY` reader - Calendar Day of the week, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type RTC_DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DAY` writer - Calendar Day of the week, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type RTC_DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_TIME_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Calendar seconds, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&self) -> RTC_SEC_R {
        RTC_SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Calendar minutes, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&self) -> RTC_MIN_R {
        RTC_MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Calendar hours, value depending on 12/24HR mode 0=24HR: \\[20:16\\]=0-23 1=12HR: \\[20\\]:0=AM, 1=PM, \\[19:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&self) -> RTC_HOUR_R {
        RTC_HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&self) -> CTRL_12HR_R {
        CTRL_12HR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&self) -> RTC_DAY_R {
        RTC_DAY_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calendar seconds, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_sec(&mut self) -> RTC_SEC_W<0> {
        RTC_SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Calendar minutes, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_min(&mut self) -> RTC_MIN_W<8> {
        RTC_MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Calendar hours, value depending on 12/24HR mode 0=24HR: \\[20:16\\]=0-23 1=12HR: \\[20\\]:0=AM, 1=PM, \\[19:16\\]=1-12"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hour(&mut self) -> RTC_HOUR_W<16> {
        RTC_HOUR_W::new(self)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_12hr(&mut self) -> CTRL_12HR_W<22> {
        CTRL_12HR_W::new(self)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_day(&mut self) -> RTC_DAY_W<24> {
        RTC_DAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time](index.html) module"]
pub struct RTC_TIME_SPEC;
impl crate::RegisterSpec for RTC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time::R](R) reader structure"]
impl crate::Readable for RTC_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time::W](W) writer structure"]
impl crate::Writable for RTC_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_TIME to value 0x0100_0000"]
impl crate::Resettable for RTC_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
