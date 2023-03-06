#[doc = "Register `FIRPOSITIONS` reader"]
pub struct R(crate::R<FIRPOSITIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIRPOSITIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIRPOSITIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIRPOSITIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIRPOSITIONS` writer"]
pub struct W(crate::W<FIRPOSITIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRPOSITIONS_SPEC>;
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
impl From<crate::W<FIRPOSITIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRPOSITIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIR0POSITION` reader - Position of first pixel."]
pub type FIR0POSITION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR0POSITION` writer - Position of first pixel."]
pub type FIR0POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRPOSITIONS_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIR1POSITION` reader - Position of second pixel."]
pub type FIR1POSITION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR1POSITION` writer - Position of second pixel."]
pub type FIR1POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRPOSITIONS_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIR2POSITION` reader - Position of third pixel."]
pub type FIR2POSITION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR2POSITION` writer - Position of third pixel."]
pub type FIR2POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRPOSITIONS_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIR3POSITION` reader - Position of fourth pixel."]
pub type FIR3POSITION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR3POSITION` writer - Position of fourth pixel."]
pub type FIR3POSITION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRPOSITIONS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Position of first pixel."]
    #[inline(always)]
    pub fn fir0position(&self) -> FIR0POSITION_R {
        FIR0POSITION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Position of second pixel."]
    #[inline(always)]
    pub fn fir1position(&self) -> FIR1POSITION_R {
        FIR1POSITION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Position of third pixel."]
    #[inline(always)]
    pub fn fir2position(&self) -> FIR2POSITION_R {
        FIR2POSITION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Position of fourth pixel."]
    #[inline(always)]
    pub fn fir3position(&self) -> FIR3POSITION_R {
        FIR3POSITION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Position of first pixel."]
    #[inline(always)]
    #[must_use]
    pub fn fir0position(&mut self) -> FIR0POSITION_W<0> {
        FIR0POSITION_W::new(self)
    }
    #[doc = "Bits 4:7 - Position of second pixel."]
    #[inline(always)]
    #[must_use]
    pub fn fir1position(&mut self) -> FIR1POSITION_W<4> {
        FIR1POSITION_W::new(self)
    }
    #[doc = "Bits 8:11 - Position of third pixel."]
    #[inline(always)]
    #[must_use]
    pub fn fir2position(&mut self) -> FIR2POSITION_W<8> {
        FIR2POSITION_W::new(self)
    }
    #[doc = "Bits 12:15 - Position of fourth pixel."]
    #[inline(always)]
    #[must_use]
    pub fn fir3position(&mut self) -> FIR3POSITION_W<12> {
        FIR3POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIR sequence control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firpositions](index.html) module"]
pub struct FIRPOSITIONS_SPEC;
impl crate::RegisterSpec for FIRPOSITIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [firpositions::R](R) reader structure"]
impl crate::Readable for FIRPOSITIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [firpositions::W](W) writer structure"]
impl crate::Writable for FIRPOSITIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIRPOSITIONS to value 0xa965"]
impl crate::Resettable for FIRPOSITIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0xa965;
}
