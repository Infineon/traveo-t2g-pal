#[doc = "Register `TASKBUFFERADDRESS0` reader"]
pub struct R(crate::R<TASKBUFFERADDRESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TASKBUFFERADDRESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TASKBUFFERADDRESS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TASKBUFFERADDRESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TASKBUFFERADDRESS0` writer"]
pub struct W(crate::W<TASKBUFFERADDRESS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKBUFFERADDRESS0_SPEC>;
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
impl From<crate::W<TASKBUFFERADDRESS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKBUFFERADDRESS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESSTB0` reader - Task buffer 0 base address in multiple of 64-bit words."]
pub type ADDRESSTB0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESSTB0` writer - Task buffer 0 base address in multiple of 64-bit words."]
pub type ADDRESSTB0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TASKBUFFERADDRESS0_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 3:31 - Task buffer 0 base address in multiple of 64-bit words."]
    #[inline(always)]
    pub fn addresstb0(&self) -> ADDRESSTB0_R {
        ADDRESSTB0_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Task buffer 0 base address in multiple of 64-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn addresstb0(&mut self) -> ADDRESSTB0_W<3> {
        ADDRESSTB0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task buffer 0 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taskbufferaddress0](index.html) module"]
pub struct TASKBUFFERADDRESS0_SPEC;
impl crate::RegisterSpec for TASKBUFFERADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taskbufferaddress0::R](R) reader structure"]
impl crate::Readable for TASKBUFFERADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taskbufferaddress0::W](W) writer structure"]
impl crate::Writable for TASKBUFFERADDRESS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKBUFFERADDRESS0 to value 0"]
impl crate::Resettable for TASKBUFFERADDRESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
