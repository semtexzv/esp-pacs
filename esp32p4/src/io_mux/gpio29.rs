#[doc = "Register `gpio29` reader"]
pub type R = crate::R<GPIO29_SPEC>;
#[doc = "Register `gpio29` writer"]
pub type W = crate::W<GPIO29_SPEC>;
#[doc = "Field `GPIO29_MCU_OE` reader - output enable on sleep mode"]
pub type GPIO29_MCU_OE_R = crate::BitReader;
#[doc = "Field `GPIO29_MCU_OE` writer - output enable on sleep mode"]
pub type GPIO29_MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_SLP_SEL` reader - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO29_SLP_SEL_R = crate::BitReader;
#[doc = "Field `GPIO29_SLP_SEL` writer - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO29_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_MCU_WPD` reader - pull-down enable on sleep mode"]
pub type GPIO29_MCU_WPD_R = crate::BitReader;
#[doc = "Field `GPIO29_MCU_WPD` writer - pull-down enable on sleep mode"]
pub type GPIO29_MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_MCU_WPU` reader - pull-up enable on sleep mode"]
pub type GPIO29_MCU_WPU_R = crate::BitReader;
#[doc = "Field `GPIO29_MCU_WPU` writer - pull-up enable on sleep mode"]
pub type GPIO29_MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_MCU_IE` reader - input enable on sleep mode"]
pub type GPIO29_MCU_IE_R = crate::BitReader;
#[doc = "Field `GPIO29_MCU_IE` writer - input enable on sleep mode"]
pub type GPIO29_MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_MCU_DRV` reader - select drive strenth on sleep mode"]
pub type GPIO29_MCU_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO29_MCU_DRV` writer - select drive strenth on sleep mode"]
pub type GPIO29_MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO29_FUN_WPD` reader - pull-down enable"]
pub type GPIO29_FUN_WPD_R = crate::BitReader;
#[doc = "Field `GPIO29_FUN_WPD` writer - pull-down enable"]
pub type GPIO29_FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_FUN_WPU` reader - pull-up enable"]
pub type GPIO29_FUN_WPU_R = crate::BitReader;
#[doc = "Field `GPIO29_FUN_WPU` writer - pull-up enable"]
pub type GPIO29_FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_FUN_IE` reader - input enable"]
pub type GPIO29_FUN_IE_R = crate::BitReader;
#[doc = "Field `GPIO29_FUN_IE` writer - input enable"]
pub type GPIO29_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO29_FUN_DRV` reader - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO29_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO29_FUN_DRV` writer - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO29_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO29_MCU_SEL` reader - 0:select function0, 1:select function1 ..."]
pub type GPIO29_MCU_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO29_MCU_SEL` writer - 0:select function0, 1:select function1 ..."]
pub type GPIO29_MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO29_FILTER_EN` reader - input filter enable"]
pub type GPIO29_FILTER_EN_R = crate::BitReader;
#[doc = "Field `GPIO29_FILTER_EN` writer - input filter enable"]
pub type GPIO29_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    pub fn gpio29_mcu_oe(&self) -> GPIO29_MCU_OE_R {
        GPIO29_MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    pub fn gpio29_slp_sel(&self) -> GPIO29_SLP_SEL_R {
        GPIO29_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    pub fn gpio29_mcu_wpd(&self) -> GPIO29_MCU_WPD_R {
        GPIO29_MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    pub fn gpio29_mcu_wpu(&self) -> GPIO29_MCU_WPU_R {
        GPIO29_MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    pub fn gpio29_mcu_ie(&self) -> GPIO29_MCU_IE_R {
        GPIO29_MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    pub fn gpio29_mcu_drv(&self) -> GPIO29_MCU_DRV_R {
        GPIO29_MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    pub fn gpio29_fun_wpd(&self) -> GPIO29_FUN_WPD_R {
        GPIO29_FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    pub fn gpio29_fun_wpu(&self) -> GPIO29_FUN_WPU_R {
        GPIO29_FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    pub fn gpio29_fun_ie(&self) -> GPIO29_FUN_IE_R {
        GPIO29_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    pub fn gpio29_fun_drv(&self) -> GPIO29_FUN_DRV_R {
        GPIO29_FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    pub fn gpio29_mcu_sel(&self) -> GPIO29_MCU_SEL_R {
        GPIO29_MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    pub fn gpio29_filter_en(&self) -> GPIO29_FILTER_EN_R {
        GPIO29_FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("gpio29")
            .field(
                "gpio29_mcu_oe",
                &format_args!("{}", self.gpio29_mcu_oe().bit()),
            )
            .field(
                "gpio29_slp_sel",
                &format_args!("{}", self.gpio29_slp_sel().bit()),
            )
            .field(
                "gpio29_mcu_wpd",
                &format_args!("{}", self.gpio29_mcu_wpd().bit()),
            )
            .field(
                "gpio29_mcu_wpu",
                &format_args!("{}", self.gpio29_mcu_wpu().bit()),
            )
            .field(
                "gpio29_mcu_ie",
                &format_args!("{}", self.gpio29_mcu_ie().bit()),
            )
            .field(
                "gpio29_mcu_drv",
                &format_args!("{}", self.gpio29_mcu_drv().bits()),
            )
            .field(
                "gpio29_fun_wpd",
                &format_args!("{}", self.gpio29_fun_wpd().bit()),
            )
            .field(
                "gpio29_fun_wpu",
                &format_args!("{}", self.gpio29_fun_wpu().bit()),
            )
            .field(
                "gpio29_fun_ie",
                &format_args!("{}", self.gpio29_fun_ie().bit()),
            )
            .field(
                "gpio29_fun_drv",
                &format_args!("{}", self.gpio29_fun_drv().bits()),
            )
            .field(
                "gpio29_mcu_sel",
                &format_args!("{}", self.gpio29_mcu_sel().bits()),
            )
            .field(
                "gpio29_filter_en",
                &format_args!("{}", self.gpio29_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO29_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_oe(&mut self) -> GPIO29_MCU_OE_W<GPIO29_SPEC> {
        GPIO29_MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_slp_sel(&mut self) -> GPIO29_SLP_SEL_W<GPIO29_SPEC> {
        GPIO29_SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_wpd(&mut self) -> GPIO29_MCU_WPD_W<GPIO29_SPEC> {
        GPIO29_MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_wpu(&mut self) -> GPIO29_MCU_WPU_W<GPIO29_SPEC> {
        GPIO29_MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_ie(&mut self) -> GPIO29_MCU_IE_W<GPIO29_SPEC> {
        GPIO29_MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_drv(&mut self) -> GPIO29_MCU_DRV_W<GPIO29_SPEC> {
        GPIO29_MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_fun_wpd(&mut self) -> GPIO29_FUN_WPD_W<GPIO29_SPEC> {
        GPIO29_FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_fun_wpu(&mut self) -> GPIO29_FUN_WPU_W<GPIO29_SPEC> {
        GPIO29_FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_fun_ie(&mut self) -> GPIO29_FUN_IE_W<GPIO29_SPEC> {
        GPIO29_FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_fun_drv(&mut self) -> GPIO29_FUN_DRV_W<GPIO29_SPEC> {
        GPIO29_FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_mcu_sel(&mut self) -> GPIO29_MCU_SEL_W<GPIO29_SPEC> {
        GPIO29_MCU_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio29_filter_en(&mut self) -> GPIO29_FILTER_EN_W<GPIO29_SPEC> {
        GPIO29_FILTER_EN_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "iomux control register for gpio29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO29_SPEC;
impl crate::RegisterSpec for GPIO29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio29::R`](R) reader structure"]
impl crate::Readable for GPIO29_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio29::W`](W) writer structure"]
impl crate::Writable for GPIO29_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio29 to value 0x0800"]
impl crate::Resettable for GPIO29_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}