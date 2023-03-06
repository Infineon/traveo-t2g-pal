#[doc = "Register `READTIMINGCONFIG` reader"]
pub struct R(crate::R<READTIMINGCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READTIMINGCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READTIMINGCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READTIMINGCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READTIMINGCONFIG` writer"]
pub struct W(crate::W<READTIMINGCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READTIMINGCONFIG_SPEC>;
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
impl From<crate::W<READTIMINGCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READTIMINGCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READSETUPCYCLES` reader - Number of module clock cycles minus one to wait before rd#/cs#/en terminal can become active after address set (tAW)."]
pub type READSETUPCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READSETUPCYCLES` writer - Number of module clock cycles minus one to wait before rd#/cs#/en terminal can become active after address set (tAW)."]
pub type READSETUPCYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READTIMINGCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `READACTIVECYCLES` reader - Number of module clock cycles minus one rd#/cs#/en terminal should remain active (tCCLR)."]
pub type READACTIVECYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READACTIVECYCLES` writer - Number of module clock cycles minus one rd#/cs#/en terminal should remain active (tCCLR)."]
pub type READACTIVECYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READTIMINGCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `READHOLDCYCLES` reader - Number of module clock cycles minus one to wait after rd#/cs#/en terminal became inactive before address can change (tAH)."]
pub type READHOLDCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READHOLDCYCLES` writer - Number of module clock cycles minus one to wait after rd#/cs#/en terminal became inactive before address can change (tAH)."]
pub type READHOLDCYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READTIMINGCONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Number of module clock cycles minus one to wait before rd#/cs#/en terminal can become active after address set (tAW)."]
    #[inline(always)]
    pub fn readsetupcycles(&self) -> READSETUPCYCLES_R {
        READSETUPCYCLES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of module clock cycles minus one rd#/cs#/en terminal should remain active (tCCLR)."]
    #[inline(always)]
    pub fn readactivecycles(&self) -> READACTIVECYCLES_R {
        READACTIVECYCLES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of module clock cycles minus one to wait after rd#/cs#/en terminal became inactive before address can change (tAH)."]
    #[inline(always)]
    pub fn readholdcycles(&self) -> READHOLDCYCLES_R {
        READHOLDCYCLES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of module clock cycles minus one to wait before rd#/cs#/en terminal can become active after address set (tAW)."]
    #[inline(always)]
    #[must_use]
    pub fn readsetupcycles(&mut self) -> READSETUPCYCLES_W<0> {
        READSETUPCYCLES_W::new(self)
    }
    #[doc = "Bits 8:15 - Number of module clock cycles minus one rd#/cs#/en terminal should remain active (tCCLR)."]
    #[inline(always)]
    #[must_use]
    pub fn readactivecycles(&mut self) -> READACTIVECYCLES_W<8> {
        READACTIVECYCLES_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of module clock cycles minus one to wait after rd#/cs#/en terminal became inactive before address can change (tAH)."]
    #[inline(always)]
    #[must_use]
    pub fn readholdcycles(&mut self) -> READHOLDCYCLES_W<16> {
        READHOLDCYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read timing configuration register. Sum of all fields is read cycle time in module clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readtimingconfig](index.html) module"]
pub struct READTIMINGCONFIG_SPEC;
impl crate::RegisterSpec for READTIMINGCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readtimingconfig::R](R) reader structure"]
impl crate::Readable for READTIMINGCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readtimingconfig::W](W) writer structure"]
impl crate::Writable for READTIMINGCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READTIMINGCONFIG to value 0x0004_0804"]
impl crate::Resettable for READTIMINGCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0804;
}
