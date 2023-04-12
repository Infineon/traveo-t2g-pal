#[doc = "Register `PTSR11` reader"]
pub struct R(crate::R<PTSR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR11` writer"]
pub struct W(crate::W<PTSR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR11_SPEC>;
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
impl From<crate::W<PTSR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQSDQS3B0` reader - DQS-DQ setting for slice 3 - DQ\\[0\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS3B0` writer - DQS-DQ setting for slice 3 - DQ\\[0\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS3B1` reader - DQS-DQ setting for slice 3 - DQ\\[1\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS3B1` writer - DQS-DQ setting for slice 3 - DQ\\[1\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS3B2` reader - DQS-DQ setting for slice 3 - DQ\\[2\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS3B2` writer - DQS-DQ setting for slice 3 - DQ\\[2\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSDQS3B3` reader - DQS-DQ setting for slice 3 - DQ\\[3\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSDQS3B3` writer - DQS-DQ setting for slice 3 - DQ\\[3\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type DQSDQS3B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR11_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 3 - DQ\\[0\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs3b0(&self) -> DQSDQS3B0_R {
        DQSDQS3B0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 3 - DQ\\[1\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs3b1(&self) -> DQSDQS3B1_R {
        DQSDQS3B1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 3 - DQ\\[2\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs3b2(&self) -> DQSDQS3B2_R {
        DQSDQS3B2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 3 - DQ\\[3\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn dqsdqs3b3(&self) -> DQSDQS3B3_R {
        DQSDQS3B3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DQS-DQ setting for slice 3 - DQ\\[0\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs3b0(&mut self) -> DQSDQS3B0_W<0> {
        DQSDQS3B0_W::new(self)
    }
    #[doc = "Bits 8:15 - DQS-DQ setting for slice 3 - DQ\\[1\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs3b1(&mut self) -> DQSDQS3B1_W<8> {
        DQSDQS3B1_W::new(self)
    }
    #[doc = "Bits 16:23 - DQS-DQ setting for slice 3 - DQ\\[2\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs3b2(&mut self) -> DQSDQS3B2_W<16> {
        DQSDQS3B2_W::new(self)
    }
    #[doc = "Bits 24:31 - DQS-DQ setting for slice 3 - DQ\\[3\\]
Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dqsdqs3b3(&mut self) -> DQSDQS3B3_W<24> {
        DQSDQS3B3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr11](index.html) module"]
pub struct PTSR11_SPEC;
impl crate::RegisterSpec for PTSR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr11::R](R) reader structure"]
impl crate::Readable for PTSR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr11::W](W) writer structure"]
impl crate::Writable for PTSR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR11 to value 0"]
impl crate::Resettable for PTSR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
