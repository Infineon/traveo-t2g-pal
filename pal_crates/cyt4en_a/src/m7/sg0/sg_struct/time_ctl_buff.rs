#[doc = "Register `TIME_CTL_BUFF` reader"]
pub struct R(crate::R<TIME_CTL_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_CTL_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_CTL_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_CTL_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME_CTL_BUFF` writer"]
pub struct W(crate::W<TIME_CTL_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_CTL_BUFF_SPEC>;
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
impl From<crate::W<TIME_CTL_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_CTL_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD8` reader - See TIME_CTL."]
pub type PERIOD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD8` writer - See TIME_CTL."]
pub type PERIOD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_CTL_BUFF_SPEC, u8, u8, 8, O>;
#[doc = "Field `NR` reader - See TIME_CTL."]
pub type NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NR` writer - See TIME_CTL."]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_CTL_BUFF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - See TIME_CTL."]
    #[inline(always)]
    pub fn period8(&self) -> PERIOD8_R {
        PERIOD8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - See TIME_CTL."]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - See TIME_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn period8(&mut self) -> PERIOD8_W<0> {
        PERIOD8_W::new(self)
    }
    #[doc = "Bits 16:23 - See TIME_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NR_W<16> {
        NR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffered time control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_ctl_buff](index.html) module"]
pub struct TIME_CTL_BUFF_SPEC;
impl crate::RegisterSpec for TIME_CTL_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_ctl_buff::R](R) reader structure"]
impl crate::Readable for TIME_CTL_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_ctl_buff::W](W) writer structure"]
impl crate::Writable for TIME_CTL_BUFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME_CTL_BUFF to value 0"]
impl crate::Resettable for TIME_CTL_BUFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
