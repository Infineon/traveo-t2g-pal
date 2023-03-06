#[doc = "Register `CLK_SELECT` reader"]
pub struct R(crate::R<CLK_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SELECT` writer"]
pub struct W(crate::W<CLK_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SELECT_SPEC>;
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
impl From<crate::W<CLK_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFCLK_SEL` reader - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
pub type LFCLK_SEL_R = crate::FieldReader<u8, LFCLK_SEL_A>;
#[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFCLK_SEL_A {
    #[doc = "0: ILO0 - Internal Low-speed Oscillator #0."]
    ILO0 = 0,
    #[doc = "1: WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    WCO = 1,
    #[doc = "2: ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    ALTLF = 2,
    #[doc = "3: PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    PILO = 3,
    #[doc = "4: ILO1 - Internal Low-speed Oscillator #1, if present."]
    ILO1 = 4,
    #[doc = "5: ECO_PRESCALER - External-Crystal Oscillator after prescaling, if present. Does not work in DEEPSLEEP or HIBERNATE modes. Intended for applications that operate in ACTIVE/SLEEP modes only. This option is only valid when ECO is present in the product."]
    ECO_PRESCALER = 5,
    #[doc = "6: LPECO_PRESCALER - Low-Power External-Crystal Oscillator after prescaling, if present. This choice works in ACTIVE/SLEEP/DEEPSLEEP modes. This option is only valid when LPECO is present in the product."]
    LPECO_PRESCALER = 6,
}
impl From<LFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LFCLK_SEL_A) -> Self {
        variant as _
    }
}
impl LFCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFCLK_SEL_A> {
        match self.bits {
            0 => Some(LFCLK_SEL_A::ILO0),
            1 => Some(LFCLK_SEL_A::WCO),
            2 => Some(LFCLK_SEL_A::ALTLF),
            3 => Some(LFCLK_SEL_A::PILO),
            4 => Some(LFCLK_SEL_A::ILO1),
            5 => Some(LFCLK_SEL_A::ECO_PRESCALER),
            6 => Some(LFCLK_SEL_A::LPECO_PRESCALER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ILO0`"]
    #[inline(always)]
    pub fn is_ilo0(&self) -> bool {
        *self == LFCLK_SEL_A::ILO0
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == LFCLK_SEL_A::WCO
    }
    #[doc = "Checks if the value of the field is `ALTLF`"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == LFCLK_SEL_A::ALTLF
    }
    #[doc = "Checks if the value of the field is `PILO`"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == LFCLK_SEL_A::PILO
    }
    #[doc = "Checks if the value of the field is `ILO1`"]
    #[inline(always)]
    pub fn is_ilo1(&self) -> bool {
        *self == LFCLK_SEL_A::ILO1
    }
    #[doc = "Checks if the value of the field is `ECO_PRESCALER`"]
    #[inline(always)]
    pub fn is_eco_prescaler(&self) -> bool {
        *self == LFCLK_SEL_A::ECO_PRESCALER
    }
    #[doc = "Checks if the value of the field is `LPECO_PRESCALER`"]
    #[inline(always)]
    pub fn is_lpeco_prescaler(&self) -> bool {
        *self == LFCLK_SEL_A::LPECO_PRESCALER
    }
}
#[doc = "Field `LFCLK_SEL` writer - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
pub type LFCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, LFCLK_SEL_A, 3, O>;
impl<'a, const O: u8> LFCLK_SEL_W<'a, O> {
    #[doc = "ILO0 - Internal Low-speed Oscillator #0."]
    #[inline(always)]
    pub fn ilo0(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ILO0)
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::WCO)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ALTLF)
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::PILO)
    }
    #[doc = "ILO1 - Internal Low-speed Oscillator #1, if present."]
    #[inline(always)]
    pub fn ilo1(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ILO1)
    }
    #[doc = "ECO_PRESCALER - External-Crystal Oscillator after prescaling, if present. Does not work in DEEPSLEEP or HIBERNATE modes. Intended for applications that operate in ACTIVE/SLEEP modes only. This option is only valid when ECO is present in the product."]
    #[inline(always)]
    pub fn eco_prescaler(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ECO_PRESCALER)
    }
    #[doc = "LPECO_PRESCALER - Low-Power External-Crystal Oscillator after prescaling, if present. This choice works in ACTIVE/SLEEP/DEEPSLEEP modes. This option is only valid when LPECO is present in the product."]
    #[inline(always)]
    pub fn lpeco_prescaler(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::LPECO_PRESCALER)
    }
}
#[doc = "Field `PUMP_SEL` reader - N/A"]
pub type PUMP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMP_SEL` writer - N/A"]
pub type PUMP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, u8, 4, O>;
#[doc = "Field `PUMP_DIV` reader - N/A"]
pub type PUMP_DIV_R = crate::FieldReader<u8, PUMP_DIV_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUMP_DIV_A {
    #[doc = "0: N/A"]
    NO_DIV = 0,
    #[doc = "1: N/A"]
    DIV_BY_2 = 1,
    #[doc = "2: N/A"]
    DIV_BY_4 = 2,
    #[doc = "3: N/A"]
    DIV_BY_8 = 3,
    #[doc = "4: N/A"]
    DIV_BY_16 = 4,
}
impl From<PUMP_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PUMP_DIV_A) -> Self {
        variant as _
    }
}
impl PUMP_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUMP_DIV_A> {
        match self.bits {
            0 => Some(PUMP_DIV_A::NO_DIV),
            1 => Some(PUMP_DIV_A::DIV_BY_2),
            2 => Some(PUMP_DIV_A::DIV_BY_4),
            3 => Some(PUMP_DIV_A::DIV_BY_8),
            4 => Some(PUMP_DIV_A::DIV_BY_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == PUMP_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_8
    }
    #[doc = "Checks if the value of the field is `DIV_BY_16`"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_16
    }
}
#[doc = "Field `PUMP_DIV` writer - N/A"]
pub type PUMP_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, PUMP_DIV_A, 3, O>;
impl<'a, const O: u8> PUMP_DIV_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::NO_DIV)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_4)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_16)
    }
}
#[doc = "Field `PUMP_ENABLE` reader - N/A"]
pub type PUMP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PUMP_ENABLE` writer - N/A"]
pub type PUMP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SELECT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LFCLK_SEL_R {
        LFCLK_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    pub fn pump_sel(&self) -> PUMP_SEL_R {
        PUMP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - N/A"]
    #[inline(always)]
    pub fn pump_div(&self) -> PUMP_DIV_R {
        PUMP_DIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn pump_enable(&self) -> PUMP_ENABLE_R {
        PUMP_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. It takes four cycles of the originally selected clock to switch away from it. Do not disable the original clock during this time."]
    #[inline(always)]
    #[must_use]
    pub fn lfclk_sel(&mut self) -> LFCLK_SEL_W<0> {
        LFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn pump_sel(&mut self) -> PUMP_SEL_W<8> {
        PUMP_SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn pump_div(&mut self) -> PUMP_DIV_W<12> {
        PUMP_DIV_W::new(self)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn pump_enable(&mut self) -> PUMP_ENABLE_W<15> {
        PUMP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_select](index.html) module"]
pub struct CLK_SELECT_SPEC;
impl crate::RegisterSpec for CLK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_select::R](R) reader structure"]
impl crate::Readable for CLK_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_select::W](W) writer structure"]
impl crate::Writable for CLK_SELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_SELECT to value 0"]
impl crate::Resettable for CLK_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
