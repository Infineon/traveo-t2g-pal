#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operation mode for color matrix"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Operation mode for color matrix\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Module in neutral mode, input data is bypassed"]
    NEUTRAL = 0,
    #[doc = "1: Module in matrix mode, input data is multiplied with matrix values"]
    MATRIX = 1,
    #[doc = "2: Module in alpha pre-multiplication mode, input color is multiplied with input alpha"]
    PREMUL = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NEUTRAL,
            1 => MODE_A::MATRIX,
            2 => MODE_A::PREMUL,
            3 => MODE_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `MATRIX`"]
    #[inline(always)]
    pub fn is_matrix(&self) -> bool {
        *self == MODE_A::MATRIX
    }
    #[doc = "Checks if the value of the field is `PREMUL`"]
    #[inline(always)]
    pub fn is_premul(&self) -> bool {
        *self == MODE_A::PREMUL
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == MODE_A::RSVD
    }
}
#[doc = "Field `MODE` writer - Operation mode for color matrix"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Module in neutral mode, input data is bypassed"]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "Module in matrix mode, input data is multiplied with matrix values"]
    #[inline(always)]
    pub fn matrix(self) -> &'a mut W {
        self.variant(MODE_A::MATRIX)
    }
    #[doc = "Module in alpha pre-multiplication mode, input color is multiplied with input alpha"]
    #[inline(always)]
    pub fn premul(self) -> &'a mut W {
        self.variant(MODE_A::PREMUL)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(MODE_A::RSVD)
    }
}
#[doc = "Field `ALPHAMASK` reader - Value 1 enables the alpha mask mode. In this mode all pixels with an alpha value smaller than 0.5 are by-passed unchanged."]
pub type ALPHAMASK_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK` writer - Value 1 enables the alpha mask mode. In this mode all pixels with an alpha value smaller than 0.5 are by-passed unchanged."]
pub type ALPHAMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHAINVERT` reader - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value greater or equal 0.5 are by-passed)."]
pub type ALPHAINVERT_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINVERT` writer - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value greater or equal 0.5 are by-passed)."]
pub type ALPHAINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Operation mode for color matrix"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Value 1 enables the alpha mask mode. In this mode all pixels with an alpha value smaller than 0.5 are by-passed unchanged."]
    #[inline(always)]
    pub fn alphamask(&self) -> ALPHAMASK_R {
        ALPHAMASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value greater or equal 0.5 are by-passed)."]
    #[inline(always)]
    pub fn alphainvert(&self) -> ALPHAINVERT_R {
        ALPHAINVERT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation mode for color matrix"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Value 1 enables the alpha mask mode. In this mode all pixels with an alpha value smaller than 0.5 are by-passed unchanged."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask(&mut self) -> ALPHAMASK_W<4> {
        ALPHAMASK_W::new(self)
    }
    #[doc = "Bit 5 - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value greater or equal 0.5 are by-passed)."]
    #[inline(always)]
    #[must_use]
    pub fn alphainvert(&mut self) -> ALPHAINVERT_W<5> {
        ALPHAINVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color Matrix control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
