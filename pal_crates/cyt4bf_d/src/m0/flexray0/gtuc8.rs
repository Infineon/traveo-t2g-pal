#[doc = "Register `GTUC8` reader"]
pub struct R(crate::R<GTUC8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC8` writer"]
pub struct W(crate::W<GTUC8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC8_SPEC>;
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
impl From<crate::W<GTUC8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSL` reader - Minislot Length (gdMinislot) Configures the duration of a minislot in macroticks. The minislot length must be identical in all nodes of a cluster. Valid values are 2 to 63 MT."]
pub type MSL_R = crate::FieldReader<u8, MSL_A>;
#[doc = "Minislot Length (gdMinislot) Configures the duration of a minislot in macroticks. The minislot length must be identical in all nodes of a cluster. Valid values are 2 to 63 MT.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSL_A {
    #[doc = "2: N/A"]
    MIN = 2,
    #[doc = "63: N/A"]
    MAX = 63,
}
impl From<MSL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSL_A) -> Self {
        variant as _
    }
}
impl MSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSL_A> {
        match self.bits {
            2 => Some(MSL_A::MIN),
            63 => Some(MSL_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == MSL_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == MSL_A::MAX
    }
}
#[doc = "Field `MSL` writer - Minislot Length (gdMinislot) Configures the duration of a minislot in macroticks. The minislot length must be identical in all nodes of a cluster. Valid values are 2 to 63 MT."]
pub type MSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC8_SPEC, u8, MSL_A, 6, O>;
impl<'a, const O: u8> MSL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(MSL_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(MSL_A::MAX)
    }
}
#[doc = "Field `NMS` reader - Number of Minislots (gNumberOfMinislots) Configures the number of minislots within the dynamic segment of a cycle. The number of minislots must be identical in all nodes of a cluster. Valid values are 0 to 7986."]
pub type NMS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NMS` writer - Number of Minislots (gNumberOfMinislots) Configures the number of minislots within the dynamic segment of a cycle. The number of minislots must be identical in all nodes of a cluster. Valid values are 0 to 7986."]
pub type NMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC8_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:5 - Minislot Length (gdMinislot) Configures the duration of a minislot in macroticks. The minislot length must be identical in all nodes of a cluster. Valid values are 2 to 63 MT."]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:28 - Number of Minislots (gNumberOfMinislots) Configures the number of minislots within the dynamic segment of a cycle. The number of minislots must be identical in all nodes of a cluster. Valid values are 0 to 7986."]
    #[inline(always)]
    pub fn nms(&self) -> NMS_R {
        NMS_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minislot Length (gdMinislot) Configures the duration of a minislot in macroticks. The minislot length must be identical in all nodes of a cluster. Valid values are 2 to 63 MT."]
    #[inline(always)]
    #[must_use]
    pub fn msl(&mut self) -> MSL_W<0> {
        MSL_W::new(self)
    }
    #[doc = "Bits 16:28 - Number of Minislots (gNumberOfMinislots) Configures the number of minislots within the dynamic segment of a cycle. The number of minislots must be identical in all nodes of a cluster. Valid values are 0 to 7986."]
    #[inline(always)]
    #[must_use]
    pub fn nms(&mut self) -> NMS_W<16> {
        NMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc8](index.html) module"]
pub struct GTUC8_SPEC;
impl crate::RegisterSpec for GTUC8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc8::R](R) reader structure"]
impl crate::Readable for GTUC8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc8::W](W) writer structure"]
impl crate::Writable for GTUC8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC8 to value 0x02"]
impl crate::Resettable for GTUC8_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
