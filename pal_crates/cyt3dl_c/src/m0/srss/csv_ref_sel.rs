#[doc = "Register `CSV_REF_SEL` reader"]
pub struct R(crate::R<CSV_REF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSV_REF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSV_REF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSV_REF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSV_REF_SEL` writer"]
pub struct W(crate::W<CSV_REF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSV_REF_SEL_SPEC>;
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
impl From<crate::W<CSV_REF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSV_REF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_MUX` reader - Selects a source for clock clk_ref_hf. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
pub type REF_MUX_R = crate::FieldReader<u8, REF_MUX_A>;
#[doc = "Selects a source for clock clk_ref_hf. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_MUX_A {
    #[doc = "0: IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "1: EXTCLK - External Clock Pin"]
    EXTCLK = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    ECO = 2,
    #[doc = "3: ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    ALTHF = 3,
}
impl From<REF_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_MUX_A) -> Self {
        variant as _
    }
}
impl REF_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REF_MUX_A> {
        match self.bits {
            0 => Some(REF_MUX_A::IMO),
            1 => Some(REF_MUX_A::EXTCLK),
            2 => Some(REF_MUX_A::ECO),
            3 => Some(REF_MUX_A::ALTHF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == REF_MUX_A::IMO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == REF_MUX_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == REF_MUX_A::ECO
    }
    #[doc = "Checks if the value of the field is `ALTHF`"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == REF_MUX_A::ALTHF
    }
}
#[doc = "Field `REF_MUX` writer - Selects a source for clock clk_ref_hf. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
pub type REF_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSV_REF_SEL_SPEC, u8, REF_MUX_A, 3, O>;
impl<'a, const O: u8> REF_MUX_W<'a, O> {
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(REF_MUX_A::IMO)
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(REF_MUX_A::EXTCLK)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(REF_MUX_A::ECO)
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut W {
        self.variant(REF_MUX_A::ALTHF)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects a source for clock clk_ref_hf. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
    #[inline(always)]
    pub fn ref_mux(&self) -> REF_MUX_R {
        REF_MUX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a source for clock clk_ref_hf. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
    #[inline(always)]
    #[must_use]
    pub fn ref_mux(&mut self) -> REF_MUX_W<0> {
        REF_MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select CSV Reference clock for Active domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csv_ref_sel](index.html) module"]
pub struct CSV_REF_SEL_SPEC;
impl crate::RegisterSpec for CSV_REF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csv_ref_sel::R](R) reader structure"]
impl crate::Readable for CSV_REF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csv_ref_sel::W](W) writer structure"]
impl crate::Writable for CSV_REF_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSV_REF_SEL to value 0"]
impl crate::Resettable for CSV_REF_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
