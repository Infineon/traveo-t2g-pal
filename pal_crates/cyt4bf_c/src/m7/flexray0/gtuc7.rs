#[doc = "Register `GTUC7` reader"]
pub struct R(crate::R<GTUC7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC7` writer"]
pub struct W(crate::W<GTUC7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC7_SPEC>;
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
impl From<crate::W<GTUC7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSL` reader - Static Slot Length (gdStaticSlot) Configures the duration of a static slot in macroticks. The static slot length must be identical in all nodes of a cluster. Valid values are 4 to 659 MT."]
pub type SSL_R = crate::FieldReader<u16, SSL_A>;
#[doc = "Static Slot Length (gdStaticSlot) Configures the duration of a static slot in macroticks. The static slot length must be identical in all nodes of a cluster. Valid values are 4 to 659 MT.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SSL_A {
    #[doc = "4: N/A"]
    MIN = 4,
    #[doc = "659: N/A"]
    MAX = 659,
}
impl From<SSL_A> for u16 {
    #[inline(always)]
    fn from(variant: SSL_A) -> Self {
        variant as _
    }
}
impl SSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSL_A> {
        match self.bits {
            4 => Some(SSL_A::MIN),
            659 => Some(SSL_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == SSL_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == SSL_A::MAX
    }
}
#[doc = "Field `SSL` writer - Static Slot Length (gdStaticSlot) Configures the duration of a static slot in macroticks. The static slot length must be identical in all nodes of a cluster. Valid values are 4 to 659 MT."]
pub type SSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC7_SPEC, u16, SSL_A, 10, O>;
impl<'a, const O: u8> SSL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(SSL_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(SSL_A::MAX)
    }
}
#[doc = "Field `NSS` reader - Number of Static Slots (gNumberOfStaticSlots) Configures the number of static slots in a cycle. At least 2 coldstart nodes must be configured to startup a FlexRay network. The number of static slots must be identical in all nodes of a cluster. Valid values are 2 to 1023."]
pub type NSS_R = crate::FieldReader<u16, NSS_A>;
#[doc = "Number of Static Slots (gNumberOfStaticSlots) Configures the number of static slots in a cycle. At least 2 coldstart nodes must be configured to startup a FlexRay network. The number of static slots must be identical in all nodes of a cluster. Valid values are 2 to 1023.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NSS_A {
    #[doc = "2: N/A"]
    MIN = 2,
    #[doc = "1023: N/A"]
    MAX = 1023,
}
impl From<NSS_A> for u16 {
    #[inline(always)]
    fn from(variant: NSS_A) -> Self {
        variant as _
    }
}
impl NSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSS_A> {
        match self.bits {
            2 => Some(NSS_A::MIN),
            1023 => Some(NSS_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == NSS_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == NSS_A::MAX
    }
}
#[doc = "Field `NSS` writer - Number of Static Slots (gNumberOfStaticSlots) Configures the number of static slots in a cycle. At least 2 coldstart nodes must be configured to startup a FlexRay network. The number of static slots must be identical in all nodes of a cluster. Valid values are 2 to 1023."]
pub type NSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC7_SPEC, u16, NSS_A, 10, O>;
impl<'a, const O: u8> NSS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(NSS_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(NSS_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:9 - Static Slot Length (gdStaticSlot) Configures the duration of a static slot in macroticks. The static slot length must be identical in all nodes of a cluster. Valid values are 4 to 659 MT."]
    #[inline(always)]
    pub fn ssl(&self) -> SSL_R {
        SSL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Number of Static Slots (gNumberOfStaticSlots) Configures the number of static slots in a cycle. At least 2 coldstart nodes must be configured to startup a FlexRay network. The number of static slots must be identical in all nodes of a cluster. Valid values are 2 to 1023."]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Static Slot Length (gdStaticSlot) Configures the duration of a static slot in macroticks. The static slot length must be identical in all nodes of a cluster. Valid values are 4 to 659 MT."]
    #[inline(always)]
    #[must_use]
    pub fn ssl(&mut self) -> SSL_W<0> {
        SSL_W::new(self)
    }
    #[doc = "Bits 16:25 - Number of Static Slots (gNumberOfStaticSlots) Configures the number of static slots in a cycle. At least 2 coldstart nodes must be configured to startup a FlexRay network. The number of static slots must be identical in all nodes of a cluster. Valid values are 2 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<16> {
        NSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc7](index.html) module"]
pub struct GTUC7_SPEC;
impl crate::RegisterSpec for GTUC7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc7::R](R) reader structure"]
impl crate::Readable for GTUC7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc7::W](W) writer structure"]
impl crate::Writable for GTUC7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC7 to value 0x0002_0004"]
impl crate::Resettable for GTUC7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0004;
}
