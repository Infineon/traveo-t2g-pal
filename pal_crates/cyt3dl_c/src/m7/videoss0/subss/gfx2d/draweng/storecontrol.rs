#[doc = "Register `STORECONTROL` reader"]
pub struct R(crate::R<STORECONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORECONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORECONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORECONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORECONTROL` writer"]
pub struct W(crate::W<STORECONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORECONTROL_SPEC>;
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
impl From<crate::W<STORECONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORECONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOREBITSPERPIXEL` reader - The resulting alpha mask can be saved to memory with the following BitsPerPixel settings."]
pub type STOREBITSPERPIXEL_R = crate::FieldReader<u8, STOREBITSPERPIXEL_A>;
#[doc = "The resulting alpha mask can be saved to memory with the following BitsPerPixel settings.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOREBITSPERPIXEL_A {
    #[doc = "1: 1 bit per pixel"]
    TOTALBITS_1 = 1,
    #[doc = "2: 2 bit per pixel"]
    TOTALBITS_2 = 2,
    #[doc = "4: 4 bit per pixel"]
    TOTALBITS_4 = 4,
    #[doc = "8: 8 bit per pixel"]
    TOTALBITS_8 = 8,
}
impl From<STOREBITSPERPIXEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STOREBITSPERPIXEL_A) -> Self {
        variant as _
    }
}
impl STOREBITSPERPIXEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STOREBITSPERPIXEL_A> {
        match self.bits {
            1 => Some(STOREBITSPERPIXEL_A::TOTALBITS_1),
            2 => Some(STOREBITSPERPIXEL_A::TOTALBITS_2),
            4 => Some(STOREBITSPERPIXEL_A::TOTALBITS_4),
            8 => Some(STOREBITSPERPIXEL_A::TOTALBITS_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_1`"]
    #[inline(always)]
    pub fn is_totalbits_1(&self) -> bool {
        *self == STOREBITSPERPIXEL_A::TOTALBITS_1
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_2`"]
    #[inline(always)]
    pub fn is_totalbits_2(&self) -> bool {
        *self == STOREBITSPERPIXEL_A::TOTALBITS_2
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_4`"]
    #[inline(always)]
    pub fn is_totalbits_4(&self) -> bool {
        *self == STOREBITSPERPIXEL_A::TOTALBITS_4
    }
    #[doc = "Checks if the value of the field is `TOTALBITS_8`"]
    #[inline(always)]
    pub fn is_totalbits_8(&self) -> bool {
        *self == STOREBITSPERPIXEL_A::TOTALBITS_8
    }
}
#[doc = "Field `STOREBITSPERPIXEL` writer - The resulting alpha mask can be saved to memory with the following BitsPerPixel settings."]
pub type STOREBITSPERPIXEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORECONTROL_SPEC, u8, STOREBITSPERPIXEL_A, 4, O>;
impl<'a, const O: u8> STOREBITSPERPIXEL_W<'a, O> {
    #[doc = "1 bit per pixel"]
    #[inline(always)]
    pub fn totalbits_1(self) -> &'a mut W {
        self.variant(STOREBITSPERPIXEL_A::TOTALBITS_1)
    }
    #[doc = "2 bit per pixel"]
    #[inline(always)]
    pub fn totalbits_2(self) -> &'a mut W {
        self.variant(STOREBITSPERPIXEL_A::TOTALBITS_2)
    }
    #[doc = "4 bit per pixel"]
    #[inline(always)]
    pub fn totalbits_4(self) -> &'a mut W {
        self.variant(STOREBITSPERPIXEL_A::TOTALBITS_4)
    }
    #[doc = "8 bit per pixel"]
    #[inline(always)]
    pub fn totalbits_8(self) -> &'a mut W {
        self.variant(STOREBITSPERPIXEL_A::TOTALBITS_8)
    }
}
#[doc = "Field `STORESTRIDE` reader - Destination buffer stride in bytes minus one. Used for address generation."]
pub type STORESTRIDE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STORESTRIDE` writer - Destination buffer stride in bytes minus one. Used for address generation."]
pub type STORESTRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORECONTROL_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:3 - The resulting alpha mask can be saved to memory with the following BitsPerPixel settings."]
    #[inline(always)]
    pub fn storebitsperpixel(&self) -> STOREBITSPERPIXEL_R {
        STOREBITSPERPIXEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:24 - Destination buffer stride in bytes minus one. Used for address generation."]
    #[inline(always)]
    pub fn storestride(&self) -> STORESTRIDE_R {
        STORESTRIDE_R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - The resulting alpha mask can be saved to memory with the following BitsPerPixel settings."]
    #[inline(always)]
    #[must_use]
    pub fn storebitsperpixel(&mut self) -> STOREBITSPERPIXEL_W<0> {
        STOREBITSPERPIXEL_W::new(self)
    }
    #[doc = "Bits 8:24 - Destination buffer stride in bytes minus one. Used for address generation."]
    #[inline(always)]
    #[must_use]
    pub fn storestride(&mut self) -> STORESTRIDE_W<8> {
        STORESTRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Store unit dynamic control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [storecontrol](index.html) module"]
pub struct STORECONTROL_SPEC;
impl crate::RegisterSpec for STORECONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [storecontrol::R](R) reader structure"]
impl crate::Readable for STORECONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [storecontrol::W](W) writer structure"]
impl crate::Writable for STORECONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORECONTROL to value 0x08"]
impl crate::Resettable for STORECONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
