#[doc = "Register `PTSR13` reader"]
pub struct R(crate::R<PTSR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR13` writer"]
pub struct W(crate::W<PTSR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR13_SPEC>;
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
impl From<crate::W<PTSR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQSDMS0` reader - DQS-DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDMS0` writer - DQS-DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDMS1` reader - DQS-DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDMS1` writer - DQS-DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDMS2` reader - DQS-DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDMS2` writer - DQS-DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDMS3` reader - DQS-DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDMS3` writer - DQS-DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDMS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR13_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DQS-DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdms0(&self) -> DQSDMS0_R {
        DQSDMS0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DQS-DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdms1(&self) -> DQSDMS1_R {
        DQSDMS1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DQS-DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdms2(&self) -> DQSDMS2_R {
        DQSDMS2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DQS-DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdms3(&self) -> DQSDMS3_R {
        DQSDMS3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS-DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdms0(&mut self) -> DQSDMS0_W<0> {
        DQSDMS0_W::new(self)
    }
    #[doc = "Bits 8:15 - DQS-DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdms1(&mut self) -> DQSDMS1_W<8> {
        DQSDMS1_W::new(self)
    }
    #[doc = "Bits 16:23 - DQS-DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdms2(&mut self) -> DQSDMS2_W<16> {
        DQSDMS2_W::new(self)
    }
    #[doc = "Bits 24:31 - DQS-DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdms3(&mut self) -> DQSDMS3_W<24> {
        DQSDMS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr13](index.html) module"]
pub struct PTSR13_SPEC;
impl crate::RegisterSpec for PTSR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr13::R](R) reader structure"]
impl crate::Readable for PTSR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr13::W](W) writer structure"]
impl crate::Writable for PTSR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR13 to value 0"]
impl crate::Resettable for PTSR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
