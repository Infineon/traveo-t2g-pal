#[doc = "Register `WORK_FLASH_SAFETY` reader"]
pub struct R(crate::R<WORK_FLASH_SAFETY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_FLASH_SAFETY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_FLASH_SAFETY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_FLASH_SAFETY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WORK_FLASH_SAFETY` writer"]
pub struct W(crate::W<WORK_FLASH_SAFETY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WORK_FLASH_SAFETY_SPEC>;
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
impl From<crate::W<WORK_FLASH_SAFETY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WORK_FLASH_SAFETY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORKFLASHWRITEENABLE` reader - 0: Work Flash embedded operations are blocked 1: Work Flash embedded operations are enabled"]
pub type WORKFLASHWRITEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WORKFLASHWRITEENABLE` writer - 0: Work Flash embedded operations are blocked 1: Work Flash embedded operations are enabled"]
pub type WORKFLASHWRITEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WORK_FLASH_SAFETY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Work Flash embedded operations are blocked 1: Work Flash embedded operations are enabled"]
    #[inline(always)]
    pub fn workflashwriteenable(&self) -> WORKFLASHWRITEENABLE_R {
        WORKFLASHWRITEENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Work Flash embedded operations are blocked 1: Work Flash embedded operations are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn workflashwriteenable(&mut self) -> WORKFLASHWRITEENABLE_W<0> {
        WORKFLASHWRITEENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Work Flash Security enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_flash_safety](index.html) module"]
pub struct WORK_FLASH_SAFETY_SPEC;
impl crate::RegisterSpec for WORK_FLASH_SAFETY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_flash_safety::R](R) reader structure"]
impl crate::Readable for WORK_FLASH_SAFETY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [work_flash_safety::W](W) writer structure"]
impl crate::Writable for WORK_FLASH_SAFETY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WORK_FLASH_SAFETY to value 0"]
impl crate::Resettable for WORK_FLASH_SAFETY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
