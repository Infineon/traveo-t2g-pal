#[doc = "Register `TTL0_MUX3` reader"]
pub struct R(crate::R<TTL0_MUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX3` writer"]
pub struct W(crate::W<TTL0_MUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX3_SPEC>;
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
impl From<crate::W<TTL0_MUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_COL12` reader - Pin ID for color bit 12."]
pub type TTL0_COL12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL12` writer - Pin ID for color bit 12."]
pub type TTL0_COL12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL13` reader - Pin ID for color bit 13."]
pub type TTL0_COL13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL13` writer - Pin ID for color bit 13."]
pub type TTL0_COL13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL14` reader - Pin ID for color bit 14."]
pub type TTL0_COL14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL14` writer - Pin ID for color bit 14."]
pub type TTL0_COL14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_COL15` reader - Pin ID for color bit 15."]
pub type TTL0_COL15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_COL15` writer - Pin ID for color bit 15."]
pub type TTL0_COL15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for color bit 12."]
    #[inline(always)]
    pub fn ttl0_col12(&self) -> TTL0_COL12_R {
        TTL0_COL12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 13."]
    #[inline(always)]
    pub fn ttl0_col13(&self) -> TTL0_COL13_R {
        TTL0_COL13_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 14."]
    #[inline(always)]
    pub fn ttl0_col14(&self) -> TTL0_COL14_R {
        TTL0_COL14_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 15."]
    #[inline(always)]
    pub fn ttl0_col15(&self) -> TTL0_COL15_R {
        TTL0_COL15_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for color bit 12."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col12(&mut self) -> TTL0_COL12_W<0> {
        TTL0_COL12_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for color bit 13."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col13(&mut self) -> TTL0_COL13_W<8> {
        TTL0_COL13_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for color bit 14."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col14(&mut self) -> TTL0_COL14_W<16> {
        TTL0_COL14_W::new(self)
    }
    #[doc = "Bits 24:28 - Pin ID for color bit 15."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_col15(&mut self) -> TTL0_COL15_W<24> {
        TTL0_COL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux3](index.html) module"]
pub struct TTL0_MUX3_SPEC;
impl crate::RegisterSpec for TTL0_MUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux3::R](R) reader structure"]
impl crate::Readable for TTL0_MUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux3::W](W) writer structure"]
impl crate::Writable for TTL0_MUX3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX3 to value 0x0f0e_0d0c"]
impl crate::Resettable for TTL0_MUX3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0e_0d0c;
}
