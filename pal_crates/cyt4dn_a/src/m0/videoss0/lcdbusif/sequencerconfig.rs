#[doc = "Register `SEQUENCERCONFIG` reader"]
pub struct R(crate::R<SEQUENCERCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQUENCERCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQUENCERCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQUENCERCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQUENCERCONFIG` writer"]
pub struct W(crate::W<SEQUENCERCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQUENCERCONFIG_SPEC>;
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
impl From<crate::W<SEQUENCERCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQUENCERCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTIONTIMEOUT` reader - Value to preload the timeout counter with each instruction read."]
pub type INSTRUCTIONTIMEOUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INSTRUCTIONTIMEOUT` writer - Value to preload the timeout counter with each instruction read."]
pub type INSTRUCTIONTIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQUENCERCONFIG_SPEC, u32, u32, 31, O>;
#[doc = "Field `INSTRUCTIONTIMEOUTENABLE` reader - Enable or disable instruction timeout counter."]
pub type INSTRUCTIONTIMEOUTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INSTRUCTIONTIMEOUTENABLE` writer - Enable or disable instruction timeout counter."]
pub type INSTRUCTIONTIMEOUTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEQUENCERCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Value to preload the timeout counter with each instruction read."]
    #[inline(always)]
    pub fn instructiontimeout(&self) -> INSTRUCTIONTIMEOUT_R {
        INSTRUCTIONTIMEOUT_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Enable or disable instruction timeout counter."]
    #[inline(always)]
    pub fn instructiontimeoutenable(&self) -> INSTRUCTIONTIMEOUTENABLE_R {
        INSTRUCTIONTIMEOUTENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Value to preload the timeout counter with each instruction read."]
    #[inline(always)]
    #[must_use]
    pub fn instructiontimeout(&mut self) -> INSTRUCTIONTIMEOUT_W<0> {
        INSTRUCTIONTIMEOUT_W::new(self)
    }
    #[doc = "Bit 31 - Enable or disable instruction timeout counter."]
    #[inline(always)]
    #[must_use]
    pub fn instructiontimeoutenable(&mut self) -> INSTRUCTIONTIMEOUTENABLE_W<31> {
        INSTRUCTIONTIMEOUTENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sequencerconfig](index.html) module"]
pub struct SEQUENCERCONFIG_SPEC;
impl crate::RegisterSpec for SEQUENCERCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sequencerconfig::R](R) reader structure"]
impl crate::Readable for SEQUENCERCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sequencerconfig::W](W) writer structure"]
impl crate::Writable for SEQUENCERCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQUENCERCONFIG to value 0x87ff_ffff"]
impl crate::Resettable for SEQUENCERCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x87ff_ffff;
}
