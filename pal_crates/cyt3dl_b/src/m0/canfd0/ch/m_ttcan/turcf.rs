#[doc = "Register `TURCF` reader"]
pub struct R(crate::R<TURCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TURCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TURCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TURCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TURCF` writer"]
pub struct W(crate::W<TURCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TURCF_SPEC>;
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
impl From<crate::W<TURCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TURCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
pub type NCL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NCL` writer - Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
pub type NCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TURCF_SPEC, u16, u16, 16, O>;
#[doc = "Field `DC` reader - Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
pub type DC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DC` writer - Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
pub type DC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TURCF_SPEC, u16, u16, 14, O>;
#[doc = "Field `ELT` reader - Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
pub type ELT_R = crate::BitReader<bool>;
#[doc = "Field `ELT` writer - Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
pub type ELT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TURCF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Numerator Configuration Low Write access to the TUR Numerator Configuration Low is only possible during configuration with TURCF.ELT = '0' or if TTOCF.EECS (external clock synchronization enabled) is set. When a new value for NCL is written outside TT Configuration Mode, the new value takes effect when TTOST.WECS is cleared to '0'. NCL is locked TTOST.WECS is '1'. 0x0000-FFFF Numerator Configuration Low"]
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<0> {
        NCL_W::new(self)
    }
    #[doc = "Bits 16:29 - Denominator Configuration 0x0000 Illegal value 0x0001-3FFF Denominator Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<16> {
        DC_W::new(self)
    }
    #[doc = "Bit 31 - Enable Local Time 0= Local time is stopped, default 1= Local time is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn elt(&mut self) -> ELT_W<31> {
        ELT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TUR Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [turcf](index.html) module"]
pub struct TURCF_SPEC;
impl crate::RegisterSpec for TURCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [turcf::R](R) reader structure"]
impl crate::Readable for TURCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [turcf::W](W) writer structure"]
impl crate::Writable for TURCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TURCF to value 0x1000_0000"]
impl crate::Resettable for TURCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
