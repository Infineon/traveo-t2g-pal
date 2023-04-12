#[doc = "Register `WRITETIMINGCONFIG` reader"]
pub struct R(crate::R<WRITETIMINGCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITETIMINGCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITETIMINGCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITETIMINGCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITETIMINGCONFIG` writer"]
pub struct W(crate::W<WRITETIMINGCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITETIMINGCONFIG_SPEC>;
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
impl From<crate::W<WRITETIMINGCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITETIMINGCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITESETUPCYCLES` reader - Number of module clock cycles minus one to wait before wr#/cs#/en terminal can become active after address set (tAW)."]
pub type WRITESETUPCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITESETUPCYCLES` writer - Number of module clock cycles minus one to wait before wr#/cs#/en terminal can become active after address set (tAW)."]
pub type WRITESETUPCYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITETIMINGCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRITEACTIVECYCLES` reader - Number of module clock cycles minus one wr#/cs#/en terminal should remain active (tCCLW)."]
pub type WRITEACTIVECYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITEACTIVECYCLES` writer - Number of module clock cycles minus one wr#/cs#/en terminal should remain active (tCCLW)."]
pub type WRITEACTIVECYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITETIMINGCONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRITEHOLDCYCLES` reader - Number of module clock cycles minus one to wait after wr#/cs#/en terminal became inactive before address can change (tAH)."]
pub type WRITEHOLDCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITEHOLDCYCLES` writer - Number of module clock cycles minus one to wait after wr#/cs#/en terminal became inactive before address can change (tAH)."]
pub type WRITEHOLDCYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITETIMINGCONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Number of module clock cycles minus one to wait before wr#/cs#/en terminal can become active after address set (tAW)."]
    #[inline(always)]
    pub fn writesetupcycles(&self) -> WRITESETUPCYCLES_R {
        WRITESETUPCYCLES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of module clock cycles minus one wr#/cs#/en terminal should remain active (tCCLW)."]
    #[inline(always)]
    pub fn writeactivecycles(&self) -> WRITEACTIVECYCLES_R {
        WRITEACTIVECYCLES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of module clock cycles minus one to wait after wr#/cs#/en terminal became inactive before address can change (tAH)."]
    #[inline(always)]
    pub fn writeholdcycles(&self) -> WRITEHOLDCYCLES_R {
        WRITEHOLDCYCLES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of module clock cycles minus one to wait before wr#/cs#/en terminal can become active after address set (tAW)."]
    #[inline(always)]
    #[must_use]
    pub fn writesetupcycles(&mut self) -> WRITESETUPCYCLES_W<0> {
        WRITESETUPCYCLES_W::new(self)
    }
    #[doc = "Bits 8:15 - Number of module clock cycles minus one wr#/cs#/en terminal should remain active (tCCLW)."]
    #[inline(always)]
    #[must_use]
    pub fn writeactivecycles(&mut self) -> WRITEACTIVECYCLES_W<8> {
        WRITEACTIVECYCLES_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of module clock cycles minus one to wait after wr#/cs#/en terminal became inactive before address can change (tAH)."]
    #[inline(always)]
    #[must_use]
    pub fn writeholdcycles(&mut self) -> WRITEHOLDCYCLES_W<16> {
        WRITEHOLDCYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write timing configuration register. Sum of all fields is write cycle time in module clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writetimingconfig](index.html) module"]
pub struct WRITETIMINGCONFIG_SPEC;
impl crate::RegisterSpec for WRITETIMINGCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writetimingconfig::R](R) reader structure"]
impl crate::Readable for WRITETIMINGCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writetimingconfig::W](W) writer structure"]
impl crate::Writable for WRITETIMINGCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITETIMINGCONFIG to value 0x0004_0804"]
impl crate::Resettable for WRITETIMINGCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0804;
}
