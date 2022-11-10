#[doc = "Register `CAP_CH0_CFG` reader"]
pub struct R(crate::R<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH0_CFG` writer"]
pub struct W(crate::W<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH0_CFG_SPEC>;
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
impl From<crate::W<CAP_CH0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0_EN` reader - When set, capture on channel 0 is enabled"]
pub type CAP0_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAP0_EN` writer - When set, capture on channel 0 is enabled"]
pub type CAP0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP_CH0_CFG_SPEC, bool, O>;
#[doc = "Field `CAP0_MODE` reader - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP0_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP0_MODE` writer - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP0_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP_CH0_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CAP0_PRESCALE` reader - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1"]
pub type CAP0_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP0_PRESCALE` writer - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1"]
pub type CAP0_PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAP_CH0_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `CAP0_IN_INVERT` reader - when set, CAP0 form GPIO matrix is inverted before prescale"]
pub type CAP0_IN_INVERT_R = crate::BitReader<bool>;
#[doc = "Field `CAP0_IN_INVERT` writer - when set, CAP0 form GPIO matrix is inverted before prescale"]
pub type CAP0_IN_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP_CH0_CFG_SPEC, bool, O>;
#[doc = "Field `CAP0_SW` writer - Write 1 will trigger a software forced capture on channel 0"]
pub type CAP0_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAP_CH0_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When set, capture on channel 0 is enabled"]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP0_EN_R {
        CAP0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap0_mode(&self) -> CAP0_MODE_R {
        CAP0_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap0_prescale(&self) -> CAP0_PRESCALE_R {
        CAP0_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP0 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap0_in_invert(&self) -> CAP0_IN_INVERT_R {
        CAP0_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 0 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_en(&mut self) -> CAP0_EN_W<0> {
        CAP0_EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap0_mode(&mut self) -> CAP0_MODE_W<1> {
        CAP0_MODE_W::new(self)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_prescale(&mut self) -> CAP0_PRESCALE_W<3> {
        CAP0_PRESCALE_W::new(self)
    }
    #[doc = "Bit 11 - when set, CAP0 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_in_invert(&mut self) -> CAP0_IN_INVERT_W<11> {
        CAP0_IN_INVERT_W::new(self)
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_sw(&mut self) -> CAP0_SW_W<12> {
        CAP0_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture channel 0 configuration and enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch0_cfg](index.html) module"]
pub struct CAP_CH0_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch0_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch0_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH0_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_CH0_CFG to value 0"]
impl crate::Resettable for CAP_CH0_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}