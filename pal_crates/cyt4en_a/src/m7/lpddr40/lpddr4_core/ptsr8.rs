#[doc = "Register `PTSR8` reader"]
pub struct R(crate::R<PTSR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR8` writer"]
pub struct W(crate::W<PTSR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR8_SPEC>;
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
impl From<crate::W<PTSR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQSDQS1B4` reader - DQS-DQ setting for slice 1 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS1B4` writer - DQS-DQ setting for slice 1 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS1B5` reader - DQS-DQ setting for slice 1 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS1B5` writer - DQS-DQ setting for slice 1 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS1B6` reader - DQS-DQ setting for slice 1 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS1B6` writer - DQS-DQ setting for slice 1 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS1B7` reader - DQS-DQ setting for slice 1 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS1B7` writer - DQS-DQ setting for slice 1 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS1B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 1 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs1b4(&self) -> DQSDQS1B4_R {
        DQSDQS1B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 1 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs1b5(&self) -> DQSDQS1B5_R {
        DQSDQS1B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 1 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs1b6(&self) -> DQSDQS1B6_R {
        DQSDQS1B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 1 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs1b7(&self) -> DQSDQS1B7_R {
        DQSDQS1B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 1 - DQ\\[4\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs1b4(&mut self) -> DQSDQS1B4_W<0> {
        DQSDQS1B4_W::new(self)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 1 - DQ\\[5\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs1b5(&mut self) -> DQSDQS1B5_W<8> {
        DQSDQS1B5_W::new(self)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 1 - DQ\\[6\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs1b6(&mut self) -> DQSDQS1B6_W<16> {
        DQSDQS1B6_W::new(self)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 1 - DQ\\[7\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs1b7(&mut self) -> DQSDQS1B7_W<24> {
        DQSDQS1B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr8](index.html) module"]
pub struct PTSR8_SPEC;
impl crate::RegisterSpec for PTSR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr8::R](R) reader structure"]
impl crate::Readable for PTSR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr8::W](W) writer structure"]
impl crate::Writable for PTSR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR8 to value 0"]
impl crate::Resettable for PTSR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
