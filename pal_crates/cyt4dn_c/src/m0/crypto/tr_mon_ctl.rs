#[doc = "Register `TR_MON_CTL` reader"]
pub struct R(crate::R<TR_MON_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_MON_CTL` writer"]
pub struct W(crate::W<TR_MON_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_MON_CTL_SPEC>;
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
impl From<crate::W<TR_MON_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_MON_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITSTREAM_SEL` reader - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
pub type BITSTREAM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSTREAM_SEL` writer - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
pub type BITSTREAM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_MON_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
    #[inline(always)]
    pub fn bitstream_sel(&self) -> BITSTREAM_SEL_R {
        BITSTREAM_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn bitstream_sel(&mut self) -> BITSTREAM_SEL_W<0> {
        BITSTREAM_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random monitor control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_ctl](index.html) module"]
pub struct TR_MON_CTL_SPEC;
impl crate::RegisterSpec for TR_MON_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_ctl::R](R) reader structure"]
impl crate::Readable for TR_MON_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_mon_ctl::W](W) writer structure"]
impl crate::Writable for TR_MON_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_MON_CTL to value 0x02"]
impl crate::Resettable for TR_MON_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
