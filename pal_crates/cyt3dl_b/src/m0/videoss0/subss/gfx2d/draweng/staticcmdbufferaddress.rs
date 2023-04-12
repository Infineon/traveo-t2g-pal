#[doc = "Register `STATICCMDBUFFERADDRESS` reader"]
pub struct R(crate::R<STATICCMDBUFFERADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCMDBUFFERADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCMDBUFFERADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCMDBUFFERADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCMDBUFFERADDRESS` writer"]
pub struct W(crate::W<STATICCMDBUFFERADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCMDBUFFERADDRESS_SPEC>;
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
impl From<crate::W<STATICCMDBUFFERADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCMDBUFFERADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDBUFFERADDRESS` reader - Start address of the command buffer. The address is word aligned, keep bits \\[1:0\\]
zero. Writing this register resets the CmdBufferReadPtr register."]
pub type CMDBUFFERADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDBUFFERADDRESS` writer - Start address of the command buffer. The address is word aligned, keep bits \\[1:0\\]
zero. Writing this register resets the CmdBufferReadPtr register."]
pub type CMDBUFFERADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCMDBUFFERADDRESS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address of the command buffer. The address is word aligned, keep bits \\[1:0\\]
zero. Writing this register resets the CmdBufferReadPtr register."]
    #[inline(always)]
    pub fn cmdbufferaddress(&self) -> CMDBUFFERADDRESS_R {
        CMDBUFFERADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the command buffer. The address is word aligned, keep bits \\[1:0\\]
zero. Writing this register resets the CmdBufferReadPtr register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdbufferaddress(&mut self) -> CMDBUFFERADDRESS_W<0> {
        CMDBUFFERADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lowest command buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcmdbufferaddress](index.html) module"]
pub struct STATICCMDBUFFERADDRESS_SPEC;
impl crate::RegisterSpec for STATICCMDBUFFERADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcmdbufferaddress::R](R) reader structure"]
impl crate::Readable for STATICCMDBUFFERADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcmdbufferaddress::W](W) writer structure"]
impl crate::Writable for STATICCMDBUFFERADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCMDBUFFERADDRESS to value 0"]
impl crate::Resettable for STATICCMDBUFFERADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
