#[doc = "Register `MAIN_FLASH_SAFETY` reader"]
pub struct R(crate::R<MAIN_FLASH_SAFETY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIN_FLASH_SAFETY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIN_FLASH_SAFETY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIN_FLASH_SAFETY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAIN_FLASH_SAFETY` writer"]
pub struct W(crate::W<MAIN_FLASH_SAFETY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAIN_FLASH_SAFETY_SPEC>;
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
impl From<crate::W<MAIN_FLASH_SAFETY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAIN_FLASH_SAFETY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINFLASHWRITEENABLE` reader - '0': Main Flash embedded operations are blocked '1': Main Flash embedded operations are enabled"]
pub type MAINFLASHWRITEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MAINFLASHWRITEENABLE` writer - '0': Main Flash embedded operations are blocked '1': Main Flash embedded operations are enabled"]
pub type MAINFLASHWRITEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MAIN_FLASH_SAFETY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - '0': Main Flash embedded operations are blocked '1': Main Flash embedded operations are enabled"]
    #[inline(always)]
    pub fn mainflashwriteenable(&self) -> MAINFLASHWRITEENABLE_R {
        MAINFLASHWRITEENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - '0': Main Flash embedded operations are blocked '1': Main Flash embedded operations are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn mainflashwriteenable(&mut self) -> MAINFLASHWRITEENABLE_W<0> {
        MAINFLASHWRITEENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main (Code) Flash Security enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_flash_safety](index.html) module"]
pub struct MAIN_FLASH_SAFETY_SPEC;
impl crate::RegisterSpec for MAIN_FLASH_SAFETY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [main_flash_safety::R](R) reader structure"]
impl crate::Readable for MAIN_FLASH_SAFETY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [main_flash_safety::W](W) writer structure"]
impl crate::Writable for MAIN_FLASH_SAFETY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAIN_FLASH_SAFETY to value 0"]
impl crate::Resettable for MAIN_FLASH_SAFETY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
