#[doc = "Register `SRC_CTL` reader"]
pub struct R(crate::R<SRC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_CTL` writer"]
pub struct W(crate::W<SRC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_CTL_SPEC>;
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
impl From<crate::W<SRC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS_RATIO` reader - Sample frequency upscale ratio: '0': 0.5 (downsampling by 2x). '1': 1 (no resampling). '2': 2 (upsampling by 2x). '3': 3. '4': 4. '5': 6. '6': 8. '7': 12 (upsampling by 12x)."]
pub type FS_RATIO_R = crate::FieldReader<u8, FS_RATIO_A>;
#[doc = "Sample frequency upscale ratio: '0': 0.5 (downsampling by 2x). '1': 1 (no resampling). '2': 2 (upsampling by 2x). '3': 3. '4': 4. '5': 6. '6': 8. '7': 12 (upsampling by 12x).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS_RATIO_A {
    #[doc = "0: N/A"]
    DOWN_2 = 0,
    #[doc = "1: N/A"]
    SAME = 1,
    #[doc = "2: N/A"]
    UP_2 = 2,
    #[doc = "3: N/A"]
    UP_3 = 3,
    #[doc = "4: N/A"]
    UP_4 = 4,
    #[doc = "5: N/A"]
    UP_6 = 5,
    #[doc = "6: N/A"]
    UP_8 = 6,
    #[doc = "7: N/A"]
    UP_12 = 7,
}
impl From<FS_RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_RATIO_A) -> Self {
        variant as _
    }
}
impl FS_RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_RATIO_A {
        match self.bits {
            0 => FS_RATIO_A::DOWN_2,
            1 => FS_RATIO_A::SAME,
            2 => FS_RATIO_A::UP_2,
            3 => FS_RATIO_A::UP_3,
            4 => FS_RATIO_A::UP_4,
            5 => FS_RATIO_A::UP_6,
            6 => FS_RATIO_A::UP_8,
            7 => FS_RATIO_A::UP_12,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DOWN_2`"]
    #[inline(always)]
    pub fn is_down_2(&self) -> bool {
        *self == FS_RATIO_A::DOWN_2
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == FS_RATIO_A::SAME
    }
    #[doc = "Checks if the value of the field is `UP_2`"]
    #[inline(always)]
    pub fn is_up_2(&self) -> bool {
        *self == FS_RATIO_A::UP_2
    }
    #[doc = "Checks if the value of the field is `UP_3`"]
    #[inline(always)]
    pub fn is_up_3(&self) -> bool {
        *self == FS_RATIO_A::UP_3
    }
    #[doc = "Checks if the value of the field is `UP_4`"]
    #[inline(always)]
    pub fn is_up_4(&self) -> bool {
        *self == FS_RATIO_A::UP_4
    }
    #[doc = "Checks if the value of the field is `UP_6`"]
    #[inline(always)]
    pub fn is_up_6(&self) -> bool {
        *self == FS_RATIO_A::UP_6
    }
    #[doc = "Checks if the value of the field is `UP_8`"]
    #[inline(always)]
    pub fn is_up_8(&self) -> bool {
        *self == FS_RATIO_A::UP_8
    }
    #[doc = "Checks if the value of the field is `UP_12`"]
    #[inline(always)]
    pub fn is_up_12(&self) -> bool {
        *self == FS_RATIO_A::UP_12
    }
}
#[doc = "Field `FS_RATIO` writer - Sample frequency upscale ratio: '0': 0.5 (downsampling by 2x). '1': 1 (no resampling). '2': 2 (upsampling by 2x). '3': 3. '4': 4. '5': 6. '6': 8. '7': 12 (upsampling by 12x)."]
pub type FS_RATIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SRC_CTL_SPEC, u8, FS_RATIO_A, 3, O>;
impl<'a, const O: u8> FS_RATIO_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn down_2(self) -> &'a mut W {
        self.variant(FS_RATIO_A::DOWN_2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(FS_RATIO_A::SAME)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_2(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_3(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_3)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_4(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_4)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_6(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_8(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn up_12(self) -> &'a mut W {
        self.variant(FS_RATIO_A::UP_12)
    }
}
#[doc = "Field `CH0_SEL` reader - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]. '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined. Note: CH0_SEL and CH1_SEL are used to specify mono and stereo audio formats."]
pub type CH0_SEL_R = crate::FieldReader<u8, CH0_SEL_A>;
#[doc = "Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]. '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined. Note: CH0_SEL and CH1_SEL are used to specify mono and stereo audio formats.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0_SEL_A {
    #[doc = "0: N/A"]
    DATA_LOW = 0,
    #[doc = "1: N/A"]
    DATA_HIGH = 1,
    #[doc = "2: N/A"]
    CONSTANT_0 = 2,
}
impl From<CH0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0_SEL_A) -> Self {
        variant as _
    }
}
impl CH0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH0_SEL_A> {
        match self.bits {
            0 => Some(CH0_SEL_A::DATA_LOW),
            1 => Some(CH0_SEL_A::DATA_HIGH),
            2 => Some(CH0_SEL_A::CONSTANT_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_LOW`"]
    #[inline(always)]
    pub fn is_data_low(&self) -> bool {
        *self == CH0_SEL_A::DATA_LOW
    }
    #[doc = "Checks if the value of the field is `DATA_HIGH`"]
    #[inline(always)]
    pub fn is_data_high(&self) -> bool {
        *self == CH0_SEL_A::DATA_HIGH
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0`"]
    #[inline(always)]
    pub fn is_constant_0(&self) -> bool {
        *self == CH0_SEL_A::CONSTANT_0
    }
}
#[doc = "Field `CH0_SEL` writer - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]. '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined. Note: CH0_SEL and CH1_SEL are used to specify mono and stereo audio formats."]
pub type CH0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRC_CTL_SPEC, u8, CH0_SEL_A, 2, O>;
impl<'a, const O: u8> CH0_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_low(self) -> &'a mut W {
        self.variant(CH0_SEL_A::DATA_LOW)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_high(self) -> &'a mut W {
        self.variant(CH0_SEL_A::DATA_HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn constant_0(self) -> &'a mut W {
        self.variant(CH0_SEL_A::CONSTANT_0)
    }
}
#[doc = "Field `CH1_SEL` reader - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined."]
pub type CH1_SEL_R = crate::FieldReader<u8, CH1_SEL_A>;
#[doc = "Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH1_SEL_A {
    #[doc = "0: N/A"]
    DATA_LOW = 0,
    #[doc = "1: N/A"]
    DATA_HIGH = 1,
    #[doc = "2: N/A"]
    CONSTANT_0 = 2,
}
impl From<CH1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CH1_SEL_A) -> Self {
        variant as _
    }
}
impl CH1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH1_SEL_A> {
        match self.bits {
            0 => Some(CH1_SEL_A::DATA_LOW),
            1 => Some(CH1_SEL_A::DATA_HIGH),
            2 => Some(CH1_SEL_A::CONSTANT_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_LOW`"]
    #[inline(always)]
    pub fn is_data_low(&self) -> bool {
        *self == CH1_SEL_A::DATA_LOW
    }
    #[doc = "Checks if the value of the field is `DATA_HIGH`"]
    #[inline(always)]
    pub fn is_data_high(&self) -> bool {
        *self == CH1_SEL_A::DATA_HIGH
    }
    #[doc = "Checks if the value of the field is `CONSTANT_0`"]
    #[inline(always)]
    pub fn is_constant_0(&self) -> bool {
        *self == CH1_SEL_A::CONSTANT_0
    }
}
#[doc = "Field `CH1_SEL` writer - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined."]
pub type CH1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRC_CTL_SPEC, u8, CH1_SEL_A, 2, O>;
impl<'a, const O: u8> CH1_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_low(self) -> &'a mut W {
        self.variant(CH1_SEL_A::DATA_LOW)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_high(self) -> &'a mut W {
        self.variant(CH1_SEL_A::DATA_HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn constant_0(self) -> &'a mut W {
        self.variant(CH1_SEL_A::CONSTANT_0)
    }
}
#[doc = "Field `ENABLED` reader - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. the SRC_STATUS, SRC_FIFO_STATUS and INTR_SRC registers) have their fields reset to their default value. '1': Enabled. Note: when all sources, destination and transmitter are disabled, the SRAMs are driven into low power mode, if supported by the SRAM. When exiting such low power mode software needs to allow for a certain power up time before SRAM can be used, i.e. before ACTIVE can be asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register (or equivalent for platforms other than MXS40)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. the SRC_STATUS, SRC_FIFO_STATUS and INTR_SRC registers) have their fields reset to their default value. '1': Enabled. Note: when all sources, destination and transmitter are disabled, the SRAMs are driven into low power mode, if supported by the SRAM. When exiting such low power mode software needs to allow for a certain power up time before SRAM can be used, i.e. before ACTIVE can be asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register (or equivalent for platforms other than MXS40)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 12:14 - Sample frequency upscale ratio: '0': 0.5 (downsampling by 2x). '1': 1 (no resampling). '2': 2 (upsampling by 2x). '3': 3. '4': 4. '5': 6. '6': 8. '7': 12 (upsampling by 12x)."]
    #[inline(always)]
    pub fn fs_ratio(&self) -> FS_RATIO_R {
        FS_RATIO_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]. '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined. Note: CH0_SEL and CH1_SEL are used to specify mono and stereo audio formats."]
    #[inline(always)]
    pub fn ch0_sel(&self) -> CH0_SEL_R {
        CH0_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined."]
    #[inline(always)]
    pub fn ch1_sel(&self) -> CH1_SEL_R {
        CH1_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 31 - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. the SRC_STATUS, SRC_FIFO_STATUS and INTR_SRC registers) have their fields reset to their default value. '1': Enabled. Note: when all sources, destination and transmitter are disabled, the SRAMs are driven into low power mode, if supported by the SRAM. When exiting such low power mode software needs to allow for a certain power up time before SRAM can be used, i.e. before ACTIVE can be asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register (or equivalent for platforms other than MXS40)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:14 - Sample frequency upscale ratio: '0': 0.5 (downsampling by 2x). '1': 1 (no resampling). '2': 2 (upsampling by 2x). '3': 3. '4': 4. '5': 6. '6': 8. '7': 12 (upsampling by 12x)."]
    #[inline(always)]
    #[must_use]
    pub fn fs_ratio(&mut self) -> FS_RATIO_W<12> {
        FS_RATIO_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]. '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined. Note: CH0_SEL and CH1_SEL are used to specify mono and stereo audio formats."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_sel(&mut self) -> CH0_SEL_W<16> {
        CH0_SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - Channel selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). '2': Constant PCM data value of '0'; i.e. source FIFO data is not used. '3': undefined."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_sel(&mut self) -> CH1_SEL_W<18> {
        CH1_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. the SRC_STATUS, SRC_FIFO_STATUS and INTR_SRC registers) have their fields reset to their default value. '1': Enabled. Note: when all sources, destination and transmitter are disabled, the SRAMs are driven into low power mode, if supported by the SRAM. When exiting such low power mode software needs to allow for a certain power up time before SRAM can be used, i.e. before ACTIVE can be asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register (or equivalent for platforms other than MXS40)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_ctl](index.html) module"]
pub struct SRC_CTL_SPEC;
impl crate::RegisterSpec for SRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_ctl::R](R) reader structure"]
impl crate::Readable for SRC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_ctl::W](W) writer structure"]
impl crate::Writable for SRC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_CTL to value 0x0004_1000"]
impl crate::Resettable for SRC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_1000;
}
