#[doc = "Register `RD_MODE_CTL` reader"]
pub struct R(crate::R<RD_MODE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MODE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MODE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MODE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_MODE_CTL` writer"]
pub struct W(crate::W<RD_MODE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_MODE_CTL_SPEC>;
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
impl From<crate::W<RD_MODE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_MODE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Mode byte code. Note: If a mode field is present (PRESENT='1') for octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') the CODE is sent twice."]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - Mode byte code. Note: If a mode field is present (PRESENT='1') for octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') the CODE is sent twice."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_MODE_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CODEH` reader - Mode high byte code. This field specifies the most significant mode byte, sent before the least significant mode byte (CODE). It is only used when PRESENT2 = '2'."]
pub type CODEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODEH` writer - Mode high byte code. This field specifies the most significant mode byte, sent before the least significant mode byte (CODE). It is only used when PRESENT2 = '2'."]
pub type CODEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_MODE_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_MODE_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DDR_MODE` reader - Mode of transfer rate."]
pub type DDR_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DDR_MODE` writer - Mode of transfer rate."]
pub type DDR_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD_MODE_CTL_SPEC, bool, O>;
#[doc = "Field `PRESENT2` reader - Presence of mode field: '0': not present '1': present (1 Byte) '2': present (2 Byte for OPI) Note: For octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') this field needs to be set to PRESENT2='2' to generate a mode field (or to PRESENT2='0')."]
pub type PRESENT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESENT2` writer - Presence of mode field: '0': not present '1': present (1 Byte) '2': present (2 Byte for OPI) Note: For octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') this field needs to be set to PRESENT2='2' to generate a mode field (or to PRESENT2='0')."]
pub type PRESENT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_MODE_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Mode byte code. Note: If a mode field is present (PRESENT='1') for octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') the CODE is sent twice."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mode high byte code. This field specifies the most significant mode byte, sent before the least significant mode byte (CODE). It is only used when PRESENT2 = '2'."]
    #[inline(always)]
    pub fn codeh(&self) -> CODEH_R {
        CODEH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode of transfer rate."]
    #[inline(always)]
    pub fn ddr_mode(&self) -> DDR_MODE_R {
        DDR_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Presence of mode field: '0': not present '1': present (1 Byte) '2': present (2 Byte for OPI) Note: For octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') this field needs to be set to PRESENT2='2' to generate a mode field (or to PRESENT2='0')."]
    #[inline(always)]
    pub fn present2(&self) -> PRESENT2_R {
        PRESENT2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode byte code. Note: If a mode field is present (PRESENT='1') for octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') the CODE is sent twice."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Mode high byte code. This field specifies the most significant mode byte, sent before the least significant mode byte (CODE). It is only used when PRESENT2 = '2'."]
    #[inline(always)]
    #[must_use]
    pub fn codeh(&mut self) -> CODEH_W<8> {
        CODEH_W::new(self)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 18 - Mode of transfer rate."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_mode(&mut self) -> DDR_MODE_W<18> {
        DDR_MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - Presence of mode field: '0': not present '1': present (1 Byte) '2': present (2 Byte for OPI) Note: For octal data transfer with DDR mode (WIDTH='3' and DDR_MODE='1') this field needs to be set to PRESENT2='2' to generate a mode field (or to PRESENT2='0')."]
    #[inline(always)]
    #[must_use]
    pub fn present2(&mut self) -> PRESENT2_W<30> {
        PRESENT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mode_ctl](index.html) module"]
pub struct RD_MODE_CTL_SPEC;
impl crate::RegisterSpec for RD_MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mode_ctl::R](R) reader structure"]
impl crate::Readable for RD_MODE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_mode_ctl::W](W) writer structure"]
impl crate::Writable for RD_MODE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_MODE_CTL to value 0"]
impl crate::Resettable for RD_MODE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
