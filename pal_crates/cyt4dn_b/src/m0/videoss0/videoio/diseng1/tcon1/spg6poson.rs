#[doc = "Register `SPG6POSON` reader"]
pub struct R(crate::R<SPG6POSON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG6POSON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG6POSON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG6POSON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG6POSON` writer"]
pub struct W(crate::W<SPG6POSON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG6POSON_SPEC>;
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
impl From<crate::W<SPG6POSON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG6POSON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGPSON_Y6` reader - Y scan position"]
pub type SPGPSON_Y6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSON_Y6` writer - Y scan position"]
pub type SPGPSON_Y6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG6POSON_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSON_FIELD6` reader - Field 0b=odd field, 1b=even field"]
pub type SPGPSON_FIELD6_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSON_FIELD6` writer - Field 0b=odd field, 1b=even field"]
pub type SPGPSON_FIELD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG6POSON_SPEC, bool, O>;
#[doc = "Field `SPGPSON_X6` reader - X scan position"]
pub type SPGPSON_X6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSON_X6` writer - X scan position"]
pub type SPGPSON_X6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG6POSON_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSON_TOGGLE6` reader - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSON_TOGGLE6_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSON_TOGGLE6` writer - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSON_TOGGLE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG6POSON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    pub fn spgpson_y6(&self) -> SPGPSON_Y6_R {
        SPGPSON_Y6_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    pub fn spgpson_field6(&self) -> SPGPSON_FIELD6_R {
        SPGPSON_FIELD6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    pub fn spgpson_x6(&self) -> SPGPSON_X6_R {
        SPGPSON_X6_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    pub fn spgpson_toggle6(&self) -> SPGPSON_TOGGLE6_R {
        SPGPSON_TOGGLE6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_y6(&mut self) -> SPGPSON_Y6_W<0> {
        SPGPSON_Y6_W::new(self)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_field6(&mut self) -> SPGPSON_FIELD6_W<15> {
        SPGPSON_FIELD6_W::new(self)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_x6(&mut self) -> SPGPSON_X6_W<16> {
        SPGPSON_X6_W::new(self)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_toggle6(&mut self) -> SPGPSON_TOGGLE6_W<31> {
        SPGPSON_TOGGLE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync pulse generator 6, 'Switch on' position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg6poson](index.html) module"]
pub struct SPG6POSON_SPEC;
impl crate::RegisterSpec for SPG6POSON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg6poson::R](R) reader structure"]
impl crate::Readable for SPG6POSON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg6poson::W](W) writer structure"]
impl crate::Writable for SPG6POSON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG6POSON to value 0"]
impl crate::Resettable for SPG6POSON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
