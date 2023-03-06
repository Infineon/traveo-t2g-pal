#[doc = "Register `TTL0_MUX4` reader"]
pub struct R(crate::R<TTL0_MUX4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX4` writer"]
pub struct W(crate::W<TTL0_MUX4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX4_SPEC>;
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
impl From<crate::W<TTL0_MUX4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_COL16` reader - Pin ID for color bit 16."]
pub type TTL0_COL16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL16` writer - Pin ID for color bit 16."]
pub type TTL0_COL16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX4_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL17` reader - Pin ID for color bit 17."]
pub type TTL0_COL17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL17` writer - Pin ID for color bit 17."]
pub type TTL0_COL17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX4_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL18` reader - Pin ID for color bit 18."]
pub type TTL0_COL18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL18` writer - Pin ID for color bit 18."]
pub type TTL0_COL18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX4_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL19` reader - Pin ID for color bit 19."]
pub type TTL0_COL19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL19` writer - Pin ID for color bit 19."]
pub type TTL0_COL19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for color bit 16."]
    #[inline(always)]
    pub fn ttl0_col16(&self) -> TTL0_COL16_R {
        TTL0_COL16_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 17."]
    #[inline(always)]
    pub fn ttl0_col17(&self) -> TTL0_COL17_R {
        TTL0_COL17_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 18."]
    #[inline(always)]
    pub fn ttl0_col18(&self) -> TTL0_COL18_R {
        TTL0_COL18_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 19."]
    #[inline(always)]
    pub fn ttl0_col19(&self) -> TTL0_COL19_R {
        TTL0_COL19_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for color bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col16(&mut self) -> TTL0_COL16_W<0> {
        TTL0_COL16_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 17."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col17(&mut self) -> TTL0_COL17_W<8> {
        TTL0_COL17_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 18."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col18(&mut self) -> TTL0_COL18_W<16> {
        TTL0_COL18_W::new(self)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 19."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col19(&mut self) -> TTL0_COL19_W<24> {
        TTL0_COL19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux4](index.html) module"]
pub struct TTL0_MUX4_SPEC;
impl crate::RegisterSpec for TTL0_MUX4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux4::R](R) reader structure"]
impl crate::Readable for TTL0_MUX4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux4::W](W) writer structure"]
impl crate::Writable for TTL0_MUX4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX4 to value 0x1312_1110"]
impl crate::Resettable for TTL0_MUX4_SPEC {
    const RESET_VALUE: Self::Ux = 0x1312_1110;
}
