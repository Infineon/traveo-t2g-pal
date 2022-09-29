#[doc = "Register `AES_CTL` reader"]
pub struct R(crate::R<AES_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_CTL` writer"]
pub struct W(crate::W<AES_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_CTL_SPEC>;
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
impl From<crate::W<AES_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_SIZE` reader - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
pub type KEY_SIZE_R = crate::FieldReader<u8, KEY_SIZE_A>;
#[doc = "AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_SIZE_A {
    #[doc = "0: N/A"]
    AES128 = 0,
    #[doc = "1: N/A"]
    AES192 = 1,
    #[doc = "2: N/A"]
    AES256 = 2,
}
impl From<KEY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SIZE_A) -> Self {
        variant as _
    }
}
impl KEY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_SIZE_A> {
        match self.bits {
            0 => Some(KEY_SIZE_A::AES128),
            1 => Some(KEY_SIZE_A::AES192),
            2 => Some(KEY_SIZE_A::AES256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEY_SIZE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEY_SIZE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEY_SIZE_A::AES256
    }
}
#[doc = "Field `KEY_SIZE` writer - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
pub type KEY_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AES_CTL_SPEC, u8, KEY_SIZE_A, 2, O>;
impl<'a, const O: u8> KEY_SIZE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES128)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES192)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES256)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES key size: '0': 128-bit key, 10 rounds AES (inverse) cipher operation. '1': 192-bit key, 12 rounds AES (inverse) cipher operation. '2': 256-bit key, 14 rounds AES (inverse) cipher operation. '3': Undefined"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<0> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ctl](index.html) module"]
pub struct AES_CTL_SPEC;
impl crate::RegisterSpec for AES_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_ctl::R](R) reader structure"]
impl crate::Readable for AES_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_ctl::W](W) writer structure"]
impl crate::Writable for AES_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_CTL to value 0"]
impl crate::Resettable for AES_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
