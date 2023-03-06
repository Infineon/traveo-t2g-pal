#[doc = "Register `TTL0_MUX1` reader"]
pub struct R(crate::R<TTL0_MUX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX1` writer"]
pub struct W(crate::W<TTL0_MUX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX1_SPEC>;
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
impl From<crate::W<TTL0_MUX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_COL4` reader - Pin ID for color bit 4 or ITU656 data bit 4."]
pub type TTL0_COL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL4` writer - Pin ID for color bit 4 or ITU656 data bit 4."]
pub type TTL0_COL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL5` reader - Pin ID for color bit 5 or ITU656 data bit 5."]
pub type TTL0_COL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL5` writer - Pin ID for color bit 5 or ITU656 data bit 5."]
pub type TTL0_COL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL6` reader - Pin ID for color bit 6 or ITU656 data bit 6."]
pub type TTL0_COL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL6` writer - Pin ID for color bit 6 or ITU656 data bit 6."]
pub type TTL0_COL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL7` reader - Pin ID for color bit 7 or ITU656 data bit 7."]
pub type TTL0_COL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL7` writer - Pin ID for color bit 7 or ITU656 data bit 7."]
pub type TTL0_COL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for color bit 4 or ITU656 data bit 4."]
    #[inline(always)]
    pub fn ttl0_col4(&self) -> TTL0_COL4_R {
        TTL0_COL4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 5 or ITU656 data bit 5."]
    #[inline(always)]
    pub fn ttl0_col5(&self) -> TTL0_COL5_R {
        TTL0_COL5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 6 or ITU656 data bit 6."]
    #[inline(always)]
    pub fn ttl0_col6(&self) -> TTL0_COL6_R {
        TTL0_COL6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 7 or ITU656 data bit 7."]
    #[inline(always)]
    pub fn ttl0_col7(&self) -> TTL0_COL7_R {
        TTL0_COL7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for color bit 4 or ITU656 data bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col4(&mut self) -> TTL0_COL4_W<0> {
        TTL0_COL4_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 5 or ITU656 data bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col5(&mut self) -> TTL0_COL5_W<8> {
        TTL0_COL5_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 6 or ITU656 data bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col6(&mut self) -> TTL0_COL6_W<16> {
        TTL0_COL6_W::new(self)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 7 or ITU656 data bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col7(&mut self) -> TTL0_COL7_W<24> {
        TTL0_COL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux1](index.html) module"]
pub struct TTL0_MUX1_SPEC;
impl crate::RegisterSpec for TTL0_MUX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux1::R](R) reader structure"]
impl crate::Readable for TTL0_MUX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux1::W](W) writer structure"]
impl crate::Writable for TTL0_MUX1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX1 to value 0x0706_0504"]
impl crate::Resettable for TTL0_MUX1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0706_0504;
}
