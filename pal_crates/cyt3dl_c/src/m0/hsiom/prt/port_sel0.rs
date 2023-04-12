#[doc = "Register `PORT_SEL0` reader"]
pub struct R(crate::R<PORT_SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL0` writer"]
pub struct W(crate::W<PORT_SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL0_SPEC>;
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
impl From<crate::W<PORT_SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO0_SEL` reader - Selects the peripheral connections of Pin 0. Note that available connectivity options vary depending on the device, port and the pin. See the device Datasheet for a list of peripheral connections available at each pin."]
pub type IO0_SEL_R = crate::FieldReader<u8, IO0_SEL_A>;
#[doc = "Selects the peripheral connections of Pin 0. Note that available connectivity options vary depending on the device, port and the pin. See the device Datasheet for a list of peripheral connections available at each pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "0: GPIO controls 'out'"]
    GPIO = 0,
    #[doc = "1: GPIO controls 'out', DSI controls 'output enable'"]
    GPIO_DSI = 1,
    #[doc = "2: DSI controls 'out' and 'output enable'"]
    DSI_DSI = 2,
    #[doc = "3: DSI controls 'out', GPIO controls 'output enable'"]
    DSI_GPIO = 3,
    #[doc = "4: AMUXBUS A"]
    AMUXA = 4,
    #[doc = "5: AMUXBUS B"]
    AMUXB = 5,
    #[doc = "6: Analog mux bus A, DSI control"]
    AMUXA_DSI = 6,
    #[doc = "7: Analog mux bus B, DSI control"]
    AMUXB_DSI = 7,
    #[doc = "8: Active peripherals 0"]
    ACT_0 = 8,
    #[doc = "9: Active peripherals 1"]
    ACT_1 = 9,
    #[doc = "10: Active peripherals 2"]
    ACT_2 = 10,
    #[doc = "11: Active peripherals 4"]
    ACT_3 = 11,
    #[doc = "12: Deep Sleep peripherals 0"]
    DS_0 = 12,
    #[doc = "13: Deep Sleep peripherals 1"]
    DS_1 = 13,
    #[doc = "14: Deep Sleep peripherals 2"]
    DS_2 = 14,
    #[doc = "15: Deep Sleep peripherals 3"]
    DS_3 = 15,
    #[doc = "16: Active peripherals 4"]
    ACT_4 = 16,
    #[doc = "17: Active peripherals 5"]
    ACT_5 = 17,
    #[doc = "18: Active peripherals 6"]
    ACT_6 = 18,
    #[doc = "19: Active peripherals 7"]
    ACT_7 = 19,
    #[doc = "20: Active peripherals 8"]
    ACT_8 = 20,
    #[doc = "21: Active peripherals 9"]
    ACT_9 = 21,
    #[doc = "22: Active peripherals 10"]
    ACT_10 = 22,
    #[doc = "23: Active peripherals 11"]
    ACT_11 = 23,
    #[doc = "24: Active peripherals 12"]
    ACT_12 = 24,
    #[doc = "25: Active peripherals 13"]
    ACT_13 = 25,
    #[doc = "26: Active peripherals 14"]
    ACT_14 = 26,
    #[doc = "27: Active peripherals 15"]
    ACT_15 = 27,
    #[doc = "28: DeepSleep functionality 4"]
    DS_4 = 28,
    #[doc = "29: DeepSleep functionality 5"]
    DS_5 = 29,
    #[doc = "30: DeepSleep functionality 6"]
    DS_6 = 30,
    #[doc = "31: DeepSleep functionality 7"]
    DS_7 = 31,
}
impl From<IO0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_SEL_A) -> Self {
        variant as _
    }
}
impl IO0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_SEL_A {
        match self.bits {
            0 => IO0_SEL_A::GPIO,
            1 => IO0_SEL_A::GPIO_DSI,
            2 => IO0_SEL_A::DSI_DSI,
            3 => IO0_SEL_A::DSI_GPIO,
            4 => IO0_SEL_A::AMUXA,
            5 => IO0_SEL_A::AMUXB,
            6 => IO0_SEL_A::AMUXA_DSI,
            7 => IO0_SEL_A::AMUXB_DSI,
            8 => IO0_SEL_A::ACT_0,
            9 => IO0_SEL_A::ACT_1,
            10 => IO0_SEL_A::ACT_2,
            11 => IO0_SEL_A::ACT_3,
            12 => IO0_SEL_A::DS_0,
            13 => IO0_SEL_A::DS_1,
            14 => IO0_SEL_A::DS_2,
            15 => IO0_SEL_A::DS_3,
            16 => IO0_SEL_A::ACT_4,
            17 => IO0_SEL_A::ACT_5,
            18 => IO0_SEL_A::ACT_6,
            19 => IO0_SEL_A::ACT_7,
            20 => IO0_SEL_A::ACT_8,
            21 => IO0_SEL_A::ACT_9,
            22 => IO0_SEL_A::ACT_10,
            23 => IO0_SEL_A::ACT_11,
            24 => IO0_SEL_A::ACT_12,
            25 => IO0_SEL_A::ACT_13,
            26 => IO0_SEL_A::ACT_14,
            27 => IO0_SEL_A::ACT_15,
            28 => IO0_SEL_A::DS_4,
            29 => IO0_SEL_A::DS_5,
            30 => IO0_SEL_A::DS_6,
            31 => IO0_SEL_A::DS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == IO0_SEL_A::GPIO
    }
    #[doc = "Checks if the value of the field is `GPIO_DSI`"]
    #[inline(always)]
    pub fn is_gpio_dsi(&self) -> bool {
        *self == IO0_SEL_A::GPIO_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_DSI`"]
    #[inline(always)]
    pub fn is_dsi_dsi(&self) -> bool {
        *self == IO0_SEL_A::DSI_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_GPIO`"]
    #[inline(always)]
    pub fn is_dsi_gpio(&self) -> bool {
        *self == IO0_SEL_A::DSI_GPIO
    }
    #[doc = "Checks if the value of the field is `AMUXA`"]
    #[inline(always)]
    pub fn is_amuxa(&self) -> bool {
        *self == IO0_SEL_A::AMUXA
    }
    #[doc = "Checks if the value of the field is `AMUXB`"]
    #[inline(always)]
    pub fn is_amuxb(&self) -> bool {
        *self == IO0_SEL_A::AMUXB
    }
    #[doc = "Checks if the value of the field is `AMUXA_DSI`"]
    #[inline(always)]
    pub fn is_amuxa_dsi(&self) -> bool {
        *self == IO0_SEL_A::AMUXA_DSI
    }
    #[doc = "Checks if the value of the field is `AMUXB_DSI`"]
    #[inline(always)]
    pub fn is_amuxb_dsi(&self) -> bool {
        *self == IO0_SEL_A::AMUXB_DSI
    }
    #[doc = "Checks if the value of the field is `ACT_0`"]
    #[inline(always)]
    pub fn is_act_0(&self) -> bool {
        *self == IO0_SEL_A::ACT_0
    }
    #[doc = "Checks if the value of the field is `ACT_1`"]
    #[inline(always)]
    pub fn is_act_1(&self) -> bool {
        *self == IO0_SEL_A::ACT_1
    }
    #[doc = "Checks if the value of the field is `ACT_2`"]
    #[inline(always)]
    pub fn is_act_2(&self) -> bool {
        *self == IO0_SEL_A::ACT_2
    }
    #[doc = "Checks if the value of the field is `ACT_3`"]
    #[inline(always)]
    pub fn is_act_3(&self) -> bool {
        *self == IO0_SEL_A::ACT_3
    }
    #[doc = "Checks if the value of the field is `DS_0`"]
    #[inline(always)]
    pub fn is_ds_0(&self) -> bool {
        *self == IO0_SEL_A::DS_0
    }
    #[doc = "Checks if the value of the field is `DS_1`"]
    #[inline(always)]
    pub fn is_ds_1(&self) -> bool {
        *self == IO0_SEL_A::DS_1
    }
    #[doc = "Checks if the value of the field is `DS_2`"]
    #[inline(always)]
    pub fn is_ds_2(&self) -> bool {
        *self == IO0_SEL_A::DS_2
    }
    #[doc = "Checks if the value of the field is `DS_3`"]
    #[inline(always)]
    pub fn is_ds_3(&self) -> bool {
        *self == IO0_SEL_A::DS_3
    }
    #[doc = "Checks if the value of the field is `ACT_4`"]
    #[inline(always)]
    pub fn is_act_4(&self) -> bool {
        *self == IO0_SEL_A::ACT_4
    }
    #[doc = "Checks if the value of the field is `ACT_5`"]
    #[inline(always)]
    pub fn is_act_5(&self) -> bool {
        *self == IO0_SEL_A::ACT_5
    }
    #[doc = "Checks if the value of the field is `ACT_6`"]
    #[inline(always)]
    pub fn is_act_6(&self) -> bool {
        *self == IO0_SEL_A::ACT_6
    }
    #[doc = "Checks if the value of the field is `ACT_7`"]
    #[inline(always)]
    pub fn is_act_7(&self) -> bool {
        *self == IO0_SEL_A::ACT_7
    }
    #[doc = "Checks if the value of the field is `ACT_8`"]
    #[inline(always)]
    pub fn is_act_8(&self) -> bool {
        *self == IO0_SEL_A::ACT_8
    }
    #[doc = "Checks if the value of the field is `ACT_9`"]
    #[inline(always)]
    pub fn is_act_9(&self) -> bool {
        *self == IO0_SEL_A::ACT_9
    }
    #[doc = "Checks if the value of the field is `ACT_10`"]
    #[inline(always)]
    pub fn is_act_10(&self) -> bool {
        *self == IO0_SEL_A::ACT_10
    }
    #[doc = "Checks if the value of the field is `ACT_11`"]
    #[inline(always)]
    pub fn is_act_11(&self) -> bool {
        *self == IO0_SEL_A::ACT_11
    }
    #[doc = "Checks if the value of the field is `ACT_12`"]
    #[inline(always)]
    pub fn is_act_12(&self) -> bool {
        *self == IO0_SEL_A::ACT_12
    }
    #[doc = "Checks if the value of the field is `ACT_13`"]
    #[inline(always)]
    pub fn is_act_13(&self) -> bool {
        *self == IO0_SEL_A::ACT_13
    }
    #[doc = "Checks if the value of the field is `ACT_14`"]
    #[inline(always)]
    pub fn is_act_14(&self) -> bool {
        *self == IO0_SEL_A::ACT_14
    }
    #[doc = "Checks if the value of the field is `ACT_15`"]
    #[inline(always)]
    pub fn is_act_15(&self) -> bool {
        *self == IO0_SEL_A::ACT_15
    }
    #[doc = "Checks if the value of the field is `DS_4`"]
    #[inline(always)]
    pub fn is_ds_4(&self) -> bool {
        *self == IO0_SEL_A::DS_4
    }
    #[doc = "Checks if the value of the field is `DS_5`"]
    #[inline(always)]
    pub fn is_ds_5(&self) -> bool {
        *self == IO0_SEL_A::DS_5
    }
    #[doc = "Checks if the value of the field is `DS_6`"]
    #[inline(always)]
    pub fn is_ds_6(&self) -> bool {
        *self == IO0_SEL_A::DS_6
    }
    #[doc = "Checks if the value of the field is `DS_7`"]
    #[inline(always)]
    pub fn is_ds_7(&self) -> bool {
        *self == IO0_SEL_A::DS_7
    }
}
#[doc = "Field `IO0_SEL` writer - Selects the peripheral connections of Pin 0. Note that available connectivity options vary depending on the device, port and the pin. See the device Datasheet for a list of peripheral connections available at each pin."]
pub type IO0_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PORT_SEL0_SPEC, u8, IO0_SEL_A, 5, O>;
impl<'a, const O: u8> IO0_SEL_W<'a, O> {
    #[doc = "GPIO controls 'out'"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO)
    }
    #[doc = "GPIO controls 'out', DSI controls 'output enable'"]
    #[inline(always)]
    pub fn gpio_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO_DSI)
    }
    #[doc = "DSI controls 'out' and 'output enable'"]
    #[inline(always)]
    pub fn dsi_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_DSI)
    }
    #[doc = "DSI controls 'out', GPIO controls 'output enable'"]
    #[inline(always)]
    pub fn dsi_gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_GPIO)
    }
    #[doc = "AMUXBUS A"]
    #[inline(always)]
    pub fn amuxa(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA)
    }
    #[doc = "AMUXBUS B"]
    #[inline(always)]
    pub fn amuxb(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB)
    }
    #[doc = "Analog mux bus A, DSI control"]
    #[inline(always)]
    pub fn amuxa_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA_DSI)
    }
    #[doc = "Analog mux bus B, DSI control"]
    #[inline(always)]
    pub fn amuxb_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB_DSI)
    }
    #[doc = "Active peripherals 0"]
    #[inline(always)]
    pub fn act_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_0)
    }
    #[doc = "Active peripherals 1"]
    #[inline(always)]
    pub fn act_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_1)
    }
    #[doc = "Active peripherals 2"]
    #[inline(always)]
    pub fn act_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_2)
    }
    #[doc = "Active peripherals 4"]
    #[inline(always)]
    pub fn act_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_3)
    }
    #[doc = "Deep Sleep peripherals 0"]
    #[inline(always)]
    pub fn ds_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_0)
    }
    #[doc = "Deep Sleep peripherals 1"]
    #[inline(always)]
    pub fn ds_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_1)
    }
    #[doc = "Deep Sleep peripherals 2"]
    #[inline(always)]
    pub fn ds_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_2)
    }
    #[doc = "Deep Sleep peripherals 3"]
    #[inline(always)]
    pub fn ds_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_3)
    }
    #[doc = "Active peripherals 4"]
    #[inline(always)]
    pub fn act_4(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_4)
    }
    #[doc = "Active peripherals 5"]
    #[inline(always)]
    pub fn act_5(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_5)
    }
    #[doc = "Active peripherals 6"]
    #[inline(always)]
    pub fn act_6(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_6)
    }
    #[doc = "Active peripherals 7"]
    #[inline(always)]
    pub fn act_7(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_7)
    }
    #[doc = "Active peripherals 8"]
    #[inline(always)]
    pub fn act_8(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_8)
    }
    #[doc = "Active peripherals 9"]
    #[inline(always)]
    pub fn act_9(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_9)
    }
    #[doc = "Active peripherals 10"]
    #[inline(always)]
    pub fn act_10(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_10)
    }
    #[doc = "Active peripherals 11"]
    #[inline(always)]
    pub fn act_11(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_11)
    }
    #[doc = "Active peripherals 12"]
    #[inline(always)]
    pub fn act_12(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_12)
    }
    #[doc = "Active peripherals 13"]
    #[inline(always)]
    pub fn act_13(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_13)
    }
    #[doc = "Active peripherals 14"]
    #[inline(always)]
    pub fn act_14(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_14)
    }
    #[doc = "Active peripherals 15"]
    #[inline(always)]
    pub fn act_15(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_15)
    }
    #[doc = "DeepSleep functionality 4"]
    #[inline(always)]
    pub fn ds_4(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_4)
    }
    #[doc = "DeepSleep functionality 5"]
    #[inline(always)]
    pub fn ds_5(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_5)
    }
    #[doc = "DeepSleep functionality 6"]
    #[inline(always)]
    pub fn ds_6(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_6)
    }
    #[doc = "DeepSleep functionality 7"]
    #[inline(always)]
    pub fn ds_7(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_7)
    }
}
#[doc = "Field `IO1_SEL` reader - Selects the peripheral connections of Pin 1."]
pub type IO1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO1_SEL` writer - Selects the peripheral connections of Pin 1."]
pub type IO1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `IO2_SEL` reader - Selects the peripheral connections of Pin 2."]
pub type IO2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO2_SEL` writer - Selects the peripheral connections of Pin 2."]
pub type IO2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `IO3_SEL` reader - Selects the peripheral connections of Pin 3."]
pub type IO3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO3_SEL` writer - Selects the peripheral connections of Pin 3."]
pub type IO3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Selects the peripheral connections of Pin 0. Note that available connectivity options vary depending on the device, port and the pin. See the device Datasheet for a list of peripheral connections available at each pin."]
    #[inline(always)]
    pub fn io0_sel(&self) -> IO0_SEL_R {
        IO0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects the peripheral connections of Pin 1."]
    #[inline(always)]
    pub fn io1_sel(&self) -> IO1_SEL_R {
        IO1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects the peripheral connections of Pin 2."]
    #[inline(always)]
    pub fn io2_sel(&self) -> IO2_SEL_R {
        IO2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects the peripheral connections of Pin 3."]
    #[inline(always)]
    pub fn io3_sel(&self) -> IO3_SEL_R {
        IO3_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the peripheral connections of Pin 0. Note that available connectivity options vary depending on the device, port and the pin. See the device Datasheet for a list of peripheral connections available at each pin."]
    #[inline(always)]
    #[must_use]
    pub fn io0_sel(&mut self) -> IO0_SEL_W<0> {
        IO0_SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Selects the peripheral connections of Pin 1."]
    #[inline(always)]
    #[must_use]
    pub fn io1_sel(&mut self) -> IO1_SEL_W<8> {
        IO1_SEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Selects the peripheral connections of Pin 2."]
    #[inline(always)]
    #[must_use]
    pub fn io2_sel(&mut self) -> IO2_SEL_W<16> {
        IO2_SEL_W::new(self)
    }
    #[doc = "Bits 24:28 - Selects the peripheral connections of Pin 3."]
    #[inline(always)]
    #[must_use]
    pub fn io3_sel(&mut self) -> IO3_SEL_W<24> {
        IO3_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel0](index.html) module"]
pub struct PORT_SEL0_SPEC;
impl crate::RegisterSpec for PORT_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel0::R](R) reader structure"]
impl crate::Readable for PORT_SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel0::W](W) writer structure"]
impl crate::Writable for PORT_SEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_SEL0 to value 0"]
impl crate::Resettable for PORT_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
