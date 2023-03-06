#[doc = "Register `SPG0MASKON` reader"]
pub struct R(crate::R<SPG0MASKON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG0MASKON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG0MASKON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG0MASKON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG0MASKON` writer"]
pub struct W(crate::W<SPG0MASKON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG0MASKON_SPEC>;
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
impl From<crate::W<SPG0MASKON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG0MASKON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGMKON0` reader - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKON0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPGMKON0` writer - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKON0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG0MASKON_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    pub fn spgmkon0(&self) -> SPGMKON0_R {
        SPGMKON0_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    #[must_use]
    pub fn spgmkon0(&mut self) -> SPGMKON0_W<0> {
        SPGMKON0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Sequencer Pulse Generator 0 Mask Enable register is used to mask the enable of SPG 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg0maskon](index.html) module"]
pub struct SPG0MASKON_SPEC;
impl crate::RegisterSpec for SPG0MASKON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg0maskon::R](R) reader structure"]
impl crate::Readable for SPG0MASKON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg0maskon::W](W) writer structure"]
impl crate::Writable for SPG0MASKON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG0MASKON to value 0xffff"]
impl crate::Resettable for SPG0MASKON_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
