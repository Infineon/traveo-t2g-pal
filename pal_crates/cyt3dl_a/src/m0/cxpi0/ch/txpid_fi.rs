#[doc = "Register `TXPID_FI` reader"]
pub struct R(crate::R<TXPID_FI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPID_FI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPID_FI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPID_FI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPID_FI` writer"]
pub struct W(crate::W<TXPID_FI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPID_FI_SPEC>;
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
impl From<crate::W<TXPID_FI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPID_FI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Software does not need to program the parity bit i.e. bit\\[7\\]. HW will calculate the odd parity bit and ignore the bit\\[7\\]
if SW occupies this bit. Transmission: To be transmitted PID field. HW will ignore bit\\[7\\]
and compute the parity bit based on bits\\[6:0\\]
Note that, this field can be use by SW to send PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and the odd parity will be calculated by the HW."]
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID` writer - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Software does not need to program the parity bit i.e. bit\\[7\\]. HW will calculate the odd parity bit and ignore the bit\\[7\\]
if SW occupies this bit. Transmission: To be transmitted PID field. HW will ignore bit\\[7\\]
and compute the parity bit based on bits\\[6:0\\]
Note that, this field can be use by SW to send PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and the odd parity will be calculated by the HW."]
pub type PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXPID_FI_SPEC, u8, u8, 8, O>;
#[doc = "Field `FI` reader - Frame Information. This is the byte that will be transmitted as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT. Please program to 2'b11 if no support of counter."]
pub type FI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FI` writer - Frame Information. This is the byte that will be transmitted as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT. Please program to 2'b11 if no support of counter."]
pub type FI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXPID_FI_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLCEXT` reader - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[7:4\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
pub type DLCEXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLCEXT` writer - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[7:4\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
pub type DLCEXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXPID_FI_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Software does not need to program the parity bit i.e. bit\\[7\\]. HW will calculate the odd parity bit and ignore the bit\\[7\\]
if SW occupies this bit. Transmission: To be transmitted PID field. HW will ignore bit\\[7\\]
and compute the parity bit based on bits\\[6:0\\]
Note that, this field can be use by SW to send PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and the odd parity will be calculated by the HW."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Frame Information. This is the byte that will be transmitted as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT. Please program to 2'b11 if no support of counter."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[7:4\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
    #[inline(always)]
    pub fn dlcext(&self) -> DLCEXT_R {
        DLCEXT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Software does not need to program the parity bit i.e. bit\\[7\\]. HW will calculate the odd parity bit and ignore the bit\\[7\\]
if SW occupies this bit. Transmission: To be transmitted PID field. HW will ignore bit\\[7\\]
and compute the parity bit based on bits\\[6:0\\]
Note that, this field can be use by SW to send PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and the odd parity will be calculated by the HW."]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Bits 8:15 - Frame Information. This is the byte that will be transmitted as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT. Please program to 2'b11 if no support of counter."]
    #[inline(always)]
    #[must_use]
    pub fn fi(&mut self) -> FI_W<8> {
        FI_W::new(self)
    }
    #[doc = "Bits 16:23 - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[7:4\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn dlcext(&mut self) -> DLCEXT_W<16> {
        DLCEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXPID and Frame Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpid_fi](index.html) module"]
pub struct TXPID_FI_SPEC;
impl crate::RegisterSpec for TXPID_FI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpid_fi::R](R) reader structure"]
impl crate::Readable for TXPID_FI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpid_fi::W](W) writer structure"]
impl crate::Writable for TXPID_FI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPID_FI to value 0"]
impl crate::Resettable for TXPID_FI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
