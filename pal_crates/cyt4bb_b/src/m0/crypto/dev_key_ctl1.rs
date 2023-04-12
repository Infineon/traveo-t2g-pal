#[doc = "Register `DEV_KEY_CTL1` reader"]
pub struct R(crate::R<DEV_KEY_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_KEY_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_KEY_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_KEY_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_KEY_CTL1` writer"]
pub struct W(crate::W<DEV_KEY_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_KEY_CTL1_SPEC>;
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
impl From<crate::W<DEV_KEY_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_KEY_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOWED` reader - See DEV_KEY_CTL0."]
pub type ALLOWED_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWED` writer - See DEV_KEY_CTL0."]
pub type ALLOWED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEV_KEY_CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See DEV_KEY_CTL0."]
    #[inline(always)]
    pub fn allowed(&self) -> ALLOWED_R {
        ALLOWED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See DEV_KEY_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn allowed(&mut self) -> ALLOWED_W<0> {
        ALLOWED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device key control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_key_ctl1](index.html) module"]
pub struct DEV_KEY_CTL1_SPEC;
impl crate::RegisterSpec for DEV_KEY_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_key_ctl1::R](R) reader structure"]
impl crate::Readable for DEV_KEY_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_key_ctl1::W](W) writer structure"]
impl crate::Writable for DEV_KEY_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEV_KEY_CTL1 to value 0"]
impl crate::Resettable for DEV_KEY_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
