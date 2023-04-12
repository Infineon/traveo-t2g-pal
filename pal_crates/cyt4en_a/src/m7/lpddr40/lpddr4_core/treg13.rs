#[doc = "Register `TREG13` reader"]
pub struct R(crate::R<TREG13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG13` writer"]
pub struct W(crate::W<TREG13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG13_SPEC>;
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
impl From<crate::W<TREG13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_LVLDIS` reader - Leveling disable time, specifies the minimum number of DFI clock cycles after the PHY asserts training status signal to the de-assert of DTI_*LVL_EN. = 8"]
pub type T_LVLDIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLDIS` writer - Leveling disable time, specifies the minimum number of DFI clock cycles after the PHY asserts training status signal to the de-assert of DTI_*LVL_EN. = 8"]
pub type T_LVLDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG13_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_LVLEXIT` reader - Leveling exit time, specifies the minimum number of DFI clock cycles after the de-assertion of the DTI_*LVL_EN signal to the MRS command to disable DDR training. = 20"]
pub type T_LVLEXIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLEXIT` writer - Leveling exit time, specifies the minimum number of DFI clock cycles after the de-assertion of the DTI_*LVL_EN signal to the MRS command to disable DDR training. = 20"]
pub type T_LVLEXIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG13_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_LVLAA` reader - Leveling command-to-command/strobe-to-strobe delay. For read training, specifies the minimum number of DFI clock cycles after the assertion of a RD/MRR command to the next RD/MRR command. For write leveling, specifies the minimum number of DFI clock cycles after the assertion of the dfi_wrlvl_strobe signal to the next dfi_wrlvl_strobe signal assertion. = 8"]
pub type T_LVLAA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_LVLAA` writer - Leveling command-to-command/strobe-to-strobe delay. For read training, specifies the minimum number of DFI clock cycles after the assertion of a RD/MRR command to the next RD/MRR command. For write leveling, specifies the minimum number of DFI clock cycles after the assertion of the dfi_wrlvl_strobe signal to the next dfi_wrlvl_strobe signal assertion. = 8"]
pub type T_LVLAA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG13_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_FC` reader - Frequency Set Point Switching Time = 'ha0"]
pub type T_FC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_FC` writer - Frequency Set Point Switching Time = 'ha0"]
pub type T_FC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG13_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:4 - Leveling disable time, specifies the minimum number of DFI clock cycles after the PHY asserts training status signal to the de-assert of DTI_*LVL_EN. = 8"]
    #[inline(always)]
    pub fn t_lvldis(&self) -> T_LVLDIS_R {
        T_LVLDIS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Leveling exit time, specifies the minimum number of DFI clock cycles after the de-assertion of the DTI_*LVL_EN signal to the MRS command to disable DDR training. = 20"]
    #[inline(always)]
    pub fn t_lvlexit(&self) -> T_LVLEXIT_R {
        T_LVLEXIT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - Leveling command-to-command/strobe-to-strobe delay. For read training, specifies the minimum number of DFI clock cycles after the assertion of a RD/MRR command to the next RD/MRR command. For write leveling, specifies the minimum number of DFI clock cycles after the assertion of the dfi_wrlvl_strobe signal to the next dfi_wrlvl_strobe signal assertion. = 8"]
    #[inline(always)]
    pub fn t_lvlaa(&self) -> T_LVLAA_R {
        T_LVLAA_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:22 - Frequency Set Point Switching Time = 'ha0"]
    #[inline(always)]
    pub fn t_fc(&self) -> T_FC_R {
        T_FC_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Leveling disable time, specifies the minimum number of DFI clock cycles after the PHY asserts training status signal to the de-assert of DTI_*LVL_EN. = 8"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvldis(&mut self) -> T_LVLDIS_W<0> {
        T_LVLDIS_W::new(self)
    }
    #[doc = "Bits 5:9 - Leveling exit time, specifies the minimum number of DFI clock cycles after the de-assertion of the DTI_*LVL_EN signal to the MRS command to disable DDR training. = 20"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvlexit(&mut self) -> T_LVLEXIT_W<5> {
        T_LVLEXIT_W::new(self)
    }
    #[doc = "Bits 10:13 - Leveling command-to-command/strobe-to-strobe delay. For read training, specifies the minimum number of DFI clock cycles after the assertion of a RD/MRR command to the next RD/MRR command. For write leveling, specifies the minimum number of DFI clock cycles after the assertion of the dfi_wrlvl_strobe signal to the next dfi_wrlvl_strobe signal assertion. = 8"]
    #[inline(always)]
    #[must_use]
    pub fn t_lvlaa(&mut self) -> T_LVLAA_W<10> {
        T_LVLAA_W::new(self)
    }
    #[doc = "Bits 14:22 - Frequency Set Point Switching Time = 'ha0"]
    #[inline(always)]
    #[must_use]
    pub fn t_fc(&mut self) -> T_FC_W<14> {
        T_FC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg13](index.html) module"]
pub struct TREG13_SPEC;
impl crate::RegisterSpec for TREG13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg13::R](R) reader structure"]
impl crate::Readable for TREG13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg13::W](W) writer structure"]
impl crate::Writable for TREG13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG13 to value 0x0028_2210"]
impl crate::Resettable for TREG13_SPEC {
    const RESET_VALUE: Self::Ux = 0x0028_2210;
}
