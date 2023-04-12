#[doc = "Register `CFG_OUT2` reader"]
pub struct R(crate::R<CFG_OUT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_OUT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_OUT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_OUT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_OUT2` writer"]
pub struct W(crate::W<CFG_OUT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_OUT2_SPEC>;
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
impl From<crate::W<CFG_OUT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_OUT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS_TRIM0` reader - Sets the Drive Select Trim for IO pin 0 0 - Default (50ohms) 1 - 120ohms 2 - 90ohms 3 - 60ohms 4 - 50ohms 5 - 30ohms 6 - 20ohms 7 - 15ohms"]
pub type DS_TRIM0_R = crate::FieldReader<u8, DS_TRIM0_A>;
#[doc = "Sets the Drive Select Trim for IO pin 0 0 - Default (50ohms) 1 - 120ohms 2 - 90ohms 3 - 60ohms 4 - 50ohms 5 - 30ohms 6 - 20ohms 7 - 15ohms\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS_TRIM0_A {
    #[doc = "0: N/A"]
    DEFAULT = 0,
    #[doc = "1: N/A"]
    DS_120OHM = 1,
    #[doc = "2: N/A"]
    DS_90OHM = 2,
    #[doc = "3: N/A"]
    DS_60OHM = 3,
    #[doc = "4: N/A"]
    DS_50OHM = 4,
    #[doc = "5: N/A"]
    DS_30OHM = 5,
    #[doc = "6: N/A"]
    DS_20OHM = 6,
    #[doc = "7: N/A"]
    DS_15OHM = 7,
}
impl From<DS_TRIM0_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_TRIM0_A) -> Self {
        variant as _
    }
}
impl DS_TRIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS_TRIM0_A {
        match self.bits {
            0 => DS_TRIM0_A::DEFAULT,
            1 => DS_TRIM0_A::DS_120OHM,
            2 => DS_TRIM0_A::DS_90OHM,
            3 => DS_TRIM0_A::DS_60OHM,
            4 => DS_TRIM0_A::DS_50OHM,
            5 => DS_TRIM0_A::DS_30OHM,
            6 => DS_TRIM0_A::DS_20OHM,
            7 => DS_TRIM0_A::DS_15OHM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == DS_TRIM0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DS_120OHM`"]
    #[inline(always)]
    pub fn is_ds_120ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_120OHM
    }
    #[doc = "Checks if the value of the field is `DS_90OHM`"]
    #[inline(always)]
    pub fn is_ds_90ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_90OHM
    }
    #[doc = "Checks if the value of the field is `DS_60OHM`"]
    #[inline(always)]
    pub fn is_ds_60ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_60OHM
    }
    #[doc = "Checks if the value of the field is `DS_50OHM`"]
    #[inline(always)]
    pub fn is_ds_50ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_50OHM
    }
    #[doc = "Checks if the value of the field is `DS_30OHM`"]
    #[inline(always)]
    pub fn is_ds_30ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_30OHM
    }
    #[doc = "Checks if the value of the field is `DS_20OHM`"]
    #[inline(always)]
    pub fn is_ds_20ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_20OHM
    }
    #[doc = "Checks if the value of the field is `DS_15OHM`"]
    #[inline(always)]
    pub fn is_ds_15ohm(&self) -> bool {
        *self == DS_TRIM0_A::DS_15OHM
    }
}
#[doc = "Field `DS_TRIM0` writer - Sets the Drive Select Trim for IO pin 0 0 - Default (50ohms) 1 - 120ohms 2 - 90ohms 3 - 60ohms 4 - 50ohms 5 - 30ohms 6 - 20ohms 7 - 15ohms"]
pub type DS_TRIM0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_OUT2_SPEC, u8, DS_TRIM0_A, 3, O>;
impl<'a, const O: u8> DS_TRIM0_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DEFAULT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_120ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_120OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_90ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_90OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_60ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_60OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_50ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_50OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_30ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_30OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_20ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_20OHM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ds_15ohm(self) -> &'a mut W {
        self.variant(DS_TRIM0_A::DS_15OHM)
    }
}
#[doc = "Field `DS_TRIM1` reader - Sets the Drive Select Trim for IO pin 1"]
pub type DS_TRIM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM1` writer - Sets the Drive Select Trim for IO pin 1"]
pub type DS_TRIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM2` reader - Sets the Drive Select Trim for IO pin 2"]
pub type DS_TRIM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM2` writer - Sets the Drive Select Trim for IO pin 2"]
pub type DS_TRIM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM3` reader - Sets the Drive Select Trim for IO pin 3"]
pub type DS_TRIM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM3` writer - Sets the Drive Select Trim for IO pin 3"]
pub type DS_TRIM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM4` reader - Sets the Drive Select Trim for IO pin 4"]
pub type DS_TRIM4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM4` writer - Sets the Drive Select Trim for IO pin 4"]
pub type DS_TRIM4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM5` reader - Sets the Drive Select Trim for IO pin 5"]
pub type DS_TRIM5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM5` writer - Sets the Drive Select Trim for IO pin 5"]
pub type DS_TRIM5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM6` reader - Sets the Drive Select Trim for IO pin 6"]
pub type DS_TRIM6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM6` writer - Sets the Drive Select Trim for IO pin 6"]
pub type DS_TRIM6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS_TRIM7` reader - Sets the Drive Select Trim for IO pin 7"]
pub type DS_TRIM7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS_TRIM7` writer - Sets the Drive Select Trim for IO pin 7"]
pub type DS_TRIM7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_OUT2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Sets the Drive Select Trim for IO pin 0 0 - Default (50ohms) 1 - 120ohms 2 - 90ohms 3 - 60ohms 4 - 50ohms 5 - 30ohms 6 - 20ohms 7 - 15ohms"]
    #[inline(always)]
    pub fn ds_trim0(&self) -> DS_TRIM0_R {
        DS_TRIM0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Sets the Drive Select Trim for IO pin 1"]
    #[inline(always)]
    pub fn ds_trim1(&self) -> DS_TRIM1_R {
        DS_TRIM1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Sets the Drive Select Trim for IO pin 2"]
    #[inline(always)]
    pub fn ds_trim2(&self) -> DS_TRIM2_R {
        DS_TRIM2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Sets the Drive Select Trim for IO pin 3"]
    #[inline(always)]
    pub fn ds_trim3(&self) -> DS_TRIM3_R {
        DS_TRIM3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Sets the Drive Select Trim for IO pin 4"]
    #[inline(always)]
    pub fn ds_trim4(&self) -> DS_TRIM4_R {
        DS_TRIM4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Sets the Drive Select Trim for IO pin 5"]
    #[inline(always)]
    pub fn ds_trim5(&self) -> DS_TRIM5_R {
        DS_TRIM5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Sets the Drive Select Trim for IO pin 6"]
    #[inline(always)]
    pub fn ds_trim6(&self) -> DS_TRIM6_R {
        DS_TRIM6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Sets the Drive Select Trim for IO pin 7"]
    #[inline(always)]
    pub fn ds_trim7(&self) -> DS_TRIM7_R {
        DS_TRIM7_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the Drive Select Trim for IO pin 0 0 - Default (50ohms) 1 - 120ohms 2 - 90ohms 3 - 60ohms 4 - 50ohms 5 - 30ohms 6 - 20ohms 7 - 15ohms"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim0(&mut self) -> DS_TRIM0_W<0> {
        DS_TRIM0_W::new(self)
    }
    #[doc = "Bits 3:5 - Sets the Drive Select Trim for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim1(&mut self) -> DS_TRIM1_W<3> {
        DS_TRIM1_W::new(self)
    }
    #[doc = "Bits 6:8 - Sets the Drive Select Trim for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim2(&mut self) -> DS_TRIM2_W<6> {
        DS_TRIM2_W::new(self)
    }
    #[doc = "Bits 9:11 - Sets the Drive Select Trim for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim3(&mut self) -> DS_TRIM3_W<9> {
        DS_TRIM3_W::new(self)
    }
    #[doc = "Bits 12:14 - Sets the Drive Select Trim for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim4(&mut self) -> DS_TRIM4_W<12> {
        DS_TRIM4_W::new(self)
    }
    #[doc = "Bits 15:17 - Sets the Drive Select Trim for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim5(&mut self) -> DS_TRIM5_W<15> {
        DS_TRIM5_W::new(self)
    }
    #[doc = "Bits 18:20 - Sets the Drive Select Trim for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim6(&mut self) -> DS_TRIM6_W<18> {
        DS_TRIM6_W::new(self)
    }
    #[doc = "Bits 21:23 - Sets the Drive Select Trim for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn ds_trim7(&mut self) -> DS_TRIM7_W<21> {
        DS_TRIM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output buffer configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_out2](index.html) module"]
pub struct CFG_OUT2_SPEC;
impl crate::RegisterSpec for CFG_OUT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_out2::R](R) reader structure"]
impl crate::Readable for CFG_OUT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_out2::W](W) writer structure"]
impl crate::Writable for CFG_OUT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_OUT2 to value 0"]
impl crate::Resettable for CFG_OUT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
