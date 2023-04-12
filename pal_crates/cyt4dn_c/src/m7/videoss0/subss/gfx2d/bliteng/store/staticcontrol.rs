#[doc = "Register `STATICCONTROL` reader"]
pub struct R(crate::R<STATICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONTROL` writer"]
pub struct W(crate::W<STATICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONTROL_SPEC>;
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
impl From<crate::W<STATICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `SINGLEFRAMEOPERATION` reader - If this field and ShdEn are enabled, only frames with reload bit in command word are written to AXI, other frames are still consumed. If disabled all frames are writen to AXI"]
pub type SINGLEFRAMEOPERATION_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEFRAMEOPERATION` writer - If this field and ShdEn are enabled, only frames with reload bit in command word are written to AXI, other frames are still consumed. If disabled all frames are writen to AXI"]
pub type SINGLEFRAMEOPERATION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `BASEADDRESSAUTOUPDATE` reader - If enabled (value 1) the base address is automatically updated at the start of each frame. This update is then executed independently of the ShdEn setting. When disabled ShdEn controls the update of the base address operation register."]
pub type BASEADDRESSAUTOUPDATE_R = crate::BitReader<bool>;
#[doc = "Field `BASEADDRESSAUTOUPDATE` writer - If enabled (value 1) the base address is automatically updated at the start of each frame. This update is then executed independently of the ShdEn setting. When disabled ShdEn controls the update of the base address operation register."]
pub type BASEADDRESSAUTOUPDATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - If this field and ShdEn are enabled, only frames with reload bit in command word are written to AXI, other frames are still consumed. If disabled all frames are writen to AXI"]
    #[inline(always)]
    pub fn singleframeoperation(&self) -> SINGLEFRAMEOPERATION_R {
        SINGLEFRAMEOPERATION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - If enabled (value 1) the base address is automatically updated at the start of each frame. This update is then executed independently of the ShdEn setting. When disabled ShdEn controls the update of the base address operation register."]
    #[inline(always)]
    pub fn baseaddressautoupdate(&self) -> BASEADDRESSAUTOUPDATE_R {
        BASEADDRESSAUTOUPDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 4 - If this field and ShdEn are enabled, only frames with reload bit in command word are written to AXI, other frames are still consumed. If disabled all frames are writen to AXI"]
    #[inline(always)]
    #[must_use]
    pub fn singleframeoperation(&mut self) -> SINGLEFRAMEOPERATION_W<4> {
        SINGLEFRAMEOPERATION_W::new(self)
    }
    #[doc = "Bit 8 - If enabled (value 1) the base address is automatically updated at the start of each frame. This update is then executed independently of the ShdEn setting. When disabled ShdEn controls the update of the base address operation register."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddressautoupdate(&mut self) -> BASEADDRESSAUTOUPDATE_W<8> {
        BASEADDRESSAUTOUPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Store unit static control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
pub struct STATICCONTROL_SPEC;
impl crate::RegisterSpec for STATICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcontrol::R](R) reader structure"]
impl crate::Readable for STATICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcontrol::W](W) writer structure"]
impl crate::Writable for STATICCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCONTROL to value 0"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
