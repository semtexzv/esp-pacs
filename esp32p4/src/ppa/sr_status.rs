#[doc = "Register `SR_STATUS` reader"]
pub type R = crate::R<SR_STATUS_SPEC>;
#[doc = "Field `SR_RX_DSCR_SAMPLE_STATE` reader - Reserved."]
pub type SR_RX_DSCR_SAMPLE_STATE_R = crate::FieldReader;
#[doc = "Field `SR_RX_SCAN_STATE` reader - Reserved."]
pub type SR_RX_SCAN_STATE_R = crate::FieldReader;
#[doc = "Field `SR_TX_DSCR_SAMPLE_STATE` reader - Reserved."]
pub type SR_TX_DSCR_SAMPLE_STATE_R = crate::FieldReader;
#[doc = "Field `SR_TX_SCAN_STATE` reader - Reserved."]
pub type SR_TX_SCAN_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Reserved."]
    #[inline(always)]
    pub fn sr_rx_dscr_sample_state(&self) -> SR_RX_DSCR_SAMPLE_STATE_R {
        SR_RX_DSCR_SAMPLE_STATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved."]
    #[inline(always)]
    pub fn sr_rx_scan_state(&self) -> SR_RX_SCAN_STATE_R {
        SR_RX_SCAN_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Reserved."]
    #[inline(always)]
    pub fn sr_tx_dscr_sample_state(&self) -> SR_TX_DSCR_SAMPLE_STATE_R {
        SR_TX_DSCR_SAMPLE_STATE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - Reserved."]
    #[inline(always)]
    pub fn sr_tx_scan_state(&self) -> SR_TX_SCAN_STATE_R {
        SR_TX_SCAN_STATE_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_STATUS")
            .field(
                "sr_rx_dscr_sample_state",
                &format_args!("{}", self.sr_rx_dscr_sample_state().bits()),
            )
            .field(
                "sr_rx_scan_state",
                &format_args!("{}", self.sr_rx_scan_state().bits()),
            )
            .field(
                "sr_tx_dscr_sample_state",
                &format_args!("{}", self.sr_tx_dscr_sample_state().bits()),
            )
            .field(
                "sr_tx_scan_state",
                &format_args!("{}", self.sr_tx_scan_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SR FSM register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_STATUS_SPEC;
impl crate::RegisterSpec for SR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_status::R`](R) reader structure"]
impl crate::Readable for SR_STATUS_SPEC {}
#[doc = "`reset()` method sets SR_STATUS to value 0"]
impl crate::Resettable for SR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}