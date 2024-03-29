#[doc = "Register `ENABLE` reader"]
pub struct R(crate::R<ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE` writer"]
pub struct W(crate::W<ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_SPEC>;
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
impl From<crate::W<ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAN_EN` reader - Channel enable. - 0: the corresponding channel is disabled. Corresponding trigger will be reset immediately. - 1: the corresponding channel is enabled. Note: To disable a group either stop the trigger first or begin with disabling the lowest channel first. To enable a group either start with enabling the last channel first and the first channel last, or start the trigger after all channels are enabled. If these rules are not followed the result is undefined."]
pub type CHAN_EN_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_EN` writer - Channel enable. - 0: the corresponding channel is disabled. Corresponding trigger will be reset immediately. - 1: the corresponding channel is enabled. Note: To disable a group either stop the trigger first or begin with disabling the lowest channel first. To enable a group either start with enabling the last channel first and the first channel last, or start the trigger after all channels are enabled. If these rules are not followed the result is undefined."]
pub type CHAN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel enable. - 0: the corresponding channel is disabled. Corresponding trigger will be reset immediately. - 1: the corresponding channel is enabled. Note: To disable a group either stop the trigger first or begin with disabling the lowest channel first. To enable a group either start with enabling the last channel first and the first channel last, or start the trigger after all channels are enabled. If these rules are not followed the result is undefined."]
    #[inline(always)]
    pub fn chan_en(&self) -> CHAN_EN_R {
        CHAN_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable. - 0: the corresponding channel is disabled. Corresponding trigger will be reset immediately. - 1: the corresponding channel is enabled. Note: To disable a group either stop the trigger first or begin with disabling the lowest channel first. To enable a group either start with enabling the last channel first and the first channel last, or start the trigger after all channels are enabled. If these rules are not followed the result is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn chan_en(&mut self) -> CHAN_EN_W<0> {
        CHAN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](index.html) module"]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable::R](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable::W](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
