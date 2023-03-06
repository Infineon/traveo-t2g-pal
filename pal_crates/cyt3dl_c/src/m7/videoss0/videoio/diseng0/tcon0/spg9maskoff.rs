#[doc = "Register `SPG9MASKOFF` reader"]
pub struct R(crate::R<SPG9MASKOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG9MASKOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG9MASKOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG9MASKOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG9MASKOFF` writer"]
pub struct W(crate::W<SPG9MASKOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG9MASKOFF_SPEC>;
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
impl From<crate::W<SPG9MASKOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG9MASKOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGMKOFF9` reader - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKOFF9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPGMKOFF9` writer - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
pub type SPGMKOFF9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG9MASKOFF_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    pub fn spgmkoff9(&self) -> SPGMKOFF9_R {
        SPGMKOFF9_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Mask bits 0b=Include this bit in position matching, 1b= Do not include this bit in position matching"]
    #[inline(always)]
    #[must_use]
    pub fn spgmkoff9(&mut self) -> SPGMKOFF9_W<0> {
        SPGMKOFF9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Sequencer Pulse Generator 9 Mask Enable register is used to mask the disable of SPG 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg9maskoff](index.html) module"]
pub struct SPG9MASKOFF_SPEC;
impl crate::RegisterSpec for SPG9MASKOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg9maskoff::R](R) reader structure"]
impl crate::Readable for SPG9MASKOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg9maskoff::W](W) writer structure"]
impl crate::Writable for SPG9MASKOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG9MASKOFF to value 0"]
impl crate::Resettable for SPG9MASKOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
