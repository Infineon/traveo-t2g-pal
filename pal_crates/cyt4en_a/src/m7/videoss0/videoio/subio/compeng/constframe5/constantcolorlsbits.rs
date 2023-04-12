#[doc = "Register `CONSTANTCOLORLSBITS` reader"]
pub struct R(crate::R<CONSTANTCOLORLSBITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANTCOLORLSBITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANTCOLORLSBITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANTCOLORLSBITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANTCOLORLSBITS` writer"]
pub struct W(crate::W<CONSTANTCOLORLSBITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANTCOLORLSBITS_SPEC>;
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
impl From<crate::W<CONSTANTCOLORLSBITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANTCOLORLSBITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLELSBITS` reader - If enabled, the two least significant bits of 10-bit color components are taken from this register. Otherwise the two most significant bits from 8-bit values in ConstantColor register are taken."]
pub type ENABLELSBITS_R = crate::BitReader<bool>;
#[doc = "Field `ENABLELSBITS` writer - If enabled, the two least significant bits of 10-bit color components are taken from this register. Otherwise the two most significant bits from 8-bit values in ConstantColor register are taken."]
pub type ENABLELSBITS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONSTANTCOLORLSBITS_SPEC, bool, O>;
#[doc = "Field `CONSTANTBLUELSBITS` reader - LSBits of 10-bit blue component."]
pub type CONSTANTBLUELSBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTBLUELSBITS` writer - LSBits of 10-bit blue component."]
pub type CONSTANTBLUELSBITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLORLSBITS_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONSTANTGREENLSBITS` reader - LSBits of 10-bit green component."]
pub type CONSTANTGREENLSBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTGREENLSBITS` writer - LSBits of 10-bit green component."]
pub type CONSTANTGREENLSBITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLORLSBITS_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONSTANTREDLSBITS` reader - LSBits of 10-bit red component."]
pub type CONSTANTREDLSBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANTREDLSBITS` writer - LSBits of 10-bit red component."]
pub type CONSTANTREDLSBITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONSTANTCOLORLSBITS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - If enabled, the two least significant bits of 10-bit color components are taken from this register. Otherwise the two most significant bits from 8-bit values in ConstantColor register are taken."]
    #[inline(always)]
    pub fn enablelsbits(&self) -> ENABLELSBITS_R {
        ENABLELSBITS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - LSBits of 10-bit blue component."]
    #[inline(always)]
    pub fn constantbluelsbits(&self) -> CONSTANTBLUELSBITS_R {
        CONSTANTBLUELSBITS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LSBits of 10-bit green component."]
    #[inline(always)]
    pub fn constantgreenlsbits(&self) -> CONSTANTGREENLSBITS_R {
        CONSTANTGREENLSBITS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - LSBits of 10-bit red component."]
    #[inline(always)]
    pub fn constantredlsbits(&self) -> CONSTANTREDLSBITS_R {
        CONSTANTREDLSBITS_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If enabled, the two least significant bits of 10-bit color components are taken from this register. Otherwise the two most significant bits from 8-bit values in ConstantColor register are taken."]
    #[inline(always)]
    #[must_use]
    pub fn enablelsbits(&mut self) -> ENABLELSBITS_W<0> {
        ENABLELSBITS_W::new(self)
    }
    #[doc = "Bits 8:9 - LSBits of 10-bit blue component."]
    #[inline(always)]
    #[must_use]
    pub fn constantbluelsbits(&mut self) -> CONSTANTBLUELSBITS_W<8> {
        CONSTANTBLUELSBITS_W::new(self)
    }
    #[doc = "Bits 16:17 - LSBits of 10-bit green component."]
    #[inline(always)]
    #[must_use]
    pub fn constantgreenlsbits(&mut self) -> CONSTANTGREENLSBITS_W<16> {
        CONSTANTGREENLSBITS_W::new(self)
    }
    #[doc = "Bits 24:25 - LSBits of 10-bit red component."]
    #[inline(always)]
    #[must_use]
    pub fn constantredlsbits(&mut self) -> CONSTANTREDLSBITS_W<24> {
        CONSTANTREDLSBITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LSBits for 10-bit constant frame color components (optional).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constantcolorlsbits](index.html) module"]
pub struct CONSTANTCOLORLSBITS_SPEC;
impl crate::RegisterSpec for CONSTANTCOLORLSBITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constantcolorlsbits::R](R) reader structure"]
impl crate::Readable for CONSTANTCOLORLSBITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constantcolorlsbits::W](W) writer structure"]
impl crate::Writable for CONSTANTCOLORLSBITS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANTCOLORLSBITS to value 0"]
impl crate::Resettable for CONSTANTCOLORLSBITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
