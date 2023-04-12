#[doc = "Register `PTSR22` reader"]
pub struct R(crate::R<PTSR22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR22` writer"]
pub struct W(crate::W<PTSR22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR22_SPEC>;
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
impl From<crate::W<PTSR22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLVLDMS0` reader - Read DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDMS0` writer - Read DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR22_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDMS1` reader - Read DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDMS1` writer - Read DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR22_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDMS2` reader - Read DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDMS2` writer - Read DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR22_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDMS3` reader - Read DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDMS3` writer - Read DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDMS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR22_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldms0(&self) -> RDLVLDMS0_R {
        RDLVLDMS0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldms1(&self) -> RDLVLDMS1_R {
        RDLVLDMS1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldms2(&self) -> RDLVLDMS2_R {
        RDLVLDMS2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldms3(&self) -> RDLVLDMS3_R {
        RDLVLDMS3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read DM setting for slice 0 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldms0(&mut self) -> RDLVLDMS0_W<0> {
        RDLVLDMS0_W::new(self)
    }
    #[doc = "Bits 8:15 - Read DM setting for slice 1 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldms1(&mut self) -> RDLVLDMS1_W<8> {
        RDLVLDMS1_W::new(self)
    }
    #[doc = "Bits 16:23 - Read DM setting for slice 2 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldms2(&mut self) -> RDLVLDMS2_W<16> {
        RDLVLDMS2_W::new(self)
    }
    #[doc = "Bits 24:31 - Read DM setting for slice 3 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldms3(&mut self) -> RDLVLDMS3_W<24> {
        RDLVLDMS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr22](index.html) module"]
pub struct PTSR22_SPEC;
impl crate::RegisterSpec for PTSR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr22::R](R) reader structure"]
impl crate::Readable for PTSR22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr22::W](W) writer structure"]
impl crate::Writable for PTSR22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR22 to value 0"]
impl crate::Resettable for PTSR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
