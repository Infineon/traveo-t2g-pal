#[doc = "Register `STEP_CTL_BUFF` reader"]
pub struct R(crate::R<STEP_CTL_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_CTL_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_CTL_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_CTL_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP_CTL_BUFF` writer"]
pub struct W(crate::W<STEP_CTL_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_CTL_BUFF_SPEC>;
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
impl From<crate::W<STEP_CTL_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_CTL_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - See STEP_CTL."]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STEP` writer - See STEP_CTL."]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_CTL_BUFF_SPEC, u16, u16, 16, O>;
#[doc = "Field `VALID` reader - Specifies validity of the buffered segment structure: '0': Invalid. '1': Valid. HW clears this field to '0' on a completion event. Typically, SW sets this field to '1' when a new buffered segment structure is written. An underflow event is generated when STEP_CTL_BUFF.VALID is '0' on a completion event."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Specifies validity of the buffered segment structure: '0': Invalid. '1': Valid. HW clears this field to '0' on a completion event. Typically, SW sets this field to '1' when a new buffered segment structure is written. An underflow event is generated when STEP_CTL_BUFF.VALID is '0' on a completion event."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, STEP_CTL_BUFF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - See STEP_CTL."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Specifies validity of the buffered segment structure: '0': Invalid. '1': Valid. HW clears this field to '0' on a completion event. Typically, SW sets this field to '1' when a new buffered segment structure is written. An underflow event is generated when STEP_CTL_BUFF.VALID is '0' on a completion event."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - See STEP_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<0> {
        STEP_W::new(self)
    }
    #[doc = "Bit 31 - Specifies validity of the buffered segment structure: '0': Invalid. '1': Valid. HW clears this field to '0' on a completion event. Typically, SW sets this field to '1' when a new buffered segment structure is written. An underflow event is generated when STEP_CTL_BUFF.VALID is '0' on a completion event."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffered step control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step_ctl_buff](index.html) module"]
pub struct STEP_CTL_BUFF_SPEC;
impl crate::RegisterSpec for STEP_CTL_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [step_ctl_buff::R](R) reader structure"]
impl crate::Readable for STEP_CTL_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step_ctl_buff::W](W) writer structure"]
impl crate::Writable for STEP_CTL_BUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STEP_CTL_BUFF to value 0"]
impl crate::Resettable for STEP_CTL_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
