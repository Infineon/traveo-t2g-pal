#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLED` reader - Receiver enable: '0': Disabled. If a receiver is disabled, the state of the CSI-2 receiver core is initialized. Note, that MMIO registers in the core and the wrapper are accessible. Configuration of the wrapper registers should be changed only while ENABLED=0. In this state all status registers of the wrapper cannot be read. Any attempt to do so will return an error response. '1': Enabled. The CSI-2 receiver core starts its operation. Note, that CTL registers other than this ENABLED bit should not be changed in this state. Note: For using MIPI a running VIDEOSS clock is required. Ensure also that VIDEOSS0_VIDEOSSCFG_IPCTRL:CTLENABLED=0x1."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Receiver enable: '0': Disabled. If a receiver is disabled, the state of the CSI-2 receiver core is initialized. Note, that MMIO registers in the core and the wrapper are accessible. Configuration of the wrapper registers should be changed only while ENABLED=0. In this state all status registers of the wrapper cannot be read. Any attempt to do so will return an error response. '1': Enabled. The CSI-2 receiver core starts its operation. Note, that CTL registers other than this ENABLED bit should not be changed in this state. Note: For using MIPI a running VIDEOSS clock is required. Ensure also that VIDEOSS0_VIDEOSSCFG_IPCTRL:CTLENABLED=0x1."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Receiver enable: '0': Disabled. If a receiver is disabled, the state of the CSI-2 receiver core is initialized. Note, that MMIO registers in the core and the wrapper are accessible. Configuration of the wrapper registers should be changed only while ENABLED=0. In this state all status registers of the wrapper cannot be read. Any attempt to do so will return an error response. '1': Enabled. The CSI-2 receiver core starts its operation. Note, that CTL registers other than this ENABLED bit should not be changed in this state. Note: For using MIPI a running VIDEOSS clock is required. Ensure also that VIDEOSS0_VIDEOSSCFG_IPCTRL:CTLENABLED=0x1."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Receiver enable: '0': Disabled. If a receiver is disabled, the state of the CSI-2 receiver core is initialized. Note, that MMIO registers in the core and the wrapper are accessible. Configuration of the wrapper registers should be changed only while ENABLED=0. In this state all status registers of the wrapper cannot be read. Any attempt to do so will return an error response. '1': Enabled. The CSI-2 receiver core starts its operation. Note, that CTL registers other than this ENABLED bit should not be changed in this state. Note: For using MIPI a running VIDEOSS clock is required. Ensure also that VIDEOSS0_VIDEOSSCFG_IPCTRL:CTLENABLED=0x1."]
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
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
