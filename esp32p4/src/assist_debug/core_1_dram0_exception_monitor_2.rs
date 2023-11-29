#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_2` reader"]
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_PC_0` reader - reg_core_1_dram0_recording_pc_0"]
pub type CORE_1_DRAM0_RECORDING_PC_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_1_dram0_recording_pc_0"]
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_0(&self) -> CORE_1_DRAM0_RECORDING_PC_0_R {
        CORE_1_DRAM0_RECORDING_PC_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_2")
            .field(
                "core_1_dram0_recording_pc_0",
                &format_args!("{}", self.core_1_dram0_recording_pc_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "exception monitor status register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_2::R`](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}