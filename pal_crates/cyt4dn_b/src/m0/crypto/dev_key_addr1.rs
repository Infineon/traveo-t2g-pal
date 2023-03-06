#[doc = "Register `DEV_KEY_ADDR1` reader"]
pub struct R(crate::R<DEV_KEY_ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_KEY_ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_KEY_ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_KEY_ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_KEY_ADDR1` writer"]
pub struct W(crate::W<DEV_KEY_ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_KEY_ADDR1_SPEC>;
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
impl From<crate::W<DEV_KEY_ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_KEY_ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR32` reader - See DEV_KEY_ADDR0."]
pub type ADDR32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR32` writer - See DEV_KEY_ADDR0."]
pub type ADDR32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEV_KEY_ADDR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See DEV_KEY_ADDR0."]
    #[inline(always)]
    pub fn addr32(&self) -> ADDR32_R {
        ADDR32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See DEV_KEY_ADDR0."]
    #[inline(always)]
    #[must_use]
    pub fn addr32(&mut self) -> ADDR32_W<0> {
        ADDR32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device key address 1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_key_addr1](index.html) module"]
pub struct DEV_KEY_ADDR1_SPEC;
impl crate::RegisterSpec for DEV_KEY_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_key_addr1::R](R) reader structure"]
impl crate::Readable for DEV_KEY_ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_key_addr1::W](W) writer structure"]
impl crate::Writable for DEV_KEY_ADDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEV_KEY_ADDR1 to value 0"]
impl crate::Resettable for DEV_KEY_ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
