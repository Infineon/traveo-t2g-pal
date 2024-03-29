#[doc = "Register `PTSR21` reader"]
pub struct R(crate::R<PTSR21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR21` writer"]
pub struct W(crate::W<PTSR21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR21_SPEC>;
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
impl From<crate::W<PTSR21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDLVLDQS3B4` reader - Read DQ setting for slice 3 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS3B4` writer - Read DQ setting for slice 3 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS3B5` reader - Read DQ setting for slice 3 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS3B5` writer - Read DQ setting for slice 3 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS3B6` reader - Read DQ setting for slice 3 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS3B6` writer - Read DQ setting for slice 3 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDLVLDQS3B7` reader - Read DQ setting for slice 3 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDQS3B7` writer - Read DQ setting for slice 3 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
pub type RDLVLDQS3B7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR21_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read DQ setting for slice 3 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs3b4(&self) -> RDLVLDQS3B4_R {
        RDLVLDQS3B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 3 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs3b5(&self) -> RDLVLDQS3B5_R {
        RDLVLDQS3B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 3 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs3b6(&self) -> RDLVLDQS3B6_R {
        RDLVLDQS3B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 3 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    pub fn rdlvldqs3b7(&self) -> RDLVLDQS3B7_R {
        RDLVLDQS3B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read DQ setting for slice 3 bit 4 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs3b4(&mut self) -> RDLVLDQS3B4_W<0> {
        RDLVLDQS3B4_W::new(self)
    }
    #[doc = "Bits 8:15 - Read DQ setting for slice 3 bit 5 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs3b5(&mut self) -> RDLVLDQS3B5_W<8> {
        RDLVLDQS3B5_W::new(self)
    }
    #[doc = "Bits 16:23 - Read DQ setting for slice 3 bit 6 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs3b6(&mut self) -> RDLVLDQS3B6_W<16> {
        RDLVLDQS3B6_W::new(self)
    }
    #[doc = "Bits 24:31 - Read DQ setting for slice 3 bit 7 Bits \\[6:0\\]
contain the delay setting value. Bit \\[7\\]
always returns 0 when being read. Always write 0 to Bit \\[7\\]. Writing '1' to Bit \\[7\\]
may cause malfunction of the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvldqs3b7(&mut self) -> RDLVLDQS3B7_W<24> {
        RDLVLDQS3B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr21](index.html) module"]
pub struct PTSR21_SPEC;
impl crate::RegisterSpec for PTSR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr21::R](R) reader structure"]
impl crate::Readable for PTSR21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr21::W](W) writer structure"]
impl crate::Writable for PTSR21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR21 to value 0"]
impl crate::Resettable for PTSR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
