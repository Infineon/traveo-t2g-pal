#[doc = "Register `AMPL_CTL_BUFF` reader"]
pub struct R(crate::R<AMPL_CTL_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPL_CTL_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPL_CTL_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPL_CTL_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPL_CTL_BUFF` writer"]
pub struct W(crate::W<AMPL_CTL_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPL_CTL_BUFF_SPEC>;
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
impl From<crate::W<AMPL_CTL_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPL_CTL_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD16` reader - See AMPL_CTL."]
pub type PERIOD16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERIOD16` writer - See AMPL_CTL."]
pub type PERIOD16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPL_CTL_BUFF_SPEC, u16, u16, 16, O>;
#[doc = "Field `HIGH16` reader - See AMPL_CTL."]
pub type HIGH16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIGH16` writer - See AMPL_CTL."]
pub type HIGH16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPL_CTL_BUFF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - See AMPL_CTL."]
    #[inline(always)]
    pub fn period16(&self) -> PERIOD16_R {
        PERIOD16_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - See AMPL_CTL."]
    #[inline(always)]
    pub fn high16(&self) -> HIGH16_R {
        HIGH16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - See AMPL_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn period16(&mut self) -> PERIOD16_W<0> {
        PERIOD16_W::new(self)
    }
    #[doc = "Bits 16:31 - See AMPL_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn high16(&mut self) -> HIGH16_W<16> {
        HIGH16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffered amplitude control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampl_ctl_buff](index.html) module"]
pub struct AMPL_CTL_BUFF_SPEC;
impl crate::RegisterSpec for AMPL_CTL_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampl_ctl_buff::R](R) reader structure"]
impl crate::Readable for AMPL_CTL_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampl_ctl_buff::W](W) writer structure"]
impl crate::Writable for AMPL_CTL_BUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPL_CTL_BUFF to value 0"]
impl crate::Resettable for AMPL_CTL_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
