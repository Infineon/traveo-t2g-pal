#[doc = "Register `TTL0_MUX0` reader"]
pub struct R(crate::R<TTL0_MUX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX0` writer"]
pub struct W(crate::W<TTL0_MUX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX0_SPEC>;
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
impl From<crate::W<TTL0_MUX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_COL0` reader - Pin ID for color bit 0 or ITU656 data bit 0 (10-bit mode only)."]
pub type TTL0_COL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL0` writer - Pin ID for color bit 0 or ITU656 data bit 0 (10-bit mode only)."]
pub type TTL0_COL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX0_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL1` reader - Pin ID for color bit 1 or ITU656 data bit 1 (10-bit mode only)."]
pub type TTL0_COL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL1` writer - Pin ID for color bit 1 or ITU656 data bit 1 (10-bit mode only)."]
pub type TTL0_COL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX0_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL2` reader - Pin ID for color bit 2 or ITU656 data bit 2."]
pub type TTL0_COL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL2` writer - Pin ID for color bit 2 or ITU656 data bit 2."]
pub type TTL0_COL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX0_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL3` reader - Pin ID for color bit 3 or ITU656 data bit 3."]
pub type TTL0_COL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL3` writer - Pin ID for color bit 3 or ITU656 data bit 3."]
pub type TTL0_COL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for color bit 0 or ITU656 data bit 0 (10-bit mode only)."]
    #[inline(always)]
    pub fn ttl0_col0(&self) -> TTL0_COL0_R {
        TTL0_COL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 1 or ITU656 data bit 1 (10-bit mode only)."]
    #[inline(always)]
    pub fn ttl0_col1(&self) -> TTL0_COL1_R {
        TTL0_COL1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 2 or ITU656 data bit 2."]
    #[inline(always)]
    pub fn ttl0_col2(&self) -> TTL0_COL2_R {
        TTL0_COL2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 3 or ITU656 data bit 3."]
    #[inline(always)]
    pub fn ttl0_col3(&self) -> TTL0_COL3_R {
        TTL0_COL3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for color bit 0 or ITU656 data bit 0 (10-bit mode only)."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col0(&mut self) -> TTL0_COL0_W<0> {
        TTL0_COL0_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 1 or ITU656 data bit 1 (10-bit mode only)."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col1(&mut self) -> TTL0_COL1_W<8> {
        TTL0_COL1_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 2 or ITU656 data bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col2(&mut self) -> TTL0_COL2_W<16> {
        TTL0_COL2_W::new(self)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 3 or ITU656 data bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col3(&mut self) -> TTL0_COL3_W<24> {
        TTL0_COL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux0](index.html) module"]
pub struct TTL0_MUX0_SPEC;
impl crate::RegisterSpec for TTL0_MUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux0::R](R) reader structure"]
impl crate::Readable for TTL0_MUX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux0::W](W) writer structure"]
impl crate::Writable for TTL0_MUX0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX0 to value 0x0302_0100"]
impl crate::Resettable for TTL0_MUX0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0100;
}
