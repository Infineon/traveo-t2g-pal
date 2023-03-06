#[doc = "Register `SECSTATCONFIG` reader"]
pub struct R(crate::R<SECSTATCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSTATCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSTATCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSTATCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECSTATCONFIG` writer"]
pub struct W(crate::W<SECSTATCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECSTATCONFIG_SPEC>;
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
impl From<crate::W<SECSTATCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECSTATCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVGOODFRAMES` reader - Number of continuous correct frames that must be processed before SecSyncStat field goes 1 (in sync). Value of 0 is not allowed."]
pub type LEVGOODFRAMES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVGOODFRAMES` writer - Number of continuous correct frames that must be processed before SecSyncStat field goes 1 (in sync). Value of 0 is not allowed."]
pub type LEVGOODFRAMES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECSTATCONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `LEVBADFRAMES` reader - Not used."]
pub type LEVBADFRAMES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVBADFRAMES` writer - Not used."]
pub type LEVBADFRAMES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECSTATCONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `LEVSKEWINRANGE` reader - Number of continuous frames the measured skew value shall be within the range defined by SyncRangeLow and SyncRangeHigh. If this condition is true, additionally LevGoodFrames valid frames must be processed before the status line of the sec. channel and the SecSyncStat status field goes to 1 indicating synchronized status. Value of 0 is not allowed."]
pub type LEVSKEWINRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVSKEWINRANGE` writer - Number of continuous frames the measured skew value shall be within the range defined by SyncRangeLow and SyncRangeHigh. If this condition is true, additionally LevGoodFrames valid frames must be processed before the status line of the sec. channel and the SecSyncStat status field goes to 1 indicating synchronized status. Value of 0 is not allowed."]
pub type LEVSKEWINRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECSTATCONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Number of continuous correct frames that must be processed before SecSyncStat field goes 1 (in sync). Value of 0 is not allowed."]
    #[inline(always)]
    pub fn levgoodframes(&self) -> LEVGOODFRAMES_R {
        LEVGOODFRAMES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Not used."]
    #[inline(always)]
    pub fn levbadframes(&self) -> LEVBADFRAMES_R {
        LEVBADFRAMES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of continuous frames the measured skew value shall be within the range defined by SyncRangeLow and SyncRangeHigh. If this condition is true, additionally LevGoodFrames valid frames must be processed before the status line of the sec. channel and the SecSyncStat status field goes to 1 indicating synchronized status. Value of 0 is not allowed."]
    #[inline(always)]
    pub fn levskewinrange(&self) -> LEVSKEWINRANGE_R {
        LEVSKEWINRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of continuous correct frames that must be processed before SecSyncStat field goes 1 (in sync). Value of 0 is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn levgoodframes(&mut self) -> LEVGOODFRAMES_W<0> {
        LEVGOODFRAMES_W::new(self)
    }
    #[doc = "Bits 4:7 - Not used."]
    #[inline(always)]
    #[must_use]
    pub fn levbadframes(&mut self) -> LEVBADFRAMES_W<4> {
        LEVBADFRAMES_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of continuous frames the measured skew value shall be within the range defined by SyncRangeLow and SyncRangeHigh. If this condition is true, additionally LevGoodFrames valid frames must be processed before the status line of the sec. channel and the SecSyncStat status field goes to 1 indicating synchronized status. Value of 0 is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn levskewinrange(&mut self) -> LEVSKEWINRANGE_W<8> {
        LEVSKEWINRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for controlling the behavior of the SecSyncStat field in the FgSecChStat register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secstatconfig](index.html) module"]
pub struct SECSTATCONFIG_SPEC;
impl crate::RegisterSpec for SECSTATCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secstatconfig::R](R) reader structure"]
impl crate::Readable for SECSTATCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secstatconfig::W](W) writer structure"]
impl crate::Writable for SECSTATCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECSTATCONFIG to value 0x0112"]
impl crate::Resettable for SECSTATCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0112;
}
