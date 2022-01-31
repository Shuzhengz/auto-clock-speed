use super::config::Config;

pub struct Undervolt {
}

let msr_reg = vec![
    'MSR_PLATFORM_INFO': 0xCE,
    'MSR_OC_MAILBOX': 0x150,
    'IA32_PERF_STATUS': 0x198,
    'IA32_THERM_STATUS': 0x19C,
    'MSR_TEMPERATURE_TARGET': 0x1A2,
    'MSR_POWER_CTL': 0x1FC,
    'MSR_RAPL_POWER_UNIT': 0x606,
    'MSR_PKG_POWER_LIMIT': 0x610,
    'MSR_INTEL_PKG_ENERGY_STATUS': 0x611,
    'MSR_DRAM_ENERGY_STATUS': 0x619,
    'MSR_PP1_ENERGY_STATUS': 0x641,
    'MSR_CONFIG_TDP_CONTROL': 0x64B,
    'IA32_HWP_REQUEST': 0x774,
];
