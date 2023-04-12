#[doc = "Register `PTSR10` reader"]
pub struct R(crate::R<PTSR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR10` writer"]
pub struct W(crate::W<PTSR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR10_SPEC>;
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
impl From<crate::W<PTSR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQSDQS2B4` reader - DQS-DQ setting for slice 2 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS2B4` writer - DQS-DQ setting for slice 2 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS2B5` reader - DQS-DQ setting for slice 2 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS2B5` writer - DQS-DQ setting for slice 2 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS2B6` reader - DQS-DQ setting for slice 2 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS2B6` writer - DQS-DQ setting for slice 2 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS2B7` reader - DQS-DQ setting for slice 2 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS2B7` writer - DQS-DQ setting for slice 2 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS2B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR10_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 2 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs2b4(&self) -> DQSDQS2B4_R {
        DQSDQS2B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 2 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs2b5(&self) -> DQSDQS2B5_R {
        DQSDQS2B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 2 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs2b6(&self) -> DQSDQS2B6_R {
        DQSDQS2B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 2 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs2b7(&self) -> DQSDQS2B7_R {
        DQSDQS2B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 2 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs2b4(&mut self) -> DQSDQS2B4_W<0> {
        DQSDQS2B4_W::new(self)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 2 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs2b5(&mut self) -> DQSDQS2B5_W<8> {
        DQSDQS2B5_W::new(self)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 2 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs2b6(&mut self) -> DQSDQS2B6_W<16> {
        DQSDQS2B6_W::new(self)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 2 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs2b7(&mut self) -> DQSDQS2B7_W<24> {
        DQSDQS2B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr10](index.html) module"]
pub struct PTSR10_SPEC;
impl crate::RegisterSpec for PTSR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr10::R](R) reader structure"]
impl crate::Readable for PTSR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr10::W](W) writer structure"]
impl crate::Writable for PTSR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR10 to value 0"]
impl crate::Resettable for PTSR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
