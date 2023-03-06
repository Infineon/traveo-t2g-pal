#[doc = "Register `PTSR18` reader"]
pub struct R(crate::R<PTSR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR18` writer"]
pub struct W(crate::W<PTSR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR18_SPEC>;
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
impl From<crate::W<PTSR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLVLDQS2B0` reader - Read DQ setting for slice 2 bit 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS2B0` writer - Read DQ setting for slice 2 bit 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS2B1` reader - Read DQ setting for slice 2 bit 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS2B1` writer - Read DQ setting for slice 2 bit 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS2B2` reader - Read DQ setting for slice 2 bit 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS2B2` writer - Read DQ setting for slice 2 bit 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS2B3` reader - Read DQ setting for slice 2 bit 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS2B3` writer - Read DQ setting for slice 2 bit 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS2B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR18_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read DQ setting for slice 2 bit 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs2b0(&self) -> RDLVLDQS2B0_R {
        RDLVLDQS2B0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 2 bit 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs2b1(&self) -> RDLVLDQS2B1_R {
        RDLVLDQS2B1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 2 bit 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs2b2(&self) -> RDLVLDQS2B2_R {
        RDLVLDQS2B2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 2 bit 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs2b3(&self) -> RDLVLDQS2B3_R {
        RDLVLDQS2B3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read DQ setting for slice 2 bit 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs2b0(&mut self) -> RDLVLDQS2B0_W<0> {
        RDLVLDQS2B0_W::new(self)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 2 bit 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs2b1(&mut self) -> RDLVLDQS2B1_W<8> {
        RDLVLDQS2B1_W::new(self)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 2 bit 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs2b2(&mut self) -> RDLVLDQS2B2_W<16> {
        RDLVLDQS2B2_W::new(self)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 2 bit 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs2b3(&mut self) -> RDLVLDQS2B3_W<24> {
        RDLVLDQS2B3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr18](index.html) module"]
pub struct PTSR18_SPEC;
impl crate::RegisterSpec for PTSR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr18::R](R) reader structure"]
impl crate::Readable for PTSR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr18::W](W) writer structure"]
impl crate::Writable for PTSR18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR18 to value 0"]
impl crate::Resettable for PTSR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
