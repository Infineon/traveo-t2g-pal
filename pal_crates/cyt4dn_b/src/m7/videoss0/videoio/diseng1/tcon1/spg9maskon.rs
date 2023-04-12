#[doc = "Register `SPG9MASKON` reader"]
pub struct R(crate::R<SPG9MASKON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG9MASKON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG9MASKON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG9MASKON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG9MASKON` writer"]
pub struct W(crate::W<SPG9MASKON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG9MASKON_SPEC>;
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
impl From<crate::W<SPG9MASKON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG9MASKON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGMKON9` reader - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKON9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPGMKON9` writer - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKON9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG9MASKON_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    pub fn spgmkon9(&self) -> SPGMKON9_R {
        SPGMKON9_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    #[must_use]
    pub fn spgmkon9(&mut self) -> SPGMKON9_W<0> {
        SPGMKON9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Sequencer Pulse Generator 9 Mask Enable register is used to mask the enable of SPG 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg9maskon](index.html) module"]
pub struct SPG9MASKON_SPEC;
impl crate::RegisterSpec for SPG9MASKON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg9maskon::R](R) reader structure"]
impl crate::Readable for SPG9MASKON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg9maskon::W](W) writer structure"]
impl crate::Writable for SPG9MASKON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG9MASKON to value 0"]
impl crate::Resettable for SPG9MASKON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
