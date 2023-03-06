#[doc = "Register `BIST_CTL` reader"]
pub struct R(crate::R<BIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CTL` writer"]
pub struct W(crate::W<BIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CTL_SPEC>;
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
impl From<crate::W<BIST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_ENABLE` reader - Enable BIST in the desired conifguration and choose the channle to test"]
pub type TEST_ENABLE_R = crate::FieldReader<u8, TEST_ENABLE_A>;
#[doc = "Enable BIST in the desired conifguration and choose the channle to test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEST_ENABLE_A {
    #[doc = "0: Functional mode. All tests disabled"]
    TEST_DISABLED = 0,
    #[doc = "1: Channel A TX-serializer is enabled and TX data is looped back to the RX deserializer. The TX serializer sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate."]
    CH_A_SER_DES = 1,
    #[doc = "2: This test is the same as TEST_ENBL #00001.Channel B is tested in this test"]
    CH_B_SER_DES = 2,
    #[doc = "3: This test is the same as TEST_ENBL #00001.Channel C is tested in this test."]
    CH_C_SER_DES = 3,
    #[doc = "4: This test is the same as TEST_ENBL #00001.Channel D is tested in this test."]
    CH_D_SER_DES = 4,
    #[doc = "5: This test is the same as TEST_ENBL #00001.Channel CLK is tested in this test."]
    CH_CLK_SER_DES = 5,
    #[doc = "17: Channel A TX-serializer is enabled and TX data is looped back to the RX deserializer. The TX serializer sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate.This test will run in a two-phase fashion:Phase1: DESER_POL is set to 1'b0 (Default) Phase2: DESER_POL is set to 1'b1"]
    CH_A_SER_DES_2PH = 17,
    #[doc = "18: This test is the same as TEST_ENBL #10001.Channel B is tested in this test."]
    CH_B_SER_DES_2PH = 18,
    #[doc = "19: This test is the same as TEST_ENBL #10001.Channel C is tested in this test."]
    CH_C_SER_DES_2PH = 19,
    #[doc = "20: This test is the same as TEST_ENBL #10001.Channel D is tested in this test."]
    CH_D_SER_DES_2PH = 20,
    #[doc = "21: This test is the same as TEST_ENBL #10001.Channel CLK is tested in this test."]
    CH_CLK_SER_DES_2PH = 21,
    #[doc = "6: Channel A TX is enabled, and TX data is looped back from HSTX to HSRX. The TX sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate."]
    CH_A_SER_TX_RX_DES = 6,
    #[doc = "7: This test is the same as TEST_ENBL #00110.Channel B is tested in this test."]
    CH_B_TX_RX_SER_DES = 7,
    #[doc = "8: This test is the same as TEST_ENBL #00110.Channel C is tested in this test."]
    CH_C_TX_RX_SER_DES = 8,
    #[doc = "9: This test is the same as TEST_ENBL #00110.Channel D is tested in this test."]
    CH_D_TX_RX_SER_DES = 9,
    #[doc = "10: This test is the same as TEST_ENBL #00110.Channel CLK is tested in this test."]
    CH_CLK_SER_TX_RX_DES = 10,
    #[doc = "22: Channel A TX is enabled, and TX data is looped back from HSTX to HSRX. The TX sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compare it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate. This test will run in a two-phase fashion:Phase1: DESER_POL is set to 1'b0 (Default)Phase2: DESER_POL is set to 1'b1"]
    CH_A_SER_TX_RX_DES_2PH = 22,
    #[doc = "23: This test is the same as TEST_ENBL #10110.Channel B is tested in this test."]
    CH_B_SER_TX_RX_DES_2PH = 23,
    #[doc = "24: This test is the same as TEST_ENBL #10110.Channel C is tested in this test."]
    CH_C_SER_TX_RX_DES_2PH = 24,
    #[doc = "25: This test is the same as TEST_ENBL #10110.Channel D is tested in this test."]
    CH_D_SER_TX_RX_DES_2PH = 25,
    #[doc = "26: This test is the same as TEST_ENBL #10110.Channel CLK is tested in this test."]
    CH_CLK_SER_TX_RX_DES_2PH = 26,
}
impl From<TEST_ENABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: TEST_ENABLE_A) -> Self {
        variant as _
    }
}
impl TEST_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEST_ENABLE_A> {
        match self.bits {
            0 => Some(TEST_ENABLE_A::TEST_DISABLED),
            1 => Some(TEST_ENABLE_A::CH_A_SER_DES),
            2 => Some(TEST_ENABLE_A::CH_B_SER_DES),
            3 => Some(TEST_ENABLE_A::CH_C_SER_DES),
            4 => Some(TEST_ENABLE_A::CH_D_SER_DES),
            5 => Some(TEST_ENABLE_A::CH_CLK_SER_DES),
            17 => Some(TEST_ENABLE_A::CH_A_SER_DES_2PH),
            18 => Some(TEST_ENABLE_A::CH_B_SER_DES_2PH),
            19 => Some(TEST_ENABLE_A::CH_C_SER_DES_2PH),
            20 => Some(TEST_ENABLE_A::CH_D_SER_DES_2PH),
            21 => Some(TEST_ENABLE_A::CH_CLK_SER_DES_2PH),
            6 => Some(TEST_ENABLE_A::CH_A_SER_TX_RX_DES),
            7 => Some(TEST_ENABLE_A::CH_B_TX_RX_SER_DES),
            8 => Some(TEST_ENABLE_A::CH_C_TX_RX_SER_DES),
            9 => Some(TEST_ENABLE_A::CH_D_TX_RX_SER_DES),
            10 => Some(TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES),
            22 => Some(TEST_ENABLE_A::CH_A_SER_TX_RX_DES_2PH),
            23 => Some(TEST_ENABLE_A::CH_B_SER_TX_RX_DES_2PH),
            24 => Some(TEST_ENABLE_A::CH_C_SER_TX_RX_DES_2PH),
            25 => Some(TEST_ENABLE_A::CH_D_SER_TX_RX_DES_2PH),
            26 => Some(TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES_2PH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TEST_DISABLED`"]
    #[inline(always)]
    pub fn is_test_disabled(&self) -> bool {
        *self == TEST_ENABLE_A::TEST_DISABLED
    }
    #[doc = "Checks if the value of the field is `CH_A_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_a_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_A_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_B_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_b_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_B_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_C_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_c_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_C_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_D_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_d_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_D_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_CLK_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_clk_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_CLK_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_A_SER_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_a_ser_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_A_SER_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_B_SER_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_b_ser_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_B_SER_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_C_SER_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_c_ser_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_C_SER_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_D_SER_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_d_ser_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_D_SER_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_CLK_SER_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_clk_ser_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_CLK_SER_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_A_SER_TX_RX_DES`"]
    #[inline(always)]
    pub fn is_ch_a_ser_tx_rx_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_A_SER_TX_RX_DES
    }
    #[doc = "Checks if the value of the field is `CH_B_TX_RX_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_b_tx_rx_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_B_TX_RX_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_C_TX_RX_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_c_tx_rx_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_C_TX_RX_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_D_TX_RX_SER_DES`"]
    #[inline(always)]
    pub fn is_ch_d_tx_rx_ser_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_D_TX_RX_SER_DES
    }
    #[doc = "Checks if the value of the field is `CH_CLK_SER_TX_RX_DES`"]
    #[inline(always)]
    pub fn is_ch_clk_ser_tx_rx_des(&self) -> bool {
        *self == TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES
    }
    #[doc = "Checks if the value of the field is `CH_A_SER_TX_RX_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_a_ser_tx_rx_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_A_SER_TX_RX_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_B_SER_TX_RX_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_b_ser_tx_rx_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_B_SER_TX_RX_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_C_SER_TX_RX_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_c_ser_tx_rx_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_C_SER_TX_RX_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_D_SER_TX_RX_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_d_ser_tx_rx_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_D_SER_TX_RX_DES_2PH
    }
    #[doc = "Checks if the value of the field is `CH_CLK_SER_TX_RX_DES_2PH`"]
    #[inline(always)]
    pub fn is_ch_clk_ser_tx_rx_des_2ph(&self) -> bool {
        *self == TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES_2PH
    }
}
#[doc = "Field `TEST_ENABLE` writer - Enable BIST in the desired conifguration and choose the channle to test"]
pub type TEST_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIST_CTL_SPEC, u8, TEST_ENABLE_A, 5, O>;
impl<'a, const O: u8> TEST_ENABLE_W<'a, O> {
    #[doc = "Functional mode. All tests disabled"]
    #[inline(always)]
    pub fn test_disabled(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::TEST_DISABLED)
    }
    #[doc = "Channel A TX-serializer is enabled and TX data is looped back to the RX deserializer. The TX serializer sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate."]
    #[inline(always)]
    pub fn ch_a_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_A_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00001.Channel B is tested in this test"]
    #[inline(always)]
    pub fn ch_b_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_B_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00001.Channel C is tested in this test."]
    #[inline(always)]
    pub fn ch_c_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_C_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00001.Channel D is tested in this test."]
    #[inline(always)]
    pub fn ch_d_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_D_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00001.Channel CLK is tested in this test."]
    #[inline(always)]
    pub fn ch_clk_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_CLK_SER_DES)
    }
    #[doc = "Channel A TX-serializer is enabled and TX data is looped back to the RX deserializer. The TX serializer sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate.This test will run in a two-phase fashion:Phase1: DESER_POL is set to 1'b0 (Default) Phase2: DESER_POL is set to 1'b1"]
    #[inline(always)]
    pub fn ch_a_ser_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_A_SER_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10001.Channel B is tested in this test."]
    #[inline(always)]
    pub fn ch_b_ser_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_B_SER_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10001.Channel C is tested in this test."]
    #[inline(always)]
    pub fn ch_c_ser_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_C_SER_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10001.Channel D is tested in this test."]
    #[inline(always)]
    pub fn ch_d_ser_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_D_SER_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10001.Channel CLK is tested in this test."]
    #[inline(always)]
    pub fn ch_clk_ser_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_CLK_SER_DES_2PH)
    }
    #[doc = "Channel A TX is enabled, and TX data is looped back from HSTX to HSRX. The TX sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compares it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate."]
    #[inline(always)]
    pub fn ch_a_ser_tx_rx_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_A_SER_TX_RX_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00110.Channel B is tested in this test."]
    #[inline(always)]
    pub fn ch_b_tx_rx_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_B_TX_RX_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00110.Channel C is tested in this test."]
    #[inline(always)]
    pub fn ch_c_tx_rx_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_C_TX_RX_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00110.Channel D is tested in this test."]
    #[inline(always)]
    pub fn ch_d_tx_rx_ser_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_D_TX_RX_SER_DES)
    }
    #[doc = "This test is the same as TEST_ENBL #00110.Channel CLK is tested in this test."]
    #[inline(always)]
    pub fn ch_clk_ser_tx_rx_des(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES)
    }
    #[doc = "Channel A TX is enabled, and TX data is looped back from HSTX to HSRX. The TX sends the test data according to TEST_PATTERN bus. The pattern matcher will receive the HS data and compare it with the programmed TEST_PATTERN bus. The error counter is enabled to check word error rate. This test will run in a two-phase fashion:Phase1: DESER_POL is set to 1'b0 (Default)Phase2: DESER_POL is set to 1'b1"]
    #[inline(always)]
    pub fn ch_a_ser_tx_rx_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_A_SER_TX_RX_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10110.Channel B is tested in this test."]
    #[inline(always)]
    pub fn ch_b_ser_tx_rx_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_B_SER_TX_RX_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10110.Channel C is tested in this test."]
    #[inline(always)]
    pub fn ch_c_ser_tx_rx_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_C_SER_TX_RX_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10110.Channel D is tested in this test."]
    #[inline(always)]
    pub fn ch_d_ser_tx_rx_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_D_SER_TX_RX_DES_2PH)
    }
    #[doc = "This test is the same as TEST_ENBL #10110.Channel CLK is tested in this test."]
    #[inline(always)]
    pub fn ch_clk_ser_tx_rx_des_2ph(self) -> &'a mut W {
        self.variant(TEST_ENABLE_A::CH_CLK_SER_TX_RX_DES_2PH)
    }
}
impl R {
    #[doc = "Bits 0:4 - Enable BIST in the desired conifguration and choose the channle to test"]
    #[inline(always)]
    pub fn test_enable(&self) -> TEST_ENABLE_R {
        TEST_ENABLE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Enable BIST in the desired conifguration and choose the channle to test"]
    #[inline(always)]
    #[must_use]
    pub fn test_enable(&mut self) -> TEST_ENABLE_W<0> {
        TEST_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctl](index.html) module"]
pub struct BIST_CTL_SPEC;
impl crate::RegisterSpec for BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_ctl::R](R) reader structure"]
impl crate::Readable for BIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_ctl::W](W) writer structure"]
impl crate::Writable for BIST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIST_CTL to value 0"]
impl crate::Resettable for BIST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
