#[doc = "Register `PTSR4` reader"]
pub struct R(crate::R<PTSR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR4` writer"]
pub struct W(crate::W<PTSR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR4_SPEC>;
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
impl From<crate::W<PTSR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRLVLS0` reader - Write leveling setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRLVLS0` writer - Write leveling setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRLVLS1` reader - Write leveling setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRLVLS1` writer - Write leveling setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRLVLS2` reader - Write leveling setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRLVLS2` writer - Write leveling setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRLVLS3` reader - Write leveling setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRLVLS3` writer - Write leveling setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type WRLVLS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Write leveling setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn wrlvls0(&self) -> WRLVLS0_R {
        WRLVLS0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Write leveling setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn wrlvls1(&self) -> WRLVLS1_R {
        WRLVLS1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write leveling setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn wrlvls2(&self) -> WRLVLS2_R {
        WRLVLS2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Write leveling setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn wrlvls3(&self) -> WRLVLS3_R {
        WRLVLS3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write leveling setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvls0(&mut self) -> WRLVLS0_W<0> {
        WRLVLS0_W::new(self)
    }
    #[doc = "Bits 8:15 - Write leveling setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvls1(&mut self) -> WRLVLS1_W<8> {
        WRLVLS1_W::new(self)
    }
    #[doc = "Bits 16:23 - Write leveling setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvls2(&mut self) -> WRLVLS2_W<16> {
        WRLVLS2_W::new(self)
    }
    #[doc = "Bits 24:31 - Write leveling setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvls3(&mut self) -> WRLVLS3_W<24> {
        WRLVLS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr4](index.html) module"]
pub struct PTSR4_SPEC;
impl crate::RegisterSpec for PTSR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr4::R](R) reader structure"]
impl crate::Readable for PTSR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr4::W](W) writer structure"]
impl crate::Writable for PTSR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR4 to value 0"]
impl crate::Resettable for PTSR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
