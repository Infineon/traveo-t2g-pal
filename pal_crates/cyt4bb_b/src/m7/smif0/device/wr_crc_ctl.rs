#[doc = "Register `WR_CRC_CTL` reader"]
pub struct R(crate::R<WR_CRC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_CRC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_CRC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_CRC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_CRC_CTL` writer"]
pub struct W(crate::W<WR_CRC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_CRC_CTL_SPEC>;
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
impl From<crate::W<WR_CRC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_CRC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CRC_INPUT_SIZE` reader - Number of input data bytes for CRC generation (minus 1), i.e. number of data bytes over which the data CRC field is generated (minus 1): '0': 1 byte ... '255': 256 bytes. Note: For octal data transfer with DDR mode (WR_DATA_CTL.WIDTH='3' and WR_DATA_CTL.DDR_MODE='1') the number of bytes for CRC generation must be even (i.e. DATA_CRC_INPUT_SIZE must be odd)."]
pub type DATA_CRC_INPUT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_CRC_INPUT_SIZE` writer - Number of input data bytes for CRC generation (minus 1), i.e. number of data bytes over which the data CRC field is generated (minus 1): '0': 1 byte ... '255': 256 bytes. Note: For octal data transfer with DDR mode (WR_DATA_CTL.WIDTH='3' and WR_DATA_CTL.DDR_MODE='1') the number of bytes for CRC generation must be even (i.e. DATA_CRC_INPUT_SIZE must be odd)."]
pub type DATA_CRC_INPUT_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WR_CRC_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMD_ADDR_CRC_WIDTH` reader - Width of command / address CRC field."]
pub type CMD_ADDR_CRC_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_ADDR_CRC_WIDTH` writer - Width of command / address CRC field."]
pub type CMD_ADDR_CRC_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WR_CRC_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CMD_ADDR_CRC_DDR_MODE` reader - Mode of transfer rate of command / address CRC field."]
pub type CMD_ADDR_CRC_DDR_MODE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_ADDR_CRC_DDR_MODE` writer - Mode of transfer rate of command / address CRC field."]
pub type CMD_ADDR_CRC_DDR_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WR_CRC_CTL_SPEC, bool, O>;
#[doc = "Field `CMD_ADDR_CRC_INPUT` reader - Specifies which fields are included in the command / address CRC generation. '0': The command / address CRC field is generated over the address and (if present) mode fields only. '1': The command / address CRC field is generated over the command, address and (if present) mode fields."]
pub type CMD_ADDR_CRC_INPUT_R = crate::BitReader<bool>;
#[doc = "Field `CMD_ADDR_CRC_INPUT` writer - Specifies which fields are included in the command / address CRC generation. '0': The command / address CRC field is generated over the address and (if present) mode fields only. '1': The command / address CRC field is generated over the command, address and (if present) mode fields."]
pub type CMD_ADDR_CRC_INPUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WR_CRC_CTL_SPEC, bool, O>;
#[doc = "Field `CMD_ADDR_CRC_PRESENT` reader - Presence of command / address CRC field: '0': not present '1': present Note: For octal data transfer with DDR mode (RD_CRC_CTL.WIDTH='3' and RD_CRC_CTL.DDR_MODE='1') the command / address CRC byte is sent twice, otherwise the command / address CRC byte is only sent once."]
pub type CMD_ADDR_CRC_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `CMD_ADDR_CRC_PRESENT` writer - Presence of command / address CRC field: '0': not present '1': present Note: For octal data transfer with DDR mode (RD_CRC_CTL.WIDTH='3' and RD_CRC_CTL.DDR_MODE='1') the command / address CRC byte is sent twice, otherwise the command / address CRC byte is only sent once."]
pub type CMD_ADDR_CRC_PRESENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WR_CRC_CTL_SPEC, bool, O>;
#[doc = "Field `DATA_CRC_PRESENT` reader - Presence of data CRC field: '0': not present '1': present Note: Width and data transfer mode (SDR or DDR) of read data CRC fields are the same as for the associated read data fields, i.e. are specified by RD_DATA_CTL.WIDTH and RD_DATA_CTL.DDR_MODE. Note: For octal data transfer with DDR mode (RD_DATA_CTL.WIDTH='3' and RD_DATA_CTL.DDR_MODE='1') the data CRC byte is sent twice, otherwise the data CRC byte is only sent once."]
pub type DATA_CRC_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `DATA_CRC_PRESENT` writer - Presence of data CRC field: '0': not present '1': present Note: Width and data transfer mode (SDR or DDR) of read data CRC fields are the same as for the associated read data fields, i.e. are specified by RD_DATA_CTL.WIDTH and RD_DATA_CTL.DDR_MODE. Note: For octal data transfer with DDR mode (RD_DATA_CTL.WIDTH='3' and RD_DATA_CTL.DDR_MODE='1') the data CRC byte is sent twice, otherwise the data CRC byte is only sent once."]
pub type DATA_CRC_PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR_CRC_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:23 - Number of input data bytes for CRC generation (minus 1), i.e. number of data bytes over which the data CRC field is generated (minus 1): '0': 1 byte ... '255': 256 bytes. Note: For octal data transfer with DDR mode (WR_DATA_CTL.WIDTH='3' and WR_DATA_CTL.DDR_MODE='1') the number of bytes for CRC generation must be even (i.e. DATA_CRC_INPUT_SIZE must be odd)."]
    #[inline(always)]
    pub fn data_crc_input_size(&self) -> DATA_CRC_INPUT_SIZE_R {
        DATA_CRC_INPUT_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Width of command / address CRC field."]
    #[inline(always)]
    pub fn cmd_addr_crc_width(&self) -> CMD_ADDR_CRC_WIDTH_R {
        CMD_ADDR_CRC_WIDTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Mode of transfer rate of command / address CRC field."]
    #[inline(always)]
    pub fn cmd_addr_crc_ddr_mode(&self) -> CMD_ADDR_CRC_DDR_MODE_R {
        CMD_ADDR_CRC_DDR_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Specifies which fields are included in the command / address CRC generation. '0': The command / address CRC field is generated over the address and (if present) mode fields only. '1': The command / address CRC field is generated over the command, address and (if present) mode fields."]
    #[inline(always)]
    pub fn cmd_addr_crc_input(&self) -> CMD_ADDR_CRC_INPUT_R {
        CMD_ADDR_CRC_INPUT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Presence of command / address CRC field: '0': not present '1': present Note: For octal data transfer with DDR mode (RD_CRC_CTL.WIDTH='3' and RD_CRC_CTL.DDR_MODE='1') the command / address CRC byte is sent twice, otherwise the command / address CRC byte is only sent once."]
    #[inline(always)]
    pub fn cmd_addr_crc_present(&self) -> CMD_ADDR_CRC_PRESENT_R {
        CMD_ADDR_CRC_PRESENT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Presence of data CRC field: '0': not present '1': present Note: Width and data transfer mode (SDR or DDR) of read data CRC fields are the same as for the associated read data fields, i.e. are specified by RD_DATA_CTL.WIDTH and RD_DATA_CTL.DDR_MODE. Note: For octal data transfer with DDR mode (RD_DATA_CTL.WIDTH='3' and RD_DATA_CTL.DDR_MODE='1') the data CRC byte is sent twice, otherwise the data CRC byte is only sent once."]
    #[inline(always)]
    pub fn data_crc_present(&self) -> DATA_CRC_PRESENT_R {
        DATA_CRC_PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Number of input data bytes for CRC generation (minus 1), i.e. number of data bytes over which the data CRC field is generated (minus 1): '0': 1 byte ... '255': 256 bytes. Note: For octal data transfer with DDR mode (WR_DATA_CTL.WIDTH='3' and WR_DATA_CTL.DDR_MODE='1') the number of bytes for CRC generation must be even (i.e. DATA_CRC_INPUT_SIZE must be odd)."]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_input_size(&mut self) -> DATA_CRC_INPUT_SIZE_W<16> {
        DATA_CRC_INPUT_SIZE_W::new(self)
    }
    #[doc = "Bits 24:25 - Width of command / address CRC field."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_addr_crc_width(&mut self) -> CMD_ADDR_CRC_WIDTH_W<24> {
        CMD_ADDR_CRC_WIDTH_W::new(self)
    }
    #[doc = "Bit 26 - Mode of transfer rate of command / address CRC field."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_addr_crc_ddr_mode(&mut self) -> CMD_ADDR_CRC_DDR_MODE_W<26> {
        CMD_ADDR_CRC_DDR_MODE_W::new(self)
    }
    #[doc = "Bit 27 - Specifies which fields are included in the command / address CRC generation. '0': The command / address CRC field is generated over the address and (if present) mode fields only. '1': The command / address CRC field is generated over the command, address and (if present) mode fields."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_addr_crc_input(&mut self) -> CMD_ADDR_CRC_INPUT_W<27> {
        CMD_ADDR_CRC_INPUT_W::new(self)
    }
    #[doc = "Bit 30 - Presence of command / address CRC field: '0': not present '1': present Note: For octal data transfer with DDR mode (RD_CRC_CTL.WIDTH='3' and RD_CRC_CTL.DDR_MODE='1') the command / address CRC byte is sent twice, otherwise the command / address CRC byte is only sent once."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_addr_crc_present(&mut self) -> CMD_ADDR_CRC_PRESENT_W<30> {
        CMD_ADDR_CRC_PRESENT_W::new(self)
    }
    #[doc = "Bit 31 - Presence of data CRC field: '0': not present '1': present Note: Width and data transfer mode (SDR or DDR) of read data CRC fields are the same as for the associated read data fields, i.e. are specified by RD_DATA_CTL.WIDTH and RD_DATA_CTL.DDR_MODE. Note: For octal data transfer with DDR mode (RD_DATA_CTL.WIDTH='3' and RD_DATA_CTL.DDR_MODE='1') the data CRC byte is sent twice, otherwise the data CRC byte is only sent once."]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_present(&mut self) -> DATA_CRC_PRESENT_W<31> {
        DATA_CRC_PRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Bus CRC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_crc_ctl](index.html) module"]
pub struct WR_CRC_CTL_SPEC;
impl crate::RegisterSpec for WR_CRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_crc_ctl::R](R) reader structure"]
impl crate::Readable for WR_CRC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_crc_ctl::W](W) writer structure"]
impl crate::Writable for WR_CRC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_CRC_CTL to value 0"]
impl crate::Resettable for WR_CRC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
