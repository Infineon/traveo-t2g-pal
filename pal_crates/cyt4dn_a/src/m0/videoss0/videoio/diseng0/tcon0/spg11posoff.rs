#[doc = "Register `SPG11POSOFF` reader"]
pub struct R(crate::R<SPG11POSOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG11POSOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG11POSOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG11POSOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG11POSOFF` writer"]
pub struct W(crate::W<SPG11POSOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG11POSOFF_SPEC>;
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
impl From<crate::W<SPG11POSOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG11POSOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGPSOFF_Y11` reader - Y scan position"]
pub type SPGPSOFF_Y11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSOFF_Y11` writer - Y scan position"]
pub type SPGPSOFF_Y11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG11POSOFF_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSOFF_FIELD11` reader - Field 0b=odd field, 1b=even field"]
pub type SPGPSOFF_FIELD11_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSOFF_FIELD11` writer - Field 0b=odd field, 1b=even field"]
pub type SPGPSOFF_FIELD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG11POSOFF_SPEC, bool, O>;
#[doc = "Field `SPGPSOFF_X11` reader - X scan position"]
pub type SPGPSOFF_X11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSOFF_X11` writer - X scan position"]
pub type SPGPSOFF_X11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG11POSOFF_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSOFF_TOGGLE11` reader - toggle enable 0b=disable, 1b=enable"]
pub type SPGPSOFF_TOGGLE11_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSOFF_TOGGLE11` writer - toggle enable 0b=disable, 1b=enable"]
pub type SPGPSOFF_TOGGLE11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPG11POSOFF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    pub fn spgpsoff_y11(&self) -> SPGPSOFF_Y11_R {
        SPGPSOFF_Y11_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    pub fn spgpsoff_field11(&self) -> SPGPSOFF_FIELD11_R {
        SPGPSOFF_FIELD11_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    pub fn spgpsoff_x11(&self) -> SPGPSOFF_X11_R {
        SPGPSOFF_X11_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    pub fn spgpsoff_toggle11(&self) -> SPGPSOFF_TOGGLE11_R {
        SPGPSOFF_TOGGLE11_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_y11(&mut self) -> SPGPSOFF_Y11_W<0> {
        SPGPSOFF_Y11_W::new(self)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_field11(&mut self) -> SPGPSOFF_FIELD11_W<15> {
        SPGPSOFF_FIELD11_W::new(self)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_x11(&mut self) -> SPGPSOFF_X11_W<16> {
        SPGPSOFF_X11_W::new(self)
    }
    #[doc = "Bit 31 - toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    #[must_use]
    pub fn spgpsoff_toggle11(&mut self) -> SPGPSOFF_TOGGLE11_W<31> {
        SPGPSOFF_TOGGLE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync pulse generator 11, 'Switch off' position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg11posoff](index.html) module"]
pub struct SPG11POSOFF_SPEC;
impl crate::RegisterSpec for SPG11POSOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg11posoff::R](R) reader structure"]
impl crate::Readable for SPG11POSOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg11posoff::W](W) writer structure"]
impl crate::Writable for SPG11POSOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG11POSOFF to value 0"]
impl crate::Resettable for SPG11POSOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
