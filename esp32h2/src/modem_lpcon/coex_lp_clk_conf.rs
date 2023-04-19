#[doc = "Register `COEX_LP_CLK_CONF` reader"]
pub struct R(crate::R<COEX_LP_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEX_LP_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEX_LP_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEX_LP_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEX_LP_CLK_CONF` writer"]
pub struct W(crate::W<COEX_LP_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEX_LP_CLK_CONF_SPEC>;
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
impl From<crate::W<COEX_LP_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEX_LP_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_COEX_LP_SEL_OSC_SLOW` reader - "]
pub type CLK_COEX_LP_SEL_OSC_SLOW_R = crate::BitReader<bool>;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_SLOW` writer - "]
pub type CLK_COEX_LP_SEL_OSC_SLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COEX_LP_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_FAST` reader - "]
pub type CLK_COEX_LP_SEL_OSC_FAST_R = crate::BitReader<bool>;
#[doc = "Field `CLK_COEX_LP_SEL_OSC_FAST` writer - "]
pub type CLK_COEX_LP_SEL_OSC_FAST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COEX_LP_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL` reader - "]
pub type CLK_COEX_LP_SEL_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL` writer - "]
pub type CLK_COEX_LP_SEL_XTAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COEX_LP_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL32K` reader - "]
pub type CLK_COEX_LP_SEL_XTAL32K_R = crate::BitReader<bool>;
#[doc = "Field `CLK_COEX_LP_SEL_XTAL32K` writer - "]
pub type CLK_COEX_LP_SEL_XTAL32K_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COEX_LP_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `CLK_COEX_LP_DIV_NUM` reader - "]
pub type CLK_COEX_LP_DIV_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLK_COEX_LP_DIV_NUM` writer - "]
pub type CLK_COEX_LP_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEX_LP_CLK_CONF_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_osc_slow(&self) -> CLK_COEX_LP_SEL_OSC_SLOW_R {
        CLK_COEX_LP_SEL_OSC_SLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_osc_fast(&self) -> CLK_COEX_LP_SEL_OSC_FAST_R {
        CLK_COEX_LP_SEL_OSC_FAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_xtal(&self) -> CLK_COEX_LP_SEL_XTAL_R {
        CLK_COEX_LP_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_coex_lp_sel_xtal32k(&self) -> CLK_COEX_LP_SEL_XTAL32K_R {
        CLK_COEX_LP_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn clk_coex_lp_div_num(&self) -> CLK_COEX_LP_DIV_NUM_R {
        CLK_COEX_LP_DIV_NUM_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_osc_slow(&mut self) -> CLK_COEX_LP_SEL_OSC_SLOW_W<0> {
        CLK_COEX_LP_SEL_OSC_SLOW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_osc_fast(&mut self) -> CLK_COEX_LP_SEL_OSC_FAST_W<1> {
        CLK_COEX_LP_SEL_OSC_FAST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_xtal(&mut self) -> CLK_COEX_LP_SEL_XTAL_W<2> {
        CLK_COEX_LP_SEL_XTAL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_sel_xtal32k(&mut self) -> CLK_COEX_LP_SEL_XTAL32K_W<3> {
        CLK_COEX_LP_SEL_XTAL32K_W::new(self)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_lp_div_num(&mut self) -> CLK_COEX_LP_DIV_NUM_W<4> {
        CLK_COEX_LP_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_lp_clk_conf](index.html) module"]
pub struct COEX_LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for COEX_LP_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coex_lp_clk_conf::R](R) reader structure"]
impl crate::Readable for COEX_LP_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coex_lp_clk_conf::W](W) writer structure"]
impl crate::Writable for COEX_LP_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEX_LP_CLK_CONF to value 0"]
impl crate::Resettable for COEX_LP_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}