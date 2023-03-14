#[doc = "Register `I2C_CFG` reader"]
pub struct R(crate::R<I2C_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CFG` writer"]
pub struct W(crate::W<I2C_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CFG_SPEC>;
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
impl From<crate::W<I2C_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_IN_FILT_TRIM` reader - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
pub type SDA_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_IN_FILT_TRIM` writer - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
pub type SDA_IN_FILT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDA_IN_FILT_SEL` reader - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SDA_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SDA_IN_FILT_SEL` writer - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SDA_IN_FILT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CFG_SPEC, bool, O>;
#[doc = "Field `SCL_IN_FILT_TRIM` reader - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
pub type SCL_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCL_IN_FILT_TRIM` writer - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
pub type SCL_IN_FILT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCL_IN_FILT_SEL` reader - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SCL_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SCL_IN_FILT_SEL` writer - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SCL_IN_FILT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CFG_SPEC, bool, O>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT0_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT1_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT1_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT2_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
pub type SDA_OUT_FILT2_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDA_OUT_FILT_SEL` reader - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SDA_OUT_FILT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT_SEL` writer - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SDA_OUT_FILT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W<0> {
        SDA_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 4 - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W<4> {
        SDA_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W<8> {
        SCL_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 12 - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W<12> {
        SCL_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W<16> {
        SDA_OUT_FILT0_TRIM_W::new(self)
    }
    #[doc = "Bits 18:19 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W<18> {
        SDA_OUT_FILT1_TRIM_W::new(self)
    }
    #[doc = "Bits 20:21 - Trim settings for the 50ns delay filter on SDA output used to guarantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W<20> {
        SDA_OUT_FILT2_TRIM_W::new(self)
    }
    #[doc = "Bits 28:29 - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt_sel(&mut self) -> SDA_OUT_FILT_SEL_W<28> {
        SDA_OUT_FILT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cfg](index.html) module"]
pub struct I2C_CFG_SPEC;
impl crate::RegisterSpec for I2C_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_cfg::R](R) reader structure"]
impl crate::Readable for I2C_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_cfg::W](W) writer structure"]
impl crate::Writable for I2C_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CFG to value 0x002a_1013"]
impl crate::Resettable for I2C_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x002a_1013;
}
