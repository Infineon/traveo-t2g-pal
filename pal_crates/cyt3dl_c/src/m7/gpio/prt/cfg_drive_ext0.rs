#[doc = "Register `CFG_DRIVE_EXT0` reader"]
pub struct R(crate::R<CFG_DRIVE_EXT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DRIVE_EXT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DRIVE_EXT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DRIVE_EXT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DRIVE_EXT0` writer"]
pub struct W(crate::W<CFG_DRIVE_EXT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DRIVE_EXT0_SPEC>;
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
impl From<crate::W<CFG_DRIVE_EXT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DRIVE_EXT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVE_SEL_EXT0` reader - For GPIO OVT 1.2V variant: Below are the encoding values: 3'b000 : 4mA drive strength, 1.8V 3'b001 : 3mA drive strength, 1.8V 3'b010 : 2mA drive strength, 1.8V 3'b011 : 1mA drive strength, 1.8V 3'b100 : 4mA drive strength, 1.2V 3'b101 : 3mA drive strength, 1.2V 3'b110 : 2mA drive strength, 1.2V 3'b111 : 1mA drive strength, 1.2V"]
pub type DRIVE_SEL_EXT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT0` writer - For GPIO OVT 1.2V variant: Below are the encoding values: 3'b000 : 4mA drive strength, 1.8V 3'b001 : 3mA drive strength, 1.8V 3'b010 : 2mA drive strength, 1.8V 3'b011 : 1mA drive strength, 1.8V 3'b100 : 4mA drive strength, 1.2V 3'b101 : 3mA drive strength, 1.2V 3'b110 : 2mA drive strength, 1.2V 3'b111 : 1mA drive strength, 1.2V"]
pub type DRIVE_SEL_EXT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT1` reader - Sets the GPIO drive strength for IO pin 1"]
pub type DRIVE_SEL_EXT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT1` writer - Sets the GPIO drive strength for IO pin 1"]
pub type DRIVE_SEL_EXT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT2` reader - Sets the GPIO drive strength for IO pin 2"]
pub type DRIVE_SEL_EXT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT2` writer - Sets the GPIO drive strength for IO pin 2"]
pub type DRIVE_SEL_EXT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT3` reader - Sets the GPIO drive strength for IO pin 3"]
pub type DRIVE_SEL_EXT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT3` writer - Sets the GPIO drive strength for IO pin 3"]
pub type DRIVE_SEL_EXT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - For GPIO OVT 1.2V variant: Below are the encoding values: 3'b000 : 4mA drive strength, 1.8V 3'b001 : 3mA drive strength, 1.8V 3'b010 : 2mA drive strength, 1.8V 3'b011 : 1mA drive strength, 1.8V 3'b100 : 4mA drive strength, 1.2V 3'b101 : 3mA drive strength, 1.2V 3'b110 : 2mA drive strength, 1.2V 3'b111 : 1mA drive strength, 1.2V"]
    #[inline(always)]
    pub fn drive_sel_ext0(&self) -> DRIVE_SEL_EXT0_R {
        DRIVE_SEL_EXT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel_ext1(&self) -> DRIVE_SEL_EXT1_R {
        DRIVE_SEL_EXT1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel_ext2(&self) -> DRIVE_SEL_EXT2_R {
        DRIVE_SEL_EXT2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel_ext3(&self) -> DRIVE_SEL_EXT3_R {
        DRIVE_SEL_EXT3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - For GPIO OVT 1.2V variant: Below are the encoding values: 3'b000 : 4mA drive strength, 1.8V 3'b001 : 3mA drive strength, 1.8V 3'b010 : 2mA drive strength, 1.8V 3'b011 : 1mA drive strength, 1.8V 3'b100 : 4mA drive strength, 1.2V 3'b101 : 3mA drive strength, 1.2V 3'b110 : 2mA drive strength, 1.2V 3'b111 : 1mA drive strength, 1.2V"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext0(&mut self) -> DRIVE_SEL_EXT0_W<0> {
        DRIVE_SEL_EXT0_W::new(self)
    }
    #[doc = "Bits 8:12 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext1(&mut self) -> DRIVE_SEL_EXT1_W<8> {
        DRIVE_SEL_EXT1_W::new(self)
    }
    #[doc = "Bits 16:20 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext2(&mut self) -> DRIVE_SEL_EXT2_W<16> {
        DRIVE_SEL_EXT2_W::new(self)
    }
    #[doc = "Bits 24:28 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext3(&mut self) -> DRIVE_SEL_EXT3_W<24> {
        DRIVE_SEL_EXT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output buffer drive sel extension configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_drive_ext0](index.html) module"]
pub struct CFG_DRIVE_EXT0_SPEC;
impl crate::RegisterSpec for CFG_DRIVE_EXT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_drive_ext0::R](R) reader structure"]
impl crate::Readable for CFG_DRIVE_EXT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_drive_ext0::W](W) writer structure"]
impl crate::Writable for CFG_DRIVE_EXT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DRIVE_EXT0 to value 0"]
impl crate::Resettable for CFG_DRIVE_EXT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
