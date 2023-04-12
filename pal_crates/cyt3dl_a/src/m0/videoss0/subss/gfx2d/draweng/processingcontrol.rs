#[doc = "Register `PROCESSINGCONTROL` reader"]
pub struct R(crate::R<PROCESSINGCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCESSINGCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCESSINGCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCESSINGCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROCESSINGCONTROL` writer"]
pub struct W(crate::W<PROCESSINGCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROCESSINGCONTROL_SPEC>;
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
impl From<crate::W<PROCESSINGCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROCESSINGCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBPIXELGRID` reader - Higher SubPixelGrid (super sampling) results in better anti-aliased output at the cost of processing time."]
pub type SUBPIXELGRID_R = crate::FieldReader<u8, SUBPIXELGRID_A>;
#[doc = "Higher SubPixelGrid (super sampling) results in better anti-aliased output at the cost of processing time.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUBPIXELGRID_A {
    #[doc = "1: 1 bit per pixel, anti-aliased"]
    TOTALBITS_1 = 1,
    #[doc = "2: 2 bit per pixel, results in 2 bit anti-aliased intermediate value"]
    TOTALBITS_2 = 2,
    #[doc = "4: 4 bit per pixel, results in 4 bit anti-aliased intermediate value"]
    TOTALBITS_4 = 4,
    #[doc = "8: 8 bit per pixel, results in 6 bit anti-aliased intermediate value"]
    TOTALBITS_8 = 8,
}
impl From<SUBPIXELGRID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBPIXELGRID_A) -> Self {
        variant as _
    }
}
impl SUBPIXELGRID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBPIXELGRID_A> {
        match self.bits {
            1 => Some(SUBPIXELGRID_A::TOTALBITS_1),
            2 => Some(SUBPIXELGRID_A::TOTALBITS_2),
            4 => Some(SUBPIXELGRID_A::TOTALBITS_4),
            8 => Some(SUBPIXELGRID_A::TOTALBITS_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_1`"]
    #[inline(always)]
    pub fn is_totalbits_1(&self) -> bool {
        *self == SUBPIXELGRID_A::TOTALBITS_1
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_2`"]
    #[inline(always)]
    pub fn is_totalbits_2(&self) -> bool {
        *self == SUBPIXELGRID_A::TOTALBITS_2
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_4`"]
    #[inline(always)]
    pub fn is_totalbits_4(&self) -> bool {
        *self == SUBPIXELGRID_A::TOTALBITS_4
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_8`"]
    #[inline(always)]
    pub fn is_totalbits_8(&self) -> bool {
        *self == SUBPIXELGRID_A::TOTALBITS_8
    }
}
#[doc = "Field `SUBPIXELGRID` writer - Higher SubPixelGrid (super sampling) results in better anti-aliased output at the cost of processing time."]
pub type SUBPIXELGRID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROCESSINGCONTROL_SPEC, u8, SUBPIXELGRID_A, 4, O>;
impl<'a, const O: u8> SUBPIXELGRID_W<'a, O> {
    #[doc = "1 bit per pixel, anti-aliased"]
    #[inline(always)]
    pub fn totalbits_1(self) -> &'a mut W {
        self.variant(SUBPIXELGRID_A::TOTALBITS_1)
    }
    #[doc = "2 bit per pixel, results in 2 bit anti-aliased intermediate value"]
    #[inline(always)]
    pub fn totalbits_2(self) -> &'a mut W {
        self.variant(SUBPIXELGRID_A::TOTALBITS_2)
    }
    #[doc = "4 bit per pixel, results in 4 bit anti-aliased intermediate value"]
    #[inline(always)]
    pub fn totalbits_4(self) -> &'a mut W {
        self.variant(SUBPIXELGRID_A::TOTALBITS_4)
    }
    #[doc = "8 bit per pixel, results in 6 bit anti-aliased intermediate value"]
    #[inline(always)]
    pub fn totalbits_8(self) -> &'a mut W {
        self.variant(SUBPIXELGRID_A::TOTALBITS_8)
    }
}
#[doc = "Field `FILLRULE` reader - Set the fill rule."]
pub type FILLRULE_R = crate::BitReader<FILLRULE_A>;
#[doc = "Set the fill rule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILLRULE_A {
    #[doc = "1: a edge count unequal zero is inside"]
    NONZERO = 1,
    #[doc = "0: an odd edge count is inside, an even edge count is outside"]
    EVENODD = 0,
}
impl From<FILLRULE_A> for bool {
    #[inline(always)]
    fn from(variant: FILLRULE_A) -> Self {
        variant as u8 != 0
    }
}
impl FILLRULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILLRULE_A {
        match self.bits {
            true => FILLRULE_A::NONZERO,
            false => FILLRULE_A::EVENODD,
        }
    }
    #[doc = "Checks if the value of the field is `NONZERO`"]
    #[inline(always)]
    pub fn is_nonzero(&self) -> bool {
        *self == FILLRULE_A::NONZERO
    }
    #[doc = "Checks if the value of the field is `EVENODD`"]
    #[inline(always)]
    pub fn is_evenodd(&self) -> bool {
        *self == FILLRULE_A::EVENODD
    }
}
#[doc = "Field `FILLRULE` writer - Set the fill rule."]
pub type FILLRULE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROCESSINGCONTROL_SPEC, FILLRULE_A, O>;
impl<'a, const O: u8> FILLRULE_W<'a, O> {
    #[doc = "a edge count unequal zero is inside"]
    #[inline(always)]
    pub fn nonzero(self) -> &'a mut W {
        self.variant(FILLRULE_A::NONZERO)
    }
    #[doc = "an odd edge count is inside, an even edge count is outside"]
    #[inline(always)]
    pub fn evenodd(self) -> &'a mut W {
        self.variant(FILLRULE_A::EVENODD)
    }
}
#[doc = "Field `TARGETBITSPERPIXEL` reader - Number of bits per pixel in the target buffer for the following blit operation. Must be 1, 2, 4, 8, 16, 18, 24 or 32. This field allows to obtain an aligned bounding box width."]
pub type TARGETBITSPERPIXEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TARGETBITSPERPIXEL` writer - Number of bits per pixel in the target buffer for the following blit operation. Must be 1, 2, 4, 8, 16, 18, 24 or 32. This field allows to obtain an aligned bounding box width."]
pub type TARGETBITSPERPIXEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROCESSINGCONTROL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:3 - Higher SubPixelGrid (super sampling) results in better anti-aliased output at the cost of processing time."]
    #[inline(always)]
    pub fn subpixelgrid(&self) -> SUBPIXELGRID_R {
        SUBPIXELGRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Set the fill rule."]
    #[inline(always)]
    pub fn fillrule(&self) -> FILLRULE_R {
        FILLRULE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of bits per pixel in the target buffer for the following blit operation. Must be 1, 2, 4, 8, 16, 18, 24 or 32. This field allows to obtain an aligned bounding box width."]
    #[inline(always)]
    pub fn targetbitsperpixel(&self) -> TARGETBITSPERPIXEL_R {
        TARGETBITSPERPIXEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Higher SubPixelGrid (super sampling) results in better anti-aliased output at the cost of processing time."]
    #[inline(always)]
    #[must_use]
    pub fn subpixelgrid(&mut self) -> SUBPIXELGRID_W<0> {
        SUBPIXELGRID_W::new(self)
    }
    #[doc = "Bit 4 - Set the fill rule."]
    #[inline(always)]
    #[must_use]
    pub fn fillrule(&mut self) -> FILLRULE_W<4> {
        FILLRULE_W::new(self)
    }
    #[doc = "Bits 16:21 - Number of bits per pixel in the target buffer for the following blit operation. Must be 1, 2, 4, 8, 16, 18, 24 or 32. This field allows to obtain an aligned bounding box width."]
    #[inline(always)]
    #[must_use]
    pub fn targetbitsperpixel(&mut self) -> TARGETBITSPERPIXEL_W<16> {
        TARGETBITSPERPIXEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Drawing Engine main processing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [processingcontrol](index.html) module"]
pub struct PROCESSINGCONTROL_SPEC;
impl crate::RegisterSpec for PROCESSINGCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [processingcontrol::R](R) reader structure"]
impl crate::Readable for PROCESSINGCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [processingcontrol::W](W) writer structure"]
impl crate::Writable for PROCESSINGCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROCESSINGCONTROL to value 0x0020_0008"]
impl crate::Resettable for PROCESSINGCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0008;
}
