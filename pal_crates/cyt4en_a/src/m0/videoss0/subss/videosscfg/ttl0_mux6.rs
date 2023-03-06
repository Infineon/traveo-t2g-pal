#[doc = "Register `TTL0_MUX6` reader"]
pub struct R(crate::R<TTL0_MUX6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTL0_MUX6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTL0_MUX6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTL0_MUX6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTL0_MUX6` writer"]
pub struct W(crate::W<TTL0_MUX6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTL0_MUX6_SPEC>;
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
impl From<crate::W<TTL0_MUX6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTL0_MUX6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTL0_HSYNC` reader - Pin ID for horizontal sync signal."]
pub type TTL0_HSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_HSYNC` writer - Pin ID for horizontal sync signal."]
pub type TTL0_HSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX6_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_VSYNC` reader - Pin ID for vertical sync signal."]
pub type TTL0_VSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_VSYNC` writer - Pin ID for vertical sync signal."]
pub type TTL0_VSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX6_SPEC, u8, u8, 5, O>;
#[doc = "Field `TTL0_EN` reader - Pin ID for enable signal (active pixels)."]
pub type TTL0_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTL0_EN` writer - Pin ID for enable signal (active pixels)."]
pub type TTL0_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTL0_MUX6_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pin ID for horizontal sync signal."]
    #[inline(always)]
    pub fn ttl0_hsync(&self) -> TTL0_HSYNC_R {
        TTL0_HSYNC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Pin ID for vertical sync signal."]
    #[inline(always)]
    pub fn ttl0_vsync(&self) -> TTL0_VSYNC_R {
        TTL0_VSYNC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Pin ID for enable signal (active pixels)."]
    #[inline(always)]
    pub fn ttl0_en(&self) -> TTL0_EN_R {
        TTL0_EN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin ID for horizontal sync signal."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_hsync(&mut self) -> TTL0_HSYNC_W<0> {
        TTL0_HSYNC_W::new(self)
    }
    #[doc = "Bits 8:12 - Pin ID for vertical sync signal."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_vsync(&mut self) -> TTL0_VSYNC_W<8> {
        TTL0_VSYNC_W::new(self)
    }
    #[doc = "Bits 16:20 - Pin ID for enable signal (active pixels)."]
    #[inline(always)]
    #[must_use]
    pub fn ttl0_en(&mut self) -> TTL0_EN_W<16> {
        TTL0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin multiplexer for capture input from TTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttl0_mux6](index.html) module"]
pub struct TTL0_MUX6_SPEC;
impl crate::RegisterSpec for TTL0_MUX6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttl0_mux6::R](R) reader structure"]
impl crate::Readable for TTL0_MUX6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttl0_mux6::W](W) writer structure"]
impl crate::Writable for TTL0_MUX6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTL0_MUX6 to value 0x001a_1918"]
impl crate::Resettable for TTL0_MUX6_SPEC {
    const RESET_VALUE: Self::Ux = 0x001a_1918;
}
