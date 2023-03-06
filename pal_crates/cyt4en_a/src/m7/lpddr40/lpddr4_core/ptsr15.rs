#[doc = "Register `PTSR15` reader"]
pub struct R(crate::R<PTSR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR15` writer"]
pub struct W(crate::W<PTSR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR15_SPEC>;
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
impl From<crate::W<PTSR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLVLDQS0B4` reader - Read DQ setting for slice 0 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS0B4` writer - Read DQ setting for slice 0 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR15_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS0B5` reader - Read DQ setting for slice 0 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS0B5` writer - Read DQ setting for slice 0 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR15_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS0B6` reader - Read DQ setting for slice 0 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS0B6` writer - Read DQ setting for slice 0 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR15_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS0B7` reader - Read DQ setting for slice 0 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS0B7` writer - Read DQ setting for slice 0 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS0B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR15_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read DQ setting for slice 0 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs0b4(&self) -> RDLVLDQS0B4_R {
        RDLVLDQS0B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 0 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs0b5(&self) -> RDLVLDQS0B5_R {
        RDLVLDQS0B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 0 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs0b6(&self) -> RDLVLDQS0B6_R {
        RDLVLDQS0B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 0 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs0b7(&self) -> RDLVLDQS0B7_R {
        RDLVLDQS0B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read DQ setting for slice 0 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs0b4(&mut self) -> RDLVLDQS0B4_W<0> {
        RDLVLDQS0B4_W::new(self)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 0 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs0b5(&mut self) -> RDLVLDQS0B5_W<8> {
        RDLVLDQS0B5_W::new(self)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 0 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs0b6(&mut self) -> RDLVLDQS0B6_W<16> {
        RDLVLDQS0B6_W::new(self)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 0 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs0b7(&mut self) -> RDLVLDQS0B7_W<24> {
        RDLVLDQS0B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr15](index.html) module"]
pub struct PTSR15_SPEC;
impl crate::RegisterSpec for PTSR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr15::R](R) reader structure"]
impl crate::Readable for PTSR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr15::W](W) writer structure"]
impl crate::Writable for PTSR15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR15 to value 0"]
impl crate::Resettable for PTSR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
