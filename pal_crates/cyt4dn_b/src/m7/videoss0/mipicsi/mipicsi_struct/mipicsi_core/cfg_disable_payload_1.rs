#[doc = "Register `CFG_DISABLE_PAYLOAD_1` reader"]
pub struct R(crate::R<CFG_DISABLE_PAYLOAD_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DISABLE_PAYLOAD_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DISABLE_PAYLOAD_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DISABLE_PAYLOAD_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DISABLE_PAYLOAD_1` writer"]
pub struct W(crate::W<CFG_DISABLE_PAYLOAD_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DISABLE_PAYLOAD_1_SPEC>;
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
impl From<crate::W<CFG_DISABLE_PAYLOAD_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DISABLE_PAYLOAD_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_DISABLE_PAYLOAD_1` reader - N/A"]
pub type CFG_DISABLE_PAYLOAD_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFG_DISABLE_PAYLOAD_1` writer - N/A"]
pub type CFG_DISABLE_PAYLOAD_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DISABLE_PAYLOAD_1_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - N/A"]
    #[inline(always)]
    pub fn cfg_disable_payload_1(&self) -> CFG_DISABLE_PAYLOAD_1_R {
        CFG_DISABLE_PAYLOAD_1_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_disable_payload_1(&mut self) -> CFG_DISABLE_PAYLOAD_1_W<0> {
        CFG_DISABLE_PAYLOAD_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG_DISABLE_PAYLOAD_1 is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_disable_payload_1](index.html) module"]
pub struct CFG_DISABLE_PAYLOAD_1_SPEC;
impl crate::RegisterSpec for CFG_DISABLE_PAYLOAD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_disable_payload_1::R](R) reader structure"]
impl crate::Readable for CFG_DISABLE_PAYLOAD_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_disable_payload_1::W](W) writer structure"]
impl crate::Writable for CFG_DISABLE_PAYLOAD_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DISABLE_PAYLOAD_1 to value 0"]
impl crate::Resettable for CFG_DISABLE_PAYLOAD_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
