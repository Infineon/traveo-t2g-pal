#[doc = "Register `CFG_DRIVE_EXT1` reader"]
pub struct R(crate::R<CFG_DRIVE_EXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DRIVE_EXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DRIVE_EXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DRIVE_EXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DRIVE_EXT1` writer"]
pub struct W(crate::W<CFG_DRIVE_EXT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DRIVE_EXT1_SPEC>;
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
impl From<crate::W<CFG_DRIVE_EXT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DRIVE_EXT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVE_SEL_EXT4` reader - Sets the GPIO drive strength for IO pin 4"]
pub type DRIVE_SEL_EXT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT4` writer - Sets the GPIO drive strength for IO pin 4"]
pub type DRIVE_SEL_EXT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT5` reader - Sets the GPIO drive strength for IO pin 5"]
pub type DRIVE_SEL_EXT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT5` writer - Sets the GPIO drive strength for IO pin 5"]
pub type DRIVE_SEL_EXT5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT6` reader - Sets the GPIO drive strength for IO pin 6"]
pub type DRIVE_SEL_EXT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT6` writer - Sets the GPIO drive strength for IO pin 6"]
pub type DRIVE_SEL_EXT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DRIVE_SEL_EXT7` reader - Sets the GPIO drive strength for IO pin 7"]
pub type DRIVE_SEL_EXT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_SEL_EXT7` writer - Sets the GPIO drive strength for IO pin 7"]
pub type DRIVE_SEL_EXT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DRIVE_EXT1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel_ext4(&self) -> DRIVE_SEL_EXT4_R {
        DRIVE_SEL_EXT4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel_ext5(&self) -> DRIVE_SEL_EXT5_R {
        DRIVE_SEL_EXT5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel_ext6(&self) -> DRIVE_SEL_EXT6_R {
        DRIVE_SEL_EXT6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel_ext7(&self) -> DRIVE_SEL_EXT7_R {
        DRIVE_SEL_EXT7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext4(&mut self) -> DRIVE_SEL_EXT4_W<0> {
        DRIVE_SEL_EXT4_W::new(self)
    }
    #[doc = "Bits 8:12 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext5(&mut self) -> DRIVE_SEL_EXT5_W<8> {
        DRIVE_SEL_EXT5_W::new(self)
    }
    #[doc = "Bits 16:20 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext6(&mut self) -> DRIVE_SEL_EXT6_W<16> {
        DRIVE_SEL_EXT6_W::new(self)
    }
    #[doc = "Bits 24:28 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel_ext7(&mut self) -> DRIVE_SEL_EXT7_W<24> {
        DRIVE_SEL_EXT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output buffer drive sel extension configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_drive_ext1](index.html) module"]
pub struct CFG_DRIVE_EXT1_SPEC;
impl crate::RegisterSpec for CFG_DRIVE_EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_drive_ext1::R](R) reader structure"]
impl crate::Readable for CFG_DRIVE_EXT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_drive_ext1::W](W) writer structure"]
impl crate::Writable for CFG_DRIVE_EXT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DRIVE_EXT1 to value 0"]
impl crate::Resettable for CFG_DRIVE_EXT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
