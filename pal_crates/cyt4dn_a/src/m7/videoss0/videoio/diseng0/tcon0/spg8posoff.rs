#[doc = "Register `SPG8POSOFF` reader"]
pub struct R(crate::R<SPG8POSOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG8POSOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG8POSOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG8POSOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG8POSOFF` writer"]
pub struct W(crate::W<SPG8POSOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG8POSOFF_SPEC>;
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
impl From<crate::W<SPG8POSOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG8POSOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGPSOFF_Y8` reader - Y scan position"]
pub type SPGPSOFF_Y8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSOFF_Y8` writer - Y scan position"]
pub type SPGPSOFF_Y8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG8POSOFF_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSOFF_FIELD8` reader - Field 0b=odd field, 1b=even field"]
pub type SPGPSOFF_FIELD8_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSOFF_FIELD8` writer - Field 0b=odd field, 1b=even field"]
pub type SPGPSOFF_FIELD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG8POSOFF_SPEC, bool, O>;
#[doc = "Field `SPGPSOFF_X8` reader - X scan position"]
pub type SPGPSOFF_X8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSOFF_X8` writer - X scan position"]
pub type SPGPSOFF_X8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG8POSOFF_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSOFF_TOGGLE8` reader - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSOFF_TOGGLE8_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSOFF_TOGGLE8` writer - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSOFF_TOGGLE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG8POSOFF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    pub fn spgpsoff_y8(&self) -> SPGPSOFF_Y8_R {
        SPGPSOFF_Y8_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    pub fn spgpsoff_field8(&self) -> SPGPSOFF_FIELD8_R {
        SPGPSOFF_FIELD8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    pub fn spgpsoff_x8(&self) -> SPGPSOFF_X8_R {
        SPGPSOFF_X8_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    pub fn spgpsoff_toggle8(&self) -> SPGPSOFF_TOGGLE8_R {
        SPGPSOFF_TOGGLE8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_y8(&mut self) -> SPGPSOFF_Y8_W<0> {
        SPGPSOFF_Y8_W::new(self)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_field8(&mut self) -> SPGPSOFF_FIELD8_W<15> {
        SPGPSOFF_FIELD8_W::new(self)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_x8(&mut self) -> SPGPSOFF_X8_W<16> {
        SPGPSOFF_X8_W::new(self)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_toggle8(&mut self) -> SPGPSOFF_TOGGLE8_W<31> {
        SPGPSOFF_TOGGLE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync pulse generator 8, 'Switch off' position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg8posoff](index.html) module"]
pub struct SPG8POSOFF_SPEC;
impl crate::RegisterSpec for SPG8POSOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg8posoff::R](R) reader structure"]
impl crate::Readable for SPG8POSOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg8posoff::W](W) writer structure"]
impl crate::Writable for SPG8POSOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG8POSOFF to value 0"]
impl crate::Resettable for SPG8POSOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
