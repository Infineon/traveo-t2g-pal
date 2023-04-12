#[doc = "Register `SMX1SIGS` reader"]
pub struct R(crate::R<SMX1SIGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMX1SIGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMX1SIGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMX1SIGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMX1SIGS` writer"]
pub struct W(crate::W<SMX1SIGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMX1SIGS_SPEC>;
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
impl From<crate::W<SMX1SIGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMX1SIGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMX1SIGS_S0` reader - select 0 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMX1SIGS_S0` writer - select 0 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMX1SIGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMX1SIGS_S1` reader - select 1 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMX1SIGS_S1` writer - select 1 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMX1SIGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMX1SIGS_S2` reader - select 2 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMX1SIGS_S2` writer - select 2 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMX1SIGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMX1SIGS_S3` reader - select 3 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMX1SIGS_S3` writer - select 3 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMX1SIGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMX1SIGS_S4` reader - select 4 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMX1SIGS_S4` writer - select 4 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
pub type SMX1SIGS_S4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMX1SIGS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - select 0 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    pub fn smx1sigs_s0(&self) -> SMX1SIGS_S0_R {
        SMX1SIGS_S0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - select 1 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    pub fn smx1sigs_s1(&self) -> SMX1SIGS_S1_R {
        SMX1SIGS_S1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - select 2 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    pub fn smx1sigs_s2(&self) -> SMX1SIGS_S2_R {
        SMX1SIGS_S2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - select 3 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    pub fn smx1sigs_s3(&self) -> SMX1SIGS_S3_R {
        SMX1SIGS_S3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - select 4 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    pub fn smx1sigs_s4(&self) -> SMX1SIGS_S4_R {
        SMX1SIGS_S4_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - select 0 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    #[must_use]
    pub fn smx1sigs_s0(&mut self) -> SMX1SIGS_S0_W<0> {
        SMX1SIGS_S0_W::new(self)
    }
    #[doc = "Bits 3:5 - select 1 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    #[must_use]
    pub fn smx1sigs_s1(&mut self) -> SMX1SIGS_S1_W<3> {
        SMX1SIGS_S1_W::new(self)
    }
    #[doc = "Bits 6:8 - select 2 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    #[must_use]
    pub fn smx1sigs_s2(&mut self) -> SMX1SIGS_S2_W<6> {
        SMX1SIGS_S2_W::new(self)
    }
    #[doc = "Bits 9:11 - select 3 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    #[must_use]
    pub fn smx1sigs_s3(&mut self) -> SMX1SIGS_S3_W<9> {
        SMX1SIGS_S3_W::new(self)
    }
    #[doc = "Bits 12:14 - select 4 000b=constant 0, 001b=sync sequencer output, 010b...111b sync pulse generator SPG0 .. SPG5"]
    #[inline(always)]
    #[must_use]
    pub fn smx1sigs_s4(&mut self) -> SMX1SIGS_S4_W<12> {
        SMX1SIGS_S4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selection of input signals of sync mixer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smx1sigs](index.html) module"]
pub struct SMX1SIGS_SPEC;
impl crate::RegisterSpec for SMX1SIGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smx1sigs::R](R) reader structure"]
impl crate::Readable for SMX1SIGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smx1sigs::W](W) writer structure"]
impl crate::Writable for SMX1SIGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMX1SIGS to value 0x03"]
impl crate::Resettable for SMX1SIGS_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
