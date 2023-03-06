#[doc = "Register `CLK_TRIM_ILO0_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_ILO0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_ILO0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_ILO0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_ILO0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_ILO0_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_ILO0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_ILO0_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_ILO0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_ILO0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILO0_FTRIM` reader - ILO0 frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO0_FTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILO0_FTRIM` writer - ILO0 frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO0_FTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ILO0_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ILO0_MONTRIM` reader - ILO0 internal monitor trim."]
pub type ILO0_MONTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILO0_MONTRIM` writer - ILO0 internal monitor trim."]
pub type ILO0_MONTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ILO0_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - ILO0 frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo0_ftrim(&self) -> ILO0_FTRIM_R {
        ILO0_FTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - ILO0 internal monitor trim."]
    #[inline(always)]
    pub fn ilo0_montrim(&self) -> ILO0_MONTRIM_R {
        ILO0_MONTRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ILO0 frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn ilo0_ftrim(&mut self) -> ILO0_FTRIM_W<0> {
        ILO0_FTRIM_W::new(self)
    }
    #[doc = "Bits 8:11 - ILO0 internal monitor trim."]
    #[inline(always)]
    #[must_use]
    pub fn ilo0_montrim(&mut self) -> ILO0_MONTRIM_W<8> {
        ILO0_MONTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ILO0 Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_ilo0_ctl](index.html) module"]
pub struct CLK_TRIM_ILO0_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ILO0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_ilo0_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_ILO0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_ilo0_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_ILO0_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_ILO0_CTL to value 0x052c"]
impl crate::Resettable for CLK_TRIM_ILO0_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x052c;
}
