#[doc = "Register `CFG_SLEW_EXT` reader"]
pub struct R(crate::R<CFG_SLEW_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SLEW_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SLEW_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SLEW_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_SLEW_EXT` writer"]
pub struct W(crate::W<CFG_SLEW_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SLEW_EXT_SPEC>;
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
impl From<crate::W<CFG_SLEW_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SLEW_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEW0` reader - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type SLEW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW0` writer - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type SLEW0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW1` reader - Slew rate for IO pin 1"]
pub type SLEW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW1` writer - Slew rate for IO pin 1"]
pub type SLEW1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW2` reader - Slew rate for IO pin 2"]
pub type SLEW2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW2` writer - Slew rate for IO pin 2"]
pub type SLEW2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW3` reader - Slew rate for IO pin 3"]
pub type SLEW3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW3` writer - Slew rate for IO pin 3"]
pub type SLEW3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW4` reader - Slew rate for IO pin 4"]
pub type SLEW4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW4` writer - Slew rate for IO pin 4"]
pub type SLEW4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW5` reader - Slew rate for IO pin 5"]
pub type SLEW5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW5` writer - Slew rate for IO pin 5"]
pub type SLEW5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW6` reader - Slew rate for IO pin 6"]
pub type SLEW6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW6` writer - Slew rate for IO pin 6"]
pub type SLEW6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLEW7` reader - Slew rate for IO pin 7"]
pub type SLEW7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEW7` writer - Slew rate for IO pin 7"]
pub type SLEW7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SLEW_EXT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slew0(&self) -> SLEW0_R {
        SLEW0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slew1(&self) -> SLEW1_R {
        SLEW1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slew2(&self) -> SLEW2_R {
        SLEW2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slew3(&self) -> SLEW3_R {
        SLEW3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slew4(&self) -> SLEW4_R {
        SLEW4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slew5(&self) -> SLEW5_R {
        SLEW5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slew6(&self) -> SLEW6_R {
        SLEW6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slew7(&self) -> SLEW7_R {
        SLEW7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn slew0(&mut self) -> SLEW0_W<0> {
        SLEW0_W::new(self)
    }
    #[doc = "Bits 4:6 - Slew rate for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn slew1(&mut self) -> SLEW1_W<4> {
        SLEW1_W::new(self)
    }
    #[doc = "Bits 8:10 - Slew rate for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn slew2(&mut self) -> SLEW2_W<8> {
        SLEW2_W::new(self)
    }
    #[doc = "Bits 12:14 - Slew rate for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn slew3(&mut self) -> SLEW3_W<12> {
        SLEW3_W::new(self)
    }
    #[doc = "Bits 16:18 - Slew rate for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn slew4(&mut self) -> SLEW4_W<16> {
        SLEW4_W::new(self)
    }
    #[doc = "Bits 20:22 - Slew rate for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn slew5(&mut self) -> SLEW5_W<20> {
        SLEW5_W::new(self)
    }
    #[doc = "Bits 24:26 - Slew rate for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn slew6(&mut self) -> SLEW6_W<24> {
        SLEW6_W::new(self)
    }
    #[doc = "Bits 28:30 - Slew rate for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn slew7(&mut self) -> SLEW7_W<28> {
        SLEW7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output buffer slew extension configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_slew_ext](index.html) module"]
pub struct CFG_SLEW_EXT_SPEC;
impl crate::RegisterSpec for CFG_SLEW_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_slew_ext::R](R) reader structure"]
impl crate::Readable for CFG_SLEW_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_slew_ext::W](W) writer structure"]
impl crate::Writable for CFG_SLEW_EXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_SLEW_EXT to value 0"]
impl crate::Resettable for CFG_SLEW_EXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
