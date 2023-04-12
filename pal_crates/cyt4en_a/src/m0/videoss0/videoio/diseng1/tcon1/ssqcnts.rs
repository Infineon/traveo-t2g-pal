#[doc = "Register `SSQCNTS[%s]` reader"]
pub struct R(crate::R<SSQCNTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSQCNTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSQCNTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSQCNTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSQCNTS[%s]` writer"]
pub struct W(crate::W<SSQCNTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSQCNTS_SPEC>;
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
impl From<crate::W<SSQCNTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSQCNTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSQCNTS_SEQY` reader - Y scan position"]
pub type SSQCNTS_SEQY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSQCNTS_SEQY` writer - Y scan position"]
pub type SSQCNTS_SEQY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SSQCNTS_SPEC, u16, u16, 15, O>;
#[doc = "Field `SSQCNTS_FIELD` reader - Field 0b=odd field, 1b=even field"]
pub type SSQCNTS_FIELD_R = crate::BitReader<bool>;
#[doc = "Field `SSQCNTS_FIELD` writer - Field 0b=odd field, 1b=even field"]
pub type SSQCNTS_FIELD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSQCNTS_SPEC, bool, O>;
#[doc = "Field `SSQCNTS_SEQX` reader - X scan position"]
pub type SSQCNTS_SEQX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSQCNTS_SEQX` writer - X scan position"]
pub type SSQCNTS_SEQX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SSQCNTS_SPEC, u16, u16, 15, O>;
#[doc = "Field `SSQCNTS_OUT` reader - This bit holds the value (0,1) to be output when the X/Y scan position is reached."]
pub type SSQCNTS_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SSQCNTS_OUT` writer - This bit holds the value (0,1) to be output when the X/Y scan position is reached."]
pub type SSQCNTS_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSQCNTS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    pub fn ssqcnts_seqy(&self) -> SSQCNTS_SEQY_R {
        SSQCNTS_SEQY_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    pub fn ssqcnts_field(&self) -> SSQCNTS_FIELD_R {
        SSQCNTS_FIELD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    pub fn ssqcnts_seqx(&self) -> SSQCNTS_SEQX_R {
        SSQCNTS_SEQX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - This bit holds the value (0,1) to be output when the X/Y scan position is reached."]
    #[inline(always)]
    pub fn ssqcnts_out(&self) -> SSQCNTS_OUT_R {
        SSQCNTS_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    #[must_use]
    pub fn ssqcnts_seqy(&mut self) -> SSQCNTS_SEQY_W<0> {
        SSQCNTS_SEQY_W::new(self)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    #[must_use]
    pub fn ssqcnts_field(&mut self) -> SSQCNTS_FIELD_W<15> {
        SSQCNTS_FIELD_W::new(self)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    #[must_use]
    pub fn ssqcnts_seqx(&mut self) -> SSQCNTS_SEQX_W<16> {
        SSQCNTS_SEQX_W::new(self)
    }
    #[doc = "Bit 31 - This bit holds the value (0,1) to be output when the X/Y scan position is reached."]
    #[inline(always)]
    #[must_use]
    pub fn ssqcnts_out(&mut self) -> SSQCNTS_OUT_W<31> {
        SSQCNTS_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The 64 Sequencer Position Definitions registers define the X/Y scan positions of the sequencers, hold their output value and assign the sequencer to an odd/even field\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssqcnts](index.html) module"]
pub struct SSQCNTS_SPEC;
impl crate::RegisterSpec for SSQCNTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssqcnts::R](R) reader structure"]
impl crate::Readable for SSQCNTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssqcnts::W](W) writer structure"]
impl crate::Writable for SSQCNTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSQCNTS[%s]
to value 0"]
impl crate::Resettable for SSQCNTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
