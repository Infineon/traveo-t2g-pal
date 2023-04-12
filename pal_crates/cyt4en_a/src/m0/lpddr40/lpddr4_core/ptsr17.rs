#[doc = "Register `PTSR17` reader"]
pub struct R(crate::R<PTSR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR17` writer"]
pub struct W(crate::W<PTSR17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR17_SPEC>;
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
impl From<crate::W<PTSR17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLVLDQS1B4` reader - Read DQ setting for slice 1 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS1B4` writer - Read DQ setting for slice 1 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS1B5` reader - Read DQ setting for slice 1 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS1B5` writer - Read DQ setting for slice 1 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS1B6` reader - Read DQ setting for slice 1 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS1B6` writer - Read DQ setting for slice 1 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS1B7` reader - Read DQ setting for slice 1 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS1B7` writer - Read DQ setting for slice 1 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS1B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR17_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read DQ setting for slice 1 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs1b4(&self) -> RDLVLDQS1B4_R {
        RDLVLDQS1B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 1 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs1b5(&self) -> RDLVLDQS1B5_R {
        RDLVLDQS1B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 1 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs1b6(&self) -> RDLVLDQS1B6_R {
        RDLVLDQS1B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 1 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs1b7(&self) -> RDLVLDQS1B7_R {
        RDLVLDQS1B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read DQ setting for slice 1 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs1b4(&mut self) -> RDLVLDQS1B4_W<0> {
        RDLVLDQS1B4_W::new(self)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 1 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs1b5(&mut self) -> RDLVLDQS1B5_W<8> {
        RDLVLDQS1B5_W::new(self)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 1 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs1b6(&mut self) -> RDLVLDQS1B6_W<16> {
        RDLVLDQS1B6_W::new(self)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 1 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs1b7(&mut self) -> RDLVLDQS1B7_W<24> {
        RDLVLDQS1B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr17](index.html) module"]
pub struct PTSR17_SPEC;
impl crate::RegisterSpec for PTSR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr17::R](R) reader structure"]
impl crate::Readable for PTSR17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr17::W](W) writer structure"]
impl crate::Writable for PTSR17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR17 to value 0"]
impl crate::Resettable for PTSR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
