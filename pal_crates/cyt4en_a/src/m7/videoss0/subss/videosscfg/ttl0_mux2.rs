#[doc = "Register `TTL0_MUX2` reader"]
pub struct R(crate::R<TTL0_MUX2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX2` writer"]
pub struct W(crate::W<TTL0_MUX2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX2_SPEC>;
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
impl From<crate::W<TTL0_MUX2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_COL8` reader - Pin ID for color bit 8 or ITU656 data bit 8."]
pub type TTL0_COL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL8` writer - Pin ID for color bit 8 or ITU656 data bit 8."]
pub type TTL0_COL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX2_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL9` reader - Pin ID for color bit 9 or ITU656 data bit 9."]
pub type TTL0_COL9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL9` writer - Pin ID for color bit 9 or ITU656 data bit 9."]
pub type TTL0_COL9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX2_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL10` reader - Pin ID for color bit 10."]
pub type TTL0_COL10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL10` writer - Pin ID for color bit 10."]
pub type TTL0_COL10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX2_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL11` reader - Pin ID for color bit 11."]
pub type TTL0_COL11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL11` writer - Pin ID for color bit 11."]
pub type TTL0_COL11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for color bit 8 or ITU656 data bit 8."]
    #[inline(always)]
    pub fn ttl0_col8(&self) -> TTL0_COL8_R {
        TTL0_COL8_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 9 or ITU656 data bit 9."]
    #[inline(always)]
    pub fn ttl0_col9(&self) -> TTL0_COL9_R {
        TTL0_COL9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 10."]
    #[inline(always)]
    pub fn ttl0_col10(&self) -> TTL0_COL10_R {
        TTL0_COL10_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 11."]
    #[inline(always)]
    pub fn ttl0_col11(&self) -> TTL0_COL11_R {
        TTL0_COL11_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for color bit 8 or ITU656 data bit 8."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col8(&mut self) -> TTL0_COL8_W<0> {
        TTL0_COL8_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 9 or ITU656 data bit 9."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col9(&mut self) -> TTL0_COL9_W<8> {
        TTL0_COL9_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 10."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col10(&mut self) -> TTL0_COL10_W<16> {
        TTL0_COL10_W::new(self)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 11."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col11(&mut self) -> TTL0_COL11_W<24> {
        TTL0_COL11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux2](index.html) module"]
pub struct TTL0_MUX2_SPEC;
impl crate::RegisterSpec for TTL0_MUX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux2::R](R) reader structure"]
impl crate::Readable for TTL0_MUX2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux2::W](W) writer structure"]
impl crate::Writable for TTL0_MUX2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX2 to value 0x0b0a_0908"]
impl crate::Resettable for TTL0_MUX2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b0a_0908;
}
