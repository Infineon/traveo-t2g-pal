#[doc = "Register `SPG1POSON` reader"]
pub struct R(crate::R<SPG1POSON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPG1POSON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPG1POSON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPG1POSON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPG1POSON` writer"]
pub struct W(crate::W<SPG1POSON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPG1POSON_SPEC>;
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
impl From<crate::W<SPG1POSON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPG1POSON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPGPSON_Y1` reader - Y scan position"]
pub type SPGPSON_Y1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSON_Y1` writer - Y scan position"]
pub type SPGPSON_Y1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG1POSON_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSON_FIELD1` reader - Field 0b=odd field, 1b=even field"]
pub type SPGPSON_FIELD1_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSON_FIELD1` writer - Field 0b=odd field, 1b=even field"]
pub type SPGPSON_FIELD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG1POSON_SPEC, bool, O>;
#[doc = "Field `SPGPSON_X1` reader - X scan position"]
pub type SPGPSON_X1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPGPSON_X1` writer - X scan position"]
pub type SPGPSON_X1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPG1POSON_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPGPSON_TOGGLE1` reader - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSON_TOGGLE1_R = crate::BitReader<bool>;
#[doc = "Field `SPGPSON_TOGGLE1` writer - Toggle enable 0b=disable, 1b=enable"]
pub type SPGPSON_TOGGLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPG1POSON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    pub fn spgpson_y1(&self) -> SPGPSON_Y1_R {
        SPGPSON_Y1_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    pub fn spgpson_field1(&self) -> SPGPSON_FIELD1_R {
        SPGPSON_FIELD1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    pub fn spgpson_x1(&self) -> SPGPSON_X1_R {
        SPGPSON_X1_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    pub fn spgpson_toggle1(&self) -> SPGPSON_TOGGLE1_R {
        SPGPSON_TOGGLE1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Y scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_y1(&mut self) -> SPGPSON_Y1_W<0> {
        SPGPSON_Y1_W::new(self)
    }
    #[doc = "Bit 15 - Field 0b=odd field, 1b=even field"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_field1(&mut self) -> SPGPSON_FIELD1_W<15> {
        SPGPSON_FIELD1_W::new(self)
    }
    #[doc = "Bits 16:30 - X scan position"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_x1(&mut self) -> SPGPSON_X1_W<16> {
        SPGPSON_X1_W::new(self)
    }
    #[doc = "Bit 31 - Toggle enable 0b=disable, 1b=enable"]
    #[inline(always)]
    #[must_use]
    pub fn spgpson_toggle1(&mut self) -> SPGPSON_TOGGLE1_W<31> {
        SPGPSON_TOGGLE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync pulse generator 1, 'Switch on' position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spg1poson](index.html) module"]
pub struct SPG1POSON_SPEC;
impl crate::RegisterSpec for SPG1POSON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spg1poson::R](R) reader structure"]
impl crate::Readable for SPG1POSON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spg1poson::W](W) writer structure"]
impl crate::Writable for SPG1POSON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPG1POSON to value 0xf3"]
impl crate::Resettable for SPG1POSON_SPEC {
    const RESET_VALUE: Self::Ux = 0xf3;
}
