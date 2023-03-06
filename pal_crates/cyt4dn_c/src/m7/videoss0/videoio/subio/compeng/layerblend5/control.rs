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
#[doc = "Field `MODE` reader - Operation mode."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Operation mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Module is in neutral mode. Output is same as primary input."]
    NEUTRAL = 0,
    #[doc = "1: Module is in blending mode."]
    BLEND = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::NEUTRAL,
            true => MODE_A::BLEND,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `BLEND`"]
    #[inline(always)]
    pub fn is_blend(&self) -> bool {
        *self == MODE_A::BLEND
    }
}
#[doc = "Field `MODE` writer - Operation mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Module is in neutral mode. Output is same as primary input."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "Module is in blending mode."]
    #[inline(always)]
    pub fn blend(self) -> &'a mut W {
        self.variant(MODE_A::BLEND)
    }
}
#[doc = "Field `ALPHAMASKENABLE` reader - Enables Alpha Mask feature. This will limit possible output alpha values to 0 and 255. Generation of this alpha value will depend on the AlphaMaskMode field."]
pub type ALPHAMASKENABLE_R = crate::BitReader<ALPHAMASKENABLE_A>;
#[doc = "Enables Alpha Mask feature. This will limit possible output alpha values to 0 and 255. Generation of this alpha value will depend on the AlphaMaskMode field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALPHAMASKENABLE_A {
    #[doc = "0: AlphaMask feature disabled"]
    DISABLE = 0,
    #[doc = "1: AlphaMask feature enabled"]
    ENABLE = 1,
}
impl From<ALPHAMASKENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ALPHAMASKENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALPHAMASKENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHAMASKENABLE_A {
        match self.bits {
            false => ALPHAMASKENABLE_A::DISABLE,
            true => ALPHAMASKENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALPHAMASKENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALPHAMASKENABLE_A::ENABLE
    }
}
#[doc = "Field `ALPHAMASKENABLE` writer - Enables Alpha Mask feature. This will limit possible output alpha values to 0 and 255. Generation of this alpha value will depend on the AlphaMaskMode field."]
pub type ALPHAMASKENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, ALPHAMASKENABLE_A, O>;
impl<'a, const O: u8> ALPHAMASKENABLE_W<'a, O> {
    #[doc = "AlphaMask feature disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALPHAMASKENABLE_A::DISABLE)
    }
    #[doc = "AlphaMask feature enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALPHAMASKENABLE_A::ENABLE)
    }
}
#[doc = "Field `ALPHAMASKMODE` reader - AlphaMaskMode determines how the output alpha is generated when AlphaMaskEnable is set to ENABLE"]
pub type ALPHAMASKMODE_R = crate::FieldReader<u8, ALPHAMASKMODE_A>;
#[doc = "AlphaMaskMode determines how the output alpha is generated when AlphaMaskEnable is set to ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALPHAMASKMODE_A {
    #[doc = "0: Areas with primary input alpha > 128 will be mapped to 255 and the rest will have an alpha value of 0"]
    PRIM = 0,
    #[doc = "1: The area of the secondary input will get an alpha value of 255 and the rest will be 0"]
    SEC = 1,
    #[doc = "2: Behaves as if the output of modes PRIM and SEC would be ORed together"]
    PRIM_OR_SEC = 2,
    #[doc = "3: Behaves as if the output of modes PRIM and SEC would be ANDed together"]
    PRIM_AND_SEC = 3,
    #[doc = "4: Behaves as if the output of mode PRIM would be inverted"]
    PRIM_INV = 4,
    #[doc = "5: Behaves as if the output of mode SEC would be inverted"]
    SEC_INV = 5,
    #[doc = "6: Behaves as if the output of modes PRIM and SEC_INV would be ORed together"]
    PRIM_OR_SEC_INV = 6,
    #[doc = "7: Behaves as if the output of modes PRIM and SEC_INV would be ANDed together"]
    PRIM_AND_SEC_INV = 7,
}
impl From<ALPHAMASKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALPHAMASKMODE_A) -> Self {
        variant as _
    }
}
impl ALPHAMASKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHAMASKMODE_A {
        match self.bits {
            0 => ALPHAMASKMODE_A::PRIM,
            1 => ALPHAMASKMODE_A::SEC,
            2 => ALPHAMASKMODE_A::PRIM_OR_SEC,
            3 => ALPHAMASKMODE_A::PRIM_AND_SEC,
            4 => ALPHAMASKMODE_A::PRIM_INV,
            5 => ALPHAMASKMODE_A::SEC_INV,
            6 => ALPHAMASKMODE_A::PRIM_OR_SEC_INV,
            7 => ALPHAMASKMODE_A::PRIM_AND_SEC_INV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRIM`"]
    #[inline(always)]
    pub fn is_prim(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM
    }
    #[doc = "Checks if the value of the field is `SEC`"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == ALPHAMASKMODE_A::SEC
    }
    #[doc = "Checks if the value of the field is `PRIM_OR_SEC`"]
    #[inline(always)]
    pub fn is_prim_or_sec(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM_OR_SEC
    }
    #[doc = "Checks if the value of the field is `PRIM_AND_SEC`"]
    #[inline(always)]
    pub fn is_prim_and_sec(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM_AND_SEC
    }
    #[doc = "Checks if the value of the field is `PRIM_INV`"]
    #[inline(always)]
    pub fn is_prim_inv(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM_INV
    }
    #[doc = "Checks if the value of the field is `SEC_INV`"]
    #[inline(always)]
    pub fn is_sec_inv(&self) -> bool {
        *self == ALPHAMASKMODE_A::SEC_INV
    }
    #[doc = "Checks if the value of the field is `PRIM_OR_SEC_INV`"]
    #[inline(always)]
    pub fn is_prim_or_sec_inv(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM_OR_SEC_INV
    }
    #[doc = "Checks if the value of the field is `PRIM_AND_SEC_INV`"]
    #[inline(always)]
    pub fn is_prim_and_sec_inv(&self) -> bool {
        *self == ALPHAMASKMODE_A::PRIM_AND_SEC_INV
    }
}
#[doc = "Field `ALPHAMASKMODE` writer - AlphaMaskMode determines how the output alpha is generated when AlphaMaskEnable is set to ENABLE"]
pub type ALPHAMASKMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, ALPHAMASKMODE_A, 3, O>;
impl<'a, const O: u8> ALPHAMASKMODE_W<'a, O> {
    #[doc = "Areas with primary input alpha > 128 will be mapped to 255 and the rest will have an alpha value of 0"]
    #[inline(always)]
    pub fn prim(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM)
    }
    #[doc = "The area of the secondary input will get an alpha value of 255 and the rest will be 0"]
    #[inline(always)]
    pub fn sec(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::SEC)
    }
    #[doc = "Behaves as if the output of modes PRIM and SEC would be ORed together"]
    #[inline(always)]
    pub fn prim_or_sec(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM_OR_SEC)
    }
    #[doc = "Behaves as if the output of modes PRIM and SEC would be ANDed together"]
    #[inline(always)]
    pub fn prim_and_sec(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM_AND_SEC)
    }
    #[doc = "Behaves as if the output of mode PRIM would be inverted"]
    #[inline(always)]
    pub fn prim_inv(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM_INV)
    }
    #[doc = "Behaves as if the output of mode SEC would be inverted"]
    #[inline(always)]
    pub fn sec_inv(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::SEC_INV)
    }
    #[doc = "Behaves as if the output of modes PRIM and SEC_INV would be ORed together"]
    #[inline(always)]
    pub fn prim_or_sec_inv(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM_OR_SEC_INV)
    }
    #[doc = "Behaves as if the output of modes PRIM and SEC_INV would be ANDed together"]
    #[inline(always)]
    pub fn prim_and_sec_inv(self) -> &'a mut W {
        self.variant(ALPHAMASKMODE_A::PRIM_AND_SEC_INV)
    }
}
#[doc = "Field `SECLOWPASSEN` reader - Enables a horizontal low-pass filter for the secondary input port. Should be used for dual view and dual display mode when the secondary input frame has twice the resolution of one view (= full resolution of the panel interface). Every 2nd pixel only is sampled then."]
pub type SECLOWPASSEN_R = crate::BitReader<bool>;
#[doc = "Field `SECLOWPASSEN` writer - Enables a horizontal low-pass filter for the secondary input port. Should be used for dual view and dual display mode when the secondary input frame has twice the resolution of one view (= full resolution of the panel interface). Every 2nd pixel only is sampled then."]
pub type SECLOWPASSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `SECREPLICATEEN` reader - Enables horizontal replication of pixels by factor 2 on secondary input port. Must be used for dual display mode when the secondary input frame has the resolution of one display (= half the resolution of the panel interface)."]
pub type SECREPLICATEEN_R = crate::BitReader<bool>;
#[doc = "Field `SECREPLICATEEN` writer - Enables horizontal replication of pixels by factor 2 on secondary input port. Must be used for dual display mode when the secondary input frame has the resolution of one display (= half the resolution of the panel interface)."]
pub type SECREPLICATEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `SECEVENROWEVENCOLDIS` reader - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is even. R/Y is MSBit, A is LSBit."]
pub type SECEVENROWEVENCOLDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECEVENROWEVENCOLDIS` writer - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is even. R/Y is MSBit, A is LSBit."]
pub type SECEVENROWEVENCOLDIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECEVENROWODDCOLDIS` reader - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is even and column index is odd. R/Y is MSBit, A is LSBit."]
pub type SECEVENROWODDCOLDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECEVENROWODDCOLDIS` writer - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is even and column index is odd. R/Y is MSBit, A is LSBit."]
pub type SECEVENROWODDCOLDIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECODDROWEVENCOLDIS` reader - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is odd and column index is even. R/Y is MSBit, A is LSBit."]
pub type SECODDROWEVENCOLDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECODDROWEVENCOLDIS` writer - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is odd and column index is even. R/Y is MSBit, A is LSBit."]
pub type SECODDROWEVENCOLDIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECODDROWODDCOLDIS` reader - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is odd. R/Y is MSBit, A is LSBit."]
pub type SECODDROWODDCOLDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECODDROWODDCOLDIS` writer - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is odd. R/Y is MSBit, A is LSBit."]
pub type SECODDROWODDCOLDIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Operation mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enables Alpha Mask feature. This will limit possible output alpha values to 0 and 255. Generation of this alpha value will depend on the AlphaMaskMode field."]
    #[inline(always)]
    pub fn alphamaskenable(&self) -> ALPHAMASKENABLE_R {
        ALPHAMASKENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - AlphaMaskMode determines how the output alpha is generated when AlphaMaskEnable is set to ENABLE"]
    #[inline(always)]
    pub fn alphamaskmode(&self) -> ALPHAMASKMODE_R {
        ALPHAMASKMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Enables a horizontal low-pass filter for the secondary input port. Should be used for dual view and dual display mode when the secondary input frame has twice the resolution of one view (= full resolution of the panel interface). Every 2nd pixel only is sampled then."]
    #[inline(always)]
    pub fn seclowpassen(&self) -> SECLOWPASSEN_R {
        SECLOWPASSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables horizontal replication of pixels by factor 2 on secondary input port. Must be used for dual display mode when the secondary input frame has the resolution of one display (= half the resolution of the panel interface)."]
    #[inline(always)]
    pub fn secreplicateen(&self) -> SECREPLICATEEN_R {
        SECREPLICATEEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is even. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    pub fn secevenrowevencoldis(&self) -> SECEVENROWEVENCOLDIS_R {
        SECEVENROWEVENCOLDIS_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is even and column index is odd. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    pub fn secevenrowoddcoldis(&self) -> SECEVENROWODDCOLDIS_R {
        SECEVENROWODDCOLDIS_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is odd and column index is even. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    pub fn secoddrowevencoldis(&self) -> SECODDROWEVENCOLDIS_R {
        SECODDROWEVENCOLDIS_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is odd. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    pub fn secoddrowoddcoldis(&self) -> SECODDROWODDCOLDIS_R {
        SECODDROWODDCOLDIS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Enables Alpha Mask feature. This will limit possible output alpha values to 0 and 255. Generation of this alpha value will depend on the AlphaMaskMode field."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable(&mut self) -> ALPHAMASKENABLE_W<2> {
        ALPHAMASKENABLE_W::new(self)
    }
    #[doc = "Bits 4:6 - AlphaMaskMode determines how the output alpha is generated when AlphaMaskEnable is set to ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskmode(&mut self) -> ALPHAMASKMODE_W<4> {
        ALPHAMASKMODE_W::new(self)
    }
    #[doc = "Bit 8 - Enables a horizontal low-pass filter for the secondary input port. Should be used for dual view and dual display mode when the secondary input frame has twice the resolution of one view (= full resolution of the panel interface). Every 2nd pixel only is sampled then."]
    #[inline(always)]
    #[must_use]
    pub fn seclowpassen(&mut self) -> SECLOWPASSEN_W<8> {
        SECLOWPASSEN_W::new(self)
    }
    #[doc = "Bit 9 - Enables horizontal replication of pixels by factor 2 on secondary input port. Must be used for dual display mode when the secondary input frame has the resolution of one display (= half the resolution of the panel interface)."]
    #[inline(always)]
    #[must_use]
    pub fn secreplicateen(&mut self) -> SECREPLICATEEN_W<9> {
        SECREPLICATEEN_W::new(self)
    }
    #[doc = "Bits 10:13 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is even. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    #[must_use]
    pub fn secevenrowevencoldis(&mut self) -> SECEVENROWEVENCOLDIS_W<10> {
        SECEVENROWEVENCOLDIS_W::new(self)
    }
    #[doc = "Bits 14:17 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is even and column index is odd. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    #[must_use]
    pub fn secevenrowoddcoldis(&mut self) -> SECEVENROWODDCOLDIS_W<14> {
        SECEVENROWODDCOLDIS_W::new(self)
    }
    #[doc = "Bits 18:21 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row index is odd and column index is even. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    #[must_use]
    pub fn secoddrowevencoldis(&mut self) -> SECODDROWEVENCOLDIS_W<18> {
        SECODDROWEVENCOLDIS_W::new(self)
    }
    #[doc = "Bits 22:25 - Disable bits for R/Y, G/U, B/V and A channels of secondary input when row and column index is odd. R/Y is MSBit, A is LSBit."]
    #[inline(always)]
    #[must_use]
    pub fn secoddrowoddcoldis(&mut self) -> SECODDROWODDCOLDIS_W<22> {
        SECODDROWODDCOLDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
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
#[doc = "`reset()` method sets CONTROL to value 0x01"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
