#[doc = "Register `AXICTL` reader"]
pub struct R(crate::R<AXICTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXICTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXICTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXICTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AXICTL` writer"]
pub struct W(crate::W<AXICTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXICTL_SPEC>;
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
impl From<crate::W<AXICTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXICTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARCACHE` reader - ARCACHE value of AXI transactions on Fetch Unit. Must not be changed during decoding operation."]
pub type ARCACHE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARCACHE` writer - ARCACHE value of AXI transactions on Fetch Unit. Must not be changed during decoding operation."]
pub type ARCACHE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AXICTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWCACHE` reader - AWCACHE value of AXI transactions on Store Unit. Must not be changed during decoding operation."]
pub type AWCACHE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWCACHE` writer - AWCACHE value of AXI transactions on Store Unit. Must not be changed during decoding operation."]
pub type AWCACHE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AXICTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ARCACHE value of AXI transactions on Fetch Unit. Must not be changed during decoding operation."]
    #[inline(always)]
    pub fn arcache(&self) -> ARCACHE_R {
        ARCACHE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AWCACHE value of AXI transactions on Store Unit. Must not be changed during decoding operation."]
    #[inline(always)]
    pub fn awcache(&self) -> AWCACHE_R {
        AWCACHE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ARCACHE value of AXI transactions on Fetch Unit. Must not be changed during decoding operation."]
    #[inline(always)]
    #[must_use]
    pub fn arcache(&mut self) -> ARCACHE_W<0> {
        ARCACHE_W::new(self)
    }
    #[doc = "Bits 8:11 - AWCACHE value of AXI transactions on Store Unit. Must not be changed during decoding operation."]
    #[inline(always)]
    #[must_use]
    pub fn awcache(&mut self) -> AWCACHE_W<8> {
        AWCACHE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI cache attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [axictl](index.html) module"]
pub struct AXICTL_SPEC;
impl crate::RegisterSpec for AXICTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [axictl::R](R) reader structure"]
impl crate::Readable for AXICTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [axictl::W](W) writer structure"]
impl crate::Writable for AXICTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXICTL to value 0"]
impl crate::Resettable for AXICTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
