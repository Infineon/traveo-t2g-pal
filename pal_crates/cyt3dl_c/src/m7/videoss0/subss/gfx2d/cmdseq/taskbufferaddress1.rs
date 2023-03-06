#[doc = "Register `TASKBUFFERADDRESS1` reader"]
pub struct R(crate::R<TASKBUFFERADDRESS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERADDRESS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERADDRESS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERADDRESS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERADDRESS1` writer"]
pub struct W(crate::W<TASKBUFFERADDRESS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERADDRESS1_SPEC>;
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
impl From<crate::W<TASKBUFFERADDRESS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERADDRESS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESSTB1` reader - Task buffer 1 base address in multiple of 64-bit words."]
pub type ADDRESSTB1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESSTB1` writer - Task buffer 1 base address in multiple of 64-bit words."]
pub type ADDRESSTB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERADDRESS1_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 3:31 - Task buffer 1 base address in multiple of 64-bit words."]
    #[inline(always)]
    pub fn addresstb1(&self) -> ADDRESSTB1_R {
        ADDRESSTB1_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Task buffer 1 base address in multiple of 64-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn addresstb1(&mut self) -> ADDRESSTB1_W<3> {
        ADDRESSTB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferaddress1](index.html) module"]
pub struct TASKBUFFERADDRESS1_SPEC;
impl crate::RegisterSpec for TASKBUFFERADDRESS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferaddress1::R](R) reader structure"]
impl crate::Readable for TASKBUFFERADDRESS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbufferaddress1::W](W) writer structure"]
impl crate::Writable for TASKBUFFERADDRESS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERADDRESS1 to value 0"]
impl crate::Resettable for TASKBUFFERADDRESS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
