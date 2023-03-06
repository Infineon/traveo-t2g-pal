#[doc = "Register `READCHANNELCONFIG` reader"]
pub struct R(crate::R<READCHANNELCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCHANNELCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCHANNELCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCHANNELCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READCHANNELCONFIG` writer"]
pub struct W(crate::W<READCHANNELCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCHANNELCONFIG_SPEC>;
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
impl From<crate::W<READCHANNELCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCHANNELCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READCHANNELWORDS` reader - Set to number of 32-bit words to read minus 1."]
pub type READCHANNELWORDS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `READCHANNELWORDS` writer - Set to number of 32-bit words to read minus 1."]
pub type READCHANNELWORDS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READCHANNELCONFIG_SPEC, u32, u32, 24, O>;
#[doc = "Field `READCHANNELMAXBURSTLENGTH` reader - Default burstlength for read channel requests. Will always be used except for the last access if remaining words is less than ReadChannelMaxBurstLength."]
pub type READCHANNELMAXBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READCHANNELMAXBURSTLENGTH` writer - Default burstlength for read channel requests. Will always be used except for the last access if remaining words is less than ReadChannelMaxBurstLength."]
pub type READCHANNELMAXBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READCHANNELCONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `READCHANNELENABLE` reader - Enable read channel interface. Locks the InstructionFifo from configuration interface."]
pub type READCHANNELENABLE_R = crate::BitReader<bool>;
#[doc = "Field `READCHANNELENABLE` writer - Enable read channel interface. Locks the InstructionFifo from configuration interface."]
pub type READCHANNELENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, READCHANNELCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Set to number of 32-bit words to read minus 1."]
    #[inline(always)]
    pub fn readchannelwords(&self) -> READCHANNELWORDS_R {
        READCHANNELWORDS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:28 - Default burstlength for read channel requests. Will always be used except for the last access if remaining words is less than ReadChannelMaxBurstLength."]
    #[inline(always)]
    pub fn readchannelmaxburstlength(&self) -> READCHANNELMAXBURSTLENGTH_R {
        READCHANNELMAXBURSTLENGTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable read channel interface. Locks the InstructionFifo from configuration interface."]
    #[inline(always)]
    pub fn readchannelenable(&self) -> READCHANNELENABLE_R {
        READCHANNELENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Set to number of 32-bit words to read minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn readchannelwords(&mut self) -> READCHANNELWORDS_W<0> {
        READCHANNELWORDS_W::new(self)
    }
    #[doc = "Bits 24:28 - Default burstlength for read channel requests. Will always be used except for the last access if remaining words is less than ReadChannelMaxBurstLength."]
    #[inline(always)]
    #[must_use]
    pub fn readchannelmaxburstlength(&mut self) -> READCHANNELMAXBURSTLENGTH_W<24> {
        READCHANNELMAXBURSTLENGTH_W::new(self)
    }
    #[doc = "Bit 31 - Enable read channel interface. Locks the InstructionFifo from configuration interface."]
    #[inline(always)]
    #[must_use]
    pub fn readchannelenable(&mut self) -> READCHANNELENABLE_W<31> {
        READCHANNELENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Channel Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readchannelconfig](index.html) module"]
pub struct READCHANNELCONFIG_SPEC;
impl crate::RegisterSpec for READCHANNELCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readchannelconfig::R](R) reader structure"]
impl crate::Readable for READCHANNELCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readchannelconfig::W](W) writer structure"]
impl crate::Writable for READCHANNELCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCHANNELCONFIG to value 0x0800_0000"]
impl crate::Resettable for READCHANNELCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
