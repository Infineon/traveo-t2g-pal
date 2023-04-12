#[doc = "Register `CLK_GEN_CMD` reader"]
pub struct R(crate::R<CLK_GEN_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GEN_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GEN_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GEN_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GEN_CMD` writer"]
pub struct W(crate::W<CLK_GEN_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GEN_CMD_SPEC>;
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
impl From<crate::W<CLK_GEN_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GEN_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE8` reader - Enable Dual-Pixel (8-Lane Mode)"]
pub type MODE8_R = crate::BitReader<MODE8_A>;
#[doc = "Enable Dual-Pixel (8-Lane Mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE8_A {
    #[doc = "0: 4 Lane LVDS mode"]
    MODE4 = 0,
    #[doc = "1: 8 lane LVDS mode"]
    MODE8 = 1,
}
impl From<MODE8_A> for bool {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            false => MODE8_A::MODE4,
            true => MODE8_A::MODE8,
        }
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == MODE8_A::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE8`"]
    #[inline(always)]
    pub fn is_mode8(&self) -> bool {
        *self == MODE8_A::MODE8
    }
}
#[doc = "Field `MODE8` writer - Enable Dual-Pixel (8-Lane Mode)"]
pub type MODE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GEN_CMD_SPEC, MODE8_A, O>;
impl<'a, const O: u8> MODE8_W<'a, O> {
    #[doc = "4 Lane LVDS mode"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(MODE8_A::MODE4)
    }
    #[doc = "8 lane LVDS mode"]
    #[inline(always)]
    pub fn mode8(self) -> &'a mut W {
        self.variant(MODE8_A::MODE8)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Dual-Pixel (8-Lane Mode)"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Dual-Pixel (8-Lane Mode)"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<0> {
        MODE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for CLK_GEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gen_cmd](index.html) module"]
pub struct CLK_GEN_CMD_SPEC;
impl crate::RegisterSpec for CLK_GEN_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gen_cmd::R](R) reader structure"]
impl crate::Readable for CLK_GEN_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gen_cmd::W](W) writer structure"]
impl crate::Writable for CLK_GEN_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_GEN_CMD to value 0"]
impl crate::Resettable for CLK_GEN_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
