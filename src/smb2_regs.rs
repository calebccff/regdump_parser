
use bitflags::bitflags;
use std::fmt;

pub struct Register {
    pub addr: u16,
    pub value: u8,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#06x}: {: >48}: {:#04x} {:?}", self.addr, register_name(self.addr), self.value, register_flags(self.addr, self.value).unwrap_or(Box::new("")))
    }
}

bitflags! {
    pub struct CHGR_FREQ_BASE_VAL: u8 {
        const TYPE_MASK                                        = 0b11111111;
        const SUBTYPE_MASK                                     = 0b11111111;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_1: u8 {
        const BVR_INITIAL_RAMP_BIT                             = 0b10000000;
        const CC_SOFT_TERMINATE_BIT                            = 0b01000000;
        const STEP_CHARGING_STATUS_MASK                        = 0b00111000;
        const BATTERY_CHARGER_STATUS_MASK                      = 0b00000111;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_2: u8 {
        const INPUT_CURRENT_LIMITED_BIT                        = 0b10000000;
        const CHARGER_ERROR_STATUS_SFT_EXPIRE_BIT              = 0b01000000;
        const CHARGER_ERROR_STATUS_BAT_OV_BIT                  = 0b00100000;
        const CHARGER_ERROR_STATUS_BAT_TERM_MISSING_BIT        = 0b00010000;
        const BAT_TEMP_STATUS_HOT_SOFT_LIMIT_BIT               = 0b00001000;
        const BAT_TEMP_STATUS_COLD_SOFT_LIMIT_BIT              = 0b00000100;
        const BAT_TEMP_STATUS_TOO_HOT_BIT                      = 0b00000010;
        const BAT_TEMP_STATUS_TOO_COLD_BIT                     = 0b00000001;
        const BAT_TEMP_STATUS_MASK                             = 0b00001111;
        const BAT_TEMP_STATUS_SOFT_LIMIT_MASK                  = 0b00001100;
    }
}

bitflags! {
    pub struct CHG_OPTION: u8 {
        const PIN_BIT                                          = 0b10000000;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_3: u8 {
        const FV_POST_JEITA_MASK                               = 0b11111111;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_4: u8 {
        const CHARGE_CURRENT_POST_JEITA_MASK                   = 0b11111111;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_5: u8 {
        const VALID_INPUT_POWER_SOURCE_BIT                     = 0b10000000;
        const DISABLE_CHARGING_BIT                             = 0b01000000;
        const FORCE_ZERO_CHARGE_CURRENT_BIT                    = 0b00100000;
        const CHARGING_ENABLE_BIT                              = 0b00010000;
        const TAPER_BIT                                        = 0b00001000;
        const ENABLE_CHG_SENSORS_BIT                           = 0b00000100;
        const ENABLE_TAPER_SENSOR_BIT                          = 0b00000010;
        const TAPER_REGION_BIT                                 = 0b00000001;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_6: u8 {
        const GF_BATT_OV_BIT                                   = 0b10000000;
        const DROP_IN_BATTERY_VOLTAGE_REFERENCE_BIT            = 0b01000000;
        const VBATT_LTET_RECHARGE_BIT                          = 0b00100000;
        const VBATT_GTET_INHIBIT_BIT                           = 0b00010000;
        const VBATT_GTET_FLOAT_VOLTAGE_BIT                     = 0b00001000;
        const BATT_GT_PRE_TO_FAST_BIT                          = 0b00000100;
        const BATT_GT_FULL_ON_BIT                              = 0b00000010;
        const VBATT_LT_2V_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_7: u8 {
        const ENABLE_TRICKLE_BIT                               = 0b10000000;
        const ENABLE_PRE_CHARGING_BIT                          = 0b01000000;
        const ENABLE_FAST_CHARGING_BIT                         = 0b00100000;
        const ENABLE_FULLON_MODE_BIT                           = 0b00010000;
        const TOO_COLD_ADC_BIT                                 = 0b00001000;
        const TOO_HOT_ADC_BIT                                  = 0b00000100;
        const HOT_SL_ADC_BIT                                   = 0b00000010;
        const COLD_SL_ADC_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct BATTERY_CHARGER_STATUS_8: u8 {
        const PRE_FAST_BIT                                     = 0b10000000;
        const PRE_FULLON_BIT                                   = 0b01000000;
        const PRE_RCHG_BIT                                     = 0b00100000;
        const PRE_INHIBIT_BIT                                  = 0b00010000;
        const PRE_OVRV_BIT                                     = 0b00001000;
        const PRE_TERM_BIT                                     = 0b00000100;
        const BAT_ID_BMISS_CMP_BIT                             = 0b00000010;
        const THERM_CMP_BIT                                    = 0b00000001;
        const CHGR_7_RT_STS_BIT                                = 0b10000000;
        const CHGR_6_RT_STS_BIT                                = 0b01000000;
        const FG_FVCAL_QUALIFIED_RT_STS_BIT                    = 0b00100000;
        const STEP_CHARGING_SOC_UPDATE_REQUEST_RT_STS_BIT      = 0b00010000;
        const STEP_CHARGING_SOC_UPDATE_FAIL_RT_STS_BIT         = 0b00001000;
        const STEP_CHARGING_STATE_CHANGE_RT_STS_BIT            = 0b00000100;
        const CHARGING_STATE_CHANGE_RT_STS_BIT                 = 0b00000010;
        const CHGR_ERROR_RT_STS_BIT                            = 0b00000001;
    }
}

bitflags! {
    pub struct STEP_CHG_SOC_VBATT_V: u8 {
        const STEP_CHG_SOC_VBATT_V_MASK                        = 0b11111111;
    }
}

bitflags! {
    pub struct STEP_CHG_SOC_VBATT_V_UPDATE: u8 {
        const STEP_CHG_SOC_VBATT_V_UPDATE_BIT                  = 0b00000001;
    }
}

bitflags! {
    pub struct CHARGING_ENABLE_CMD: u8 {
        const CHARGING_ENABLE_CMD_BIT                          = 0b00000001;
    }
}

bitflags! {
    pub struct ALLOW_FAST_CHARGING_CMD: u8 {
        const ALLOW_FAST_CHARGING_CMD_BIT                      = 0b00000001;
    }
}

bitflags! {
    pub struct QNOVO_PT_ENABLE_CMD: u8 {
        const QNOVO_PT_ENABLE_CMD_BIT                          = 0b00000001;
    }
}

bitflags! {
    pub struct CHGR_CFG1: u8 {
        const INCREASE_RCHG_TIMEOUT_CFG_BIT                    = 0b00000010;
        const LOAD_BAT_BIT                                     = 0b00000001;
    }
}

bitflags! {
    pub struct CHGR_CFG2: u8 {
        const CHG_EN_SRC_BIT                                   = 0b10000000;
        const CHG_EN_POLARITY_BIT                              = 0b01000000;
        const PRETOFAST_TRANSITION_CFG_BIT                     = 0b00100000;
        const BAT_OV_ECC_BIT                                   = 0b00010000;
        const I_TERM_BIT                                       = 0b00001000;
        const AUTO_RECHG_BIT                                   = 0b00000100;
        const EN_ANALOG_DROP_IN_VBATT_BIT                      = 0b00000010;
        const CHARGER_INHIBIT_BIT                              = 0b00000001;
    }
}

bitflags! {
    pub struct CHARGER_ENABLE_CFG: u8 {
        const CHG_ENB_TIMEOUT_SETTING_BIT                      = 0b00000010;
        const FORCE_ZERO_CFG_BIT                               = 0b00000001;
    }
}

bitflags! {
    pub struct CFG: u8 {
        const CHG_OPTION_PIN_TRIM_BIT                          = 0b10000000;
        const BATN_SNS_CFG_BIT                                 = 0b00010000;
        const CFG_TAPER_DIS_AFVC_BIT                           = 0b00001000;
        const BATFET_SHUTDOWN_CFG_BIT                          = 0b00000100;
        const VDISCHG_EN_CFG_BIT                               = 0b00000010;
        const VCHG_EN_CFG_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct CHARGER_SPARE: u8 {
        const CHARGER_SPARE_MASK                               = 0b00111111;
    }
}

bitflags! {
    pub struct PRE_CHARGE_CURRENT_CFG: u8 {
        const PRE_CHARGE_CURRENT_SETTING_MASK                  = 0b00111111;
    }
}

bitflags! {
    pub struct FAST_CHARGE_CURRENT_CFG: u8 {
        const FAST_CHARGE_CURRENT_SETTING_MASK                 = 0b11111111;
    }
}

bitflags! {
    pub struct CHARGE_CURRENT_TERMINATION_CFG: u8 {
        const ANALOG_CHARGE_CURRENT_TERMINATION_SETTING_MASK   = 0b00000111;
    }
}

bitflags! {
    pub struct TCCC_CHARGE_CURRENT_TERMINATION_CFG: u8 {
        const TCCC_CHARGE_CURRENT_TERMINATION_SETTING_MASK     = 0b00001111;
    }
}

bitflags! {
    pub struct CHARGE_CURRENT_SOFTSTART_SETTING_CFG: u8 {
        const CHARGE_CURRENT_SOFTSTART_SETTING_MASK            = 0b00000011;
    }
}

bitflags! {
    pub struct FLOAT_VOLTAGE_CFG: u8 {
        const FLOAT_VOLTAGE_SETTING_MASK                       = 0b11111111;
    }
}

bitflags! {
    pub struct AUTO_FLOAT_VOLTAGE_COMPENSATION_CFG: u8 {
        const AUTO_FLOAT_VOLTAGE_COMPENSATION_MASK             = 0b00000111;
    }
}

bitflags! {
    pub struct CHARGE_INHIBIT_THRESHOLD_CFG: u8 {
        const CHARGE_INHIBIT_THRESHOLD_MASK                    = 0b00000011;
    }
}

bitflags! {
    pub struct RECHARGE_THRESHOLD_CFG: u8 {
        const RECHARGE_THRESHOLD_MASK                          = 0b00000011;
    }
}

bitflags! {
    pub struct PRE_TO_FAST_CHARGE_THRESHOLD_CFG: u8 {
        const PRE_TO_FAST_CHARGE_THRESHOLD_MASK                = 0b00000011;
    }
}

bitflags! {
    pub struct FV_HYSTERESIS_CFG: u8 {
        const FV_DROP_HYSTERESIS_CFG_MASK                      = 0b11110000;
        const THRESH_HYSTERESIS_CFG_MASK                       = 0b00001111;
    }
}

bitflags! {
    pub struct FVC_CHARGE_INHIBIT_THRESHOLD_CFG: u8 {
        const FVC_CHARGE_INHIBIT_THRESHOLD_MASK                = 0b00111111;
    }
}

bitflags! {
    pub struct FVC_RECHARGE_THRESHOLD_CFG: u8 {
        const FVC_RECHARGE_THRESHOLD_MASK                      = 0b11111111;
    }
}

bitflags! {
    pub struct FVC_PRE_TO_FAST_CHARGE_THRESHOLD_CFG: u8 {
        const FVC_PRE_TO_FAST_CHARGE_THRESHOLD_MASK            = 0b11111111;
    }
}

bitflags! {
    pub struct FVC_FULL_ON_THRESHOLD_CFG: u8 {
        const FVC_FULL_ON_THRESHOLD_MASK                       = 0b11111111;
    }
}

bitflags! {
    pub struct FVC_CC_MODE_GLITCH_FILTER_SEL_CFG: u8 {
        const FVC_CC_MODE_GLITCH_FILTER_SEL_MASK               = 0b00000011;
    }
}

bitflags! {
    pub struct FVC_TERMINATION_GLITCH_FILTER_SEL_CFG: u8 {
        const FVC_TERMINATION_GLITCH_FILTER_SEL_MASK           = 0b00000011;
    }
}

bitflags! {
    pub struct JEITA_EN_CFG: u8 {
        const JEITA_EN_HARDLIMIT_BIT                           = 0b00010000;
        const JEITA_EN_HOT_SL_FCV_BIT                          = 0b00001000;
        const JEITA_EN_COLD_SL_FCV_BIT                         = 0b00000100;
        const JEITA_EN_HOT_SL_CCC_BIT                          = 0b00000010;
        const JEITA_EN_COLD_SL_CCC_BIT                         = 0b00000001;
    }
}

bitflags! {
    pub struct JEITA_FVCOMP_CFG: u8 {
        const JEITA_FVCOMP_MASK                                = 0b11111111;
    }
}

bitflags! {
    pub struct JEITA_CCCOMP_CFG: u8 {
        const JEITA_CCCOMP_MASK                                = 0b11111111;
    }
}

bitflags! {
    pub struct FV_CAL_CFG: u8 {
        const FV_CALIBRATION_CFG_MASK                          = 0b00000111;
    }
}

bitflags! {
    pub struct FV_ADJUST: u8 {
        const FLOAT_VOLTAGE_ADJUSTMENT_MASK                    = 0b00011111;
    }
}

bitflags! {
    pub struct FG_VADC_DISQ_THRESH: u8 {
        const VADC_DISQUAL_THRESH_MASK                         = 0b11111111;
    }
}

bitflags! {
    pub struct FG_IADC_DISQ_THRESH: u8 {
        const IADC_DISQUAL_THRESH_MASK                         = 0b11111111;
    }
}

bitflags! {
    pub struct FG_UPDATE_CFG_1: u8 {
        const BT_TMPR_TCOLD_BIT                                = 0b10000000;
        const BT_TMPR_COLD_BIT                                 = 0b01000000;
        const BT_TMPR_HOT_BIT                                  = 0b00100000;
        const BT_TMPR_THOT_BIT                                 = 0b00010000;
        const CHG_DIE_TMPR_HOT_BIT                             = 0b00001000;
        const CHG_DIE_TMPR_THOT_BIT                            = 0b00000100;
        const SKIN_TMPR_HOT_BIT                                = 0b00000010;
        const SKIN_TMPR_THOT_BIT                               = 0b00000001;
    }
}

bitflags! {
    pub struct FG_UPDATE_CFG_1_SEL: u8 {
        const BT_TMPR_TCOLD_SEL_BIT                            = 0b10000000;
        const BT_TMPR_COLD_SEL_BIT                             = 0b01000000;
        const BT_TMPR_HOT_SEL_BIT                              = 0b00100000;
        const BT_TMPR_THOT_SEL_BIT                             = 0b00010000;
        const CHG_DIE_TMPR_HOT_SEL_BIT                         = 0b00001000;
        const CHG_DIE_TMPR_THOT_SEL_BIT                        = 0b00000100;
        const SKIN_TMPR_HOT_SEL_BIT                            = 0b00000010;
        const SKIN_TMPR_THOT_SEL_BIT                           = 0b00000001;
    }
}

bitflags! {
    pub struct FG_UPDATE_CFG_2: u8 {
        const SOC_LT_OTG_THRESH_BIT                            = 0b00001000;
        const SOC_LT_CHG_RECHARGE_THRESH_BIT                   = 0b00000100;
        const VBT_LT_CHG_RECHARGE_THRESH_BIT                   = 0b00000010;
        const IBT_LT_CHG_TERM_THRESH_BIT                       = 0b00000001;
    }
}

bitflags! {
    pub struct FG_UPDATE_CFG_2_SEL: u8 {
        const SOC_LT_OTG_THRESH_SEL_BIT                        = 0b00001000;
        const SOC_LT_CHG_RECHARGE_THRESH_SEL_BIT               = 0b00000100;
        const VBT_LT_CHG_RECHARGE_THRESH_SEL_BIT               = 0b00000010;
        const IBT_LT_CHG_TERM_THRESH_SEL_BIT                   = 0b00000001;
    }
}

bitflags! {
    pub struct FG_CHG_INTERFACE_CFG: u8 {
        const FG_CHARGER_INHIBIT_BIT                           = 0b00001000;
        const FG_BATFET_BIT                                    = 0b00000100;
        const IADC_SYNC_CNV_BIT                                = 0b00000010;
        const VADC_SYNC_CNV_BIT                                = 0b00000001;
        const ESR_ISINK_CFG_MASK                               = 0b11000000;
        const ESR_FASTCHG_DECR_CFG_MASK                        = 0b00110000;
    }
}

bitflags! {
    pub struct FG_CHG_INTERFACE_CFG_SEL: u8 {
        const ESR_ISINK_CFG_SEL_BIT                            = 0b00100000;
        const ESR_FASTCHG_DECR_CFG_SEL_BIT                     = 0b00010000;
        const FG_CHARGER_INHIBIT_SEL_BIT                       = 0b00001000;
        const FG_BATFET_SEL_BIT                                = 0b00000100;
        const IADC_SYNC_CNV_SEL_BIT                            = 0b00000010;
        const VADC_SYNC_CNV_SEL_BIT                            = 0b00000001;
    }
}

bitflags! {
    pub struct CHGR_STEP_CHG_MODE_CFG: u8 {
        const STEP_CHARGING_SOC_FAIL_OPTION_BIT                = 0b00001000;
        const STEP_CHARGING_MODE_SELECT_BIT                    = 0b00000100;
        const STEP_CHARGING_SOURCE_SELECT_BIT                  = 0b00000010;
        const STEP_CHARGING_ENABLE_BIT                         = 0b00000001;
    }
}

bitflags! {
    pub struct STEP_CHG_UPDATE_REQUEST_TIMEOUT_CFG: u8 {
        const STEP_CHG_UPDATE_REQUEST_TIMEOUT_CFG_MASK         = 0b00000011;
    }
}

bitflags! {
    pub struct STEP_CHG_UPDATE_FAIL_TIMEOUT_CFG: u8 {
        const STEP_CHG_UPDATE_FAIL_TIMEOUT_CFG_MASK            = 0b00000011;
    }
}

bitflags! {
    pub struct RID_CC_CONTROL_23_16: u8 {
        const RID_CC_CONTROL_23_BIT                            = 0b10000000;
        const VCONN_SOFTSTART_EN_BIT                           = 0b01000000;
        const EN_CC_1P1CLAMP_BIT                               = 0b00000010;
        const ENABLE_CRUDESEN_CC_1_BIT                         = 0b00000001;
        const VCONN_SFTST_CFG_MASK                             = 0b00110000;
        const CONNECT_RIDCC_SENSOR_TO_CC_MASK                  = 0b00001100;
    }
}

bitflags! {
    pub struct RID_CC_CONTROL_15_8: u8 {
        const ENABLE_CRUDESEN_CC_0_BIT                         = 0b10000000;
        const EN_ISRC_180UA_BIT                                = 0b00010000;
        const EN_BANDGAP_RID_C_DET_BIT                         = 0b00000010;
        const ENABLE_RD_CC_1_BIT                               = 0b00000001;
        const EN_FMB_2P5UA_CC_MASK                             = 0b01100000;
        const ENABLE_CURRENTSOURCE_CC_MASK                     = 0b00001100;
    }
}

bitflags! {
    pub struct RID_CC_CONTROL_7_0: u8 {
        const ENABLE_RD_CC_0_BIT                               = 0b10000000;
        const VCONN_ILIM500MA_BIT                              = 0b01000000;
        const EN_MICRO_USB_MODE_BIT                            = 0b00100000;
        const UFP_DFP_MODE_BIT                                 = 0b00010000;
        const VCONN_EN_CC_MASK                                 = 0b00001100;
        const VREF_SEL_RIDCC_SENSOR_MASK                       = 0b00000011;
    }
}

bitflags! {
    pub struct OTG_STATUS: u8 {
        const BOOST_SOFTSTART_DONE_BIT                         = 0b00001000;
        const OTG_STATE_MASK                                   = 0b00000111;
    }
}

bitflags! {
    pub struct OTG_INT_RT_STS_VAL: u8 {
        const TESTMODE_CHANGE_DETECT_RT_STS_BIT                = 0b00001000;
        const OTG_OC_DIS_SW_STS_RT_STS_BIT                     = 0b00000100;
        const OTG_OVERCURRENT_RT_STS_BIT                       = 0b00000010;
        const OTG_FAIL_RT_STS_BIT                              = 0b00000001;
    }
}

bitflags! {
    pub struct CMD_OTG: u8 {
        const OTG_EN_BIT                                       = 0b00000001;
    }
}

bitflags! {
    pub struct BAT_UVLO_THRESHOLD_CFG: u8 {
        const BAT_UVLO_THRESHOLD_MASK                          = 0b00000011;
    }
}

bitflags! {
    pub struct OTG_CURRENT_LIMIT_CFG: u8 {
        const OTG_CURRENT_LIMIT_MASK                           = 0b00000111;
    }
}

bitflags! {
    pub struct OTG_CFG: u8 {
        const DIS_OTG_ON_TLIM_BIT                              = 0b00100000;
        const QUICKSTART_OTG_FASTROLESWAP_BIT                  = 0b00010000;
        const INCREASE_DFP_TIME_BIT                            = 0b00001000;
        const ENABLE_OTG_IN_DEBUG_MODE_BIT                     = 0b00000100;
        const OTG_EN_SRC_CFG_BIT                               = 0b00000010;
        const CONCURRENT_MODE_CFG_BIT                          = 0b00000001;
        const OTG_RESERVED_MASK                                = 0b11000000;
    }
}

bitflags! {
    pub struct OTG_ENG_OTG_CFG: u8 {
        const ENG_BUCKBOOST_HALT1_8_MODE_BIT                   = 0b00000001;
    }
}

bitflags! {
    pub struct BATIF_INT_RT_STS_VAL: u8 {
        const BAT_7_RT_STS_BIT                                 = 0b10000000;
        const BAT_6_RT_STS_BIT                                 = 0b01000000;
        const BAT_TERMINAL_MISSING_RT_STS_BIT                  = 0b00100000;
        const BAT_THERM_OR_ID_MISSING_RT_STS_BIT               = 0b00010000;
        const BAT_LOW_RT_STS_BIT                               = 0b00001000;
        const BAT_OV_RT_STS_BIT                                = 0b00000100;
        const BAT_OCP_RT_STS_BIT                               = 0b00000010;
        const BAT_TEMP_RT_STS_BIT                              = 0b00000001;
    }
}

bitflags! {
    pub struct SHIP_MODE: u8 {
        const SHIP_MODE_EN_BIT                                 = 0b00000001;
    }
}

bitflags! {
    pub struct BATOCP_THRESHOLD_CFG: u8 {
        const BATOCP_ENABLE_CFG_BIT                            = 0b00001000;
        const BATOCP_THRESHOLD_MASK                            = 0b00000111;
    }
}

bitflags! {
    pub struct BATOCP_INTRPT_DELAY_TMR_CFG: u8 {
        const BATOCP_INTRPT_TIMEOUT_MASK                       = 0b00111000;
        const BATOCP_DELAY_TIMEOUT_MASK                        = 0b00000111;
    }
}

bitflags! {
    pub struct BATOCP_RESET_TMR_CFG: u8 {
        const EN_BATOCP_RESET_TMR_BIT                          = 0b00001000;
        const BATOCP_RESET_TIMEOUT_MASK                        = 0b00000111;
    }
}

bitflags! {
    pub struct LOW_BATT_DETECT_EN_CFG: u8 {
        const LOW_BATT_DETECT_EN_BIT                           = 0b00000001;
    }
}

bitflags! {
    pub struct LOW_BATT_THRESHOLD_CFG: u8 {
        const LOW_BATT_THRESHOLD_MASK                          = 0b00001111;
    }
}

bitflags! {
    pub struct BAT_FET_CFG: u8 {
        const BAT_FET_CFG_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct BAT_MISS_SRC_CFG: u8 {
        const BAT_MISS_ALG_EN_BIT                              = 0b00000100;
        const BAT_MISS_RESERVED_BIT                            = 0b00000010;
        const BAT_MISS_PIN_SRC_EN_BIT                          = 0b00000001;
    }
}

bitflags! {
    pub struct BAT_MISS_ALG_OPTIONS_CFG: u8 {
        const BAT_MISS_INPUT_PLUGIN_BIT                        = 0b00000100;
        const BAT_MISS_TMR_START_OPTION_BIT                    = 0b00000010;
        const BAT_MISS_POLL_EN_BIT                             = 0b00000001;
    }
}

bitflags! {
    pub struct BAT_MISS_PIN_GF_CFG: u8 {
        const BAT_MISS_PIN_GF_MASK                             = 0b00000011;
    }
}

bitflags! {
    pub struct USBIN_INPUT_STATUS: u8 {
        const USBIN_INPUT_STATUS_7_BIT                         = 0b10000000;
        const USBIN_INPUT_STATUS_6_BIT                         = 0b01000000;
        const USBIN_12V_BIT                                    = 0b00100000;
        const USBIN_9V_TO_12V_BIT                              = 0b00010000;
        const USBIN_9V_BIT                                     = 0b00001000;
        const USBIN_5V_TO_12V_BIT                              = 0b00000100;
        const USBIN_5V_TO_9V_BIT                               = 0b00000010;
        const USBIN_5V_BIT                                     = 0b00000001;
        const QC_2P0_STATUS_MASK                               = 0b00000111;
    }
}

bitflags! {
    pub struct APSD_STATUS: u8 {
        const APSD_STATUS_7_BIT                                = 0b10000000;
        const HVDCP_CHECK_TIMEOUT_BIT                          = 0b01000000;
        const SLOW_PLUGIN_TIMEOUT_BIT                          = 0b00100000;
        const ENUMERATION_DONE_BIT                             = 0b00010000;
        const VADP_CHANGE_DONE_AFTER_AUTH_BIT                  = 0b00001000;
        const QC_AUTH_DONE_STATUS_BIT                          = 0b00000100;
        const QC_CHARGER_BIT                                   = 0b00000010;
        const APSD_DTC_STATUS_DONE_BIT                         = 0b00000001;
    }
}

bitflags! {
    pub struct APSD_RESULT_STATUS: u8 {
        const ICL_OVERRIDE_LATCH_BIT                           = 0b10000000;
        const QC_3P0_BIT                                       = 0b01000000;
        const QC_2P0_BIT                                       = 0b00100000;
        const FLOAT_CHARGER_BIT                                = 0b00010000;
        const DCP_CHARGER_BIT                                  = 0b00001000;
        const CDP_CHARGER_BIT                                  = 0b00000100;
        const OCP_CHARGER_BIT                                  = 0b00000010;
        const SDP_CHARGER_BIT                                  = 0b00000001;
        const APSD_RESULT_STATUS_MASK                          = 0b01111111;
    }
}

bitflags! {
    pub struct QC_CHANGE_STATUS: u8 {
        const QC_CHANGE_STATUS_7_BIT                           = 0b10000000;
        const QC_CHANGE_STATUS_6_BIT                           = 0b01000000;
        const QC_9V_TO_12V_REASON_BIT                          = 0b00100000;
        const QC_5V_TO_9V_REASON_BIT                           = 0b00010000;
        const QC_CONTINUOUS_BIT                                = 0b00001000;
        const QC_12V_BIT                                       = 0b00000100;
        const QC_9V_BIT                                        = 0b00000010;
        const QC_5V_BIT                                        = 0b00000001;
    }
}

bitflags! {
    pub struct QC_PULSE_COUNT_STATUS: u8 {
        const QC_PULSE_COUNT_STATUS_7_BIT                      = 0b10000000;
        const QC_PULSE_COUNT_STATUS_6_BIT                      = 0b01000000;
        const QC_PULSE_COUNT_MASK                              = 0b00111111;
    }
}

bitflags! {
    pub struct TYPE_C_STATUS_1: u8 {
        const UFP_TYPEC_RDSTD_BIT                              = 0b10000000;
        const UFP_TYPEC_RD1P5_BIT                              = 0b01000000;
        const UFP_TYPEC_RD3P0_BIT                              = 0b00100000;
        const UFP_TYPEC_FMB_255K_BIT                           = 0b00010000;
        const UFP_TYPEC_FMB_301K_BIT                           = 0b00001000;
        const UFP_TYPEC_FMB_523K_BIT                           = 0b00000100;
        const UFP_TYPEC_FMB_619K_BIT                           = 0b00000010;
        const UFP_TYPEC_OPEN_OPEN_BIT                          = 0b00000001;
        const UFP_TYPEC_MASK                                   = 0b11100000;
    }
}

bitflags! {
    pub struct TYPE_C_STATUS_2: u8 {
        const DFP_RA_OPEN_BIT                                  = 0b10000000;
        const TIMER_STAGE_BIT                                  = 0b01000000;
        const EXIT_UFP_MODE_BIT                                = 0b00100000;
        const EXIT_DFP_MODE_BIT                                = 0b00010000;
        const DFP_RD_OPEN_BIT                                  = 0b00001000;
        const DFP_RD_RA_VCONN_BIT                              = 0b00000100;
        const DFP_RD_RD_BIT                                    = 0b00000010;
        const DFP_RA_RA_BIT                                    = 0b00000001;
        const DFP_TYPEC_MASK                                   = 0b00001111;
    }
}

bitflags! {
    pub struct TYPE_C_STATUS_3: u8 {
        const ENABLE_BANDGAP_BIT                               = 0b10000000;
        const U_USB_GND_NOVBUS_BIT                             = 0b01000000;
        const U_USB_FLOAT_NOVBUS_BIT                           = 0b00100000;
        const U_USB_GND_BIT                                    = 0b00010000;
        const U_USB_FMB1_BIT                                   = 0b00001000;
        const U_USB_FLOAT1_BIT                                 = 0b00000100;
        const U_USB_FMB2_BIT                                   = 0b00000010;
        const U_USB_FLOAT2_BIT                                 = 0b00000001;
    }
}

bitflags! {
    pub struct TYPE_C_STATUS_4: u8 {
        const UFP_DFP_MODE_STATUS_BIT                          = 0b10000000;
        const TYPEC_VBUS_STATUS_BIT                            = 0b01000000;
        const TYPEC_VBUS_ERROR_STATUS_BIT                      = 0b00100000;
        const TYPEC_DEBOUNCE_DONE_STATUS_BIT                   = 0b00010000;
        const TYPEC_UFP_AUDIO_ADAPT_STATUS_BIT                 = 0b00001000;
        const TYPEC_VCONN_OVERCURR_STATUS_BIT                  = 0b00000100;
        const CC_ORIENTATION_BIT                               = 0b00000010;
        const CC_ATTACHED_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct TYPE_C_STATUS_5: u8 {
        const TRY_SOURCE_FAILED_BIT                            = 0b01000000;
        const TRY_SINK_FAILED_BIT                              = 0b00100000;
        const TIMER_STAGE_2_BIT                                = 0b00010000;
        const TYPEC_LEGACY_CABLE_STATUS_BIT                    = 0b00001000;
        const TYPEC_NONCOMP_LEGACY_CABLE_STATUS_BIT            = 0b00000100;
        const TYPEC_TRYSOURCE_DETECT_STATUS_BIT                = 0b00000010;
        const TYPEC_TRYSINK_DETECT_STATUS_BIT                  = 0b00000001;
    }
}

bitflags! {
    pub struct USB_INT_RT_STS_VAL: u8 {
        const TYPE_C_CHANGE_RT_STS_BIT                         = 0b10000000;
        const USBIN_ICL_CHANGE_RT_STS_BIT                      = 0b01000000;
        const USBIN_SOURCE_CHANGE_RT_STS_BIT                   = 0b00100000;
        const USBIN_PLUGIN_RT_STS_BIT                          = 0b00010000;
        const USBIN_OV_RT_STS_BIT                              = 0b00001000;
        const USBIN_UV_RT_STS_BIT                              = 0b00000100;
        const USBIN_LT_3P6V_RT_STS_BIT                         = 0b00000010;
        const USBIN_COLLAPSE_RT_STS_BIT                        = 0b00000001;
    }
}

bitflags! {
    pub struct USBIN_CMD_IL: u8 {
        const BAT_2_SYS_FET_DIS_BIT                            = 0b00000010;
        const USBIN_SUSPEND_BIT                                = 0b00000001;
    }
}

bitflags! {
    pub struct CMD_APSD: u8 {
        const ICL_OVERRIDE_BIT                                 = 0b00000010;
        const APSD_RERUN_BIT                                   = 0b00000001;
    }
}

bitflags! {
    pub struct CMD_HVDCP_2: u8 {
        const RESTART_AICL_BIT                                 = 0b10000000;
        const TRIGGER_AICL_BIT                                 = 0b01000000;
        const FORCE_12V_BIT                                    = 0b00100000;
        const FORCE_9V_BIT                                     = 0b00010000;
        const FORCE_5V_BIT                                     = 0b00001000;
        const IDLE_BIT                                         = 0b00000100;
        const SINGLE_DECREMENT_BIT                             = 0b00000010;
        const SINGLE_INCREMENT_BIT                             = 0b00000001;
    }
}

bitflags! {
    pub struct USB_MISC2: u8 {
        const USB_MISC2_MASK                                   = 0b00000011;
    }
}

bitflags! {
    pub struct TYPE_C_CFG: u8 {
        const APSD_START_ON_CC_BIT                             = 0b10000000;
        const WAIT_FOR_APSD_BIT                                = 0b01000000;
        const FACTORY_MODE_DETECTION_EN_BIT                    = 0b00100000;
        const FACTORY_MODE_ICL_3A_4A_BIT                       = 0b00010000;
        const FACTORY_MODE_DIS_CHGING_CFG_BIT                  = 0b00001000;
        const SUSPEND_NON_COMPLIANT_CFG_BIT                    = 0b00000100;
        const VCONN_OC_CFG_BIT                                 = 0b00000010;
        const TYPE_C_OR_U_USB_BIT                              = 0b00000001;
    }
}

bitflags! {
    pub struct TYPE_C_CFG_2: u8 {
        const TYPE_C_DFP_CURRSRC_MODE_BIT                      = 0b10000000;
        const DFP_CC_1P4V_OR_1P6V_BIT                          = 0b01000000;
        const EN_TRY_SOURCE_MODE_BIT                           = 0b00001000;
        const USB_FACTORY_MODE_ENABLE_BIT                      = 0b00000100;
        const TYPE_C_UFP_MODE_BIT                              = 0b00000010;
        const EN_80UA_180UA_CUR_SOURCE_BIT                     = 0b00000001;
        const VCONN_SOFTSTART_CFG_MASK                         = 0b00110000;
    }
}

bitflags! {
    pub struct TYPE_C_CFG_3: u8 {
        const TVBUS_DEBOUNCE_BIT                               = 0b10000000;
        const TYPEC_LEGACY_CABLE_INT_EN_BIT                    = 0b01000000;
        const TYPEC_NONCOMPLIANT_LEGACY_CABLE_INT_EN_BIT       = 0b00100000;
        const TYPEC_TRYSOURCE_DETECT_INT_EN_BIT                = 0b00010000;
        const TYPEC_TRYSINK_DETECT_INT_EN_BIT                  = 0b00001000;
        const EN_TRYSINK_MODE_BIT                              = 0b00000100;
        const EN_LEGACY_CABLE_DETECTION_BIT                    = 0b00000010;
        const ALLOW_PD_DRING_UFP_TCCDB_BIT                     = 0b00000001;
    }
}

bitflags! {
    pub struct HVDCP_PULSE_COUNT_MAX: u8 {
        const HVDCP_PULSE_COUNT_MAX_QC2_MASK                   = 0b11000000;
    }
}

bitflags! {
    pub struct USBIN_ADAPTER_ALLOW_CFG: u8 {
        const USBIN_ADAPTER_ALLOW_MASK                         = 0b00001111;
    }
}

bitflags! {
    pub struct USBIN_OPTIONS_1_CFG: u8 {
        const CABLE_R_SEL_BIT                                  = 0b10000000;
        const HVDCP_AUTH_ALG_EN_CFG_BIT                        = 0b01000000;
        const HVDCP_AUTONOMOUS_MODE_EN_CFG_BIT                 = 0b00100000;
        const INPUT_PRIORITY_BIT                               = 0b00010000;
        const AUTO_SRC_DETECT_BIT                              = 0b00001000;
        const HVDCP_EN_BIT                                     = 0b00000100;
        const VADP_INCREMENT_VOLTAGE_LIMIT_BIT                 = 0b00000010;
        const VADP_TAPER_TIMER_EN_BIT                          = 0b00000001;
    }
}

bitflags! {
    pub struct USBIN_OPTIONS_2_CFG: u8 {
        const WIPWR_RST_EUD_CFG_BIT                            = 0b10000000;
        const SWITCHER_START_CFG_BIT                           = 0b01000000;
        const DCD_TIMEOUT_SEL_BIT                              = 0b00100000;
        const OCD_CURRENT_SEL_BIT                              = 0b00010000;
        const SLOW_PLUGIN_TIMER_EN_CFG_BIT                     = 0b00001000;
        const FLOAT_DIS_CHGING_CFG_BIT                         = 0b00000100;
        const SUSPEND_FLOAT_CFG_BIT                            = 0b00000010;
        const FORCE_FLOAT_SDP_CFG_BIT                          = 0b00000001;
        const FLOAT_OPTIONS_MASK                               = 0b00000111;
    }
}

bitflags! {
    pub struct TAPER_TIMER_SEL_CFG: u8 {
        const TYPEC_SPARE_CFG_BIT                              = 0b10000000;
        const TYPEC_DRP_DFP_TIME_CFG_BIT                       = 0b00100000;
        const TAPER_TIMER_SEL_MASK                             = 0b00000011;
    }
}

bitflags! {
    pub struct USBIN_LOAD_CFG: u8 {
        const USBIN_OV_CH_LOAD_OPTION_BIT                      = 0b10000000;
        const ICL_OVERRIDE_AFTER_APSD_BIT                      = 0b00010000;
    }
}

bitflags! {
    pub struct USBIN_ICL_OPTIONS: u8 {
        const CFG_USB3P0_SEL_BIT                               = 0b00000100;
        const USB51_MODE_BIT                                   = 0b00000010;
        const USBIN_MODE_CHG_BIT                               = 0b00000001;
    }
}

bitflags! {
    pub struct TYPE_C_INTRPT_ENB: u8 {
        const TYPEC_CCOUT_DETACH_INT_EN_BIT                    = 0b10000000;
        const TYPEC_CCOUT_ATTACH_INT_EN_BIT                    = 0b01000000;
        const TYPEC_VBUS_ERROR_INT_EN_BIT                      = 0b00100000;
        const TYPEC_UFP_AUDIOADAPT_INT_EN_BIT                  = 0b00010000;
        const TYPEC_DEBOUNCE_DONE_INT_EN_BIT                   = 0b00001000;
        const TYPEC_CCSTATE_CHANGE_INT_EN_BIT                  = 0b00000100;
        const TYPEC_VBUS_DEASSERT_INT_EN_BIT                   = 0b00000010;
        const TYPEC_VBUS_ASSERT_INT_EN_BIT                     = 0b00000001;
    }
}

bitflags! {
    pub struct TYPE_C_INTRPT_ENB_SOFTWARE_CTRL: u8 {
        const EXIT_SNK_BASED_ON_CC_BIT                         = 0b10000000;
        const VCONN_EN_ORIENTATION_BIT                         = 0b01000000;
        const TYPEC_VCONN_OVERCURR_INT_EN_BIT                  = 0b00100000;
        const VCONN_EN_SRC_BIT                                 = 0b00010000;
        const VCONN_EN_VALUE_BIT                               = 0b00001000;
        const UFP_EN_CMD_BIT                                   = 0b00000100;
        const DFP_EN_CMD_BIT                                   = 0b00000010;
        const TYPEC_DISABLE_CMD_BIT                            = 0b00000001;
        const TYPEC_POWER_ROLE_CMD_MASK                        = 0b00000111;
    }
}

bitflags! {
    pub struct USBIN_SOURCE_CHANGE_INTRPT_ENB: u8 {
        const SLOW_IRQ_EN_CFG_BIT                              = 0b00100000;
        const ENUMERATION_IRQ_EN_CFG_BIT                       = 0b00010000;
        const VADP_IRQ_EN_CFG_BIT                              = 0b00001000;
        const AUTH_IRQ_EN_CFG_BIT                              = 0b00000100;
        const HVDCP_IRQ_EN_CFG_BIT                             = 0b00000010;
        const APSD_IRQ_EN_CFG_BIT                              = 0b00000001;
    }
}

bitflags! {
    pub struct USBIN_CURRENT_LIMIT_CFG: u8 {
        const USBIN_CURRENT_LIMIT_MASK                         = 0b11111111;
    }
}

bitflags! {
    pub struct USBIN_AICL_OPTIONS_CFG: u8 {
        const SUSPEND_ON_COLLAPSE_USBIN_BIT                    = 0b10000000;
        const USBIN_AICL_HDC_EN_BIT                            = 0b01000000;
        const USBIN_AICL_START_AT_MAX_BIT                      = 0b00100000;
        const USBIN_AICL_RERUN_EN_BIT                          = 0b00010000;
        const USBIN_AICL_ADC_EN_BIT                            = 0b00001000;
        const USBIN_AICL_EN_BIT                                = 0b00000100;
        const USBIN_HV_COLLAPSE_RESPONSE_BIT                   = 0b00000010;
        const USBIN_LV_COLLAPSE_RESPONSE_BIT                   = 0b00000001;
    }
}

bitflags! {
    pub struct USBIN_5V_AICL_THRESHOLD_CFG: u8 {
        const USBIN_5V_AICL_THRESHOLD_CFG_MASK                 = 0b00000111;
    }
}

bitflags! {
    pub struct USBIN_9V_AICL_THRESHOLD_CFG: u8 {
        const USBIN_9V_AICL_THRESHOLD_CFG_MASK                 = 0b00000111;
    }
}

bitflags! {
    pub struct USBIN_12V_AICL_THRESHOLD_CFG: u8 {
        const USBIN_12V_AICL_THRESHOLD_CFG_MASK                = 0b00000111;
    }
}

bitflags! {
    pub struct USBIN_CONT_AICL_THRESHOLD_CFG: u8 {
        const USBIN_CONT_AICL_THRESHOLD_CFG_MASK               = 0b00111111;
    }
}

bitflags! {
    pub struct DCIN_INPUT_STATUS: u8 {
        const DCIN_INPUT_STATUS_7_BIT                          = 0b10000000;
        const DCIN_INPUT_STATUS_6_BIT                          = 0b01000000;
        const DCIN_12V_BIT                                     = 0b00100000;
        const DCIN_9V_TO_12V_BIT                               = 0b00010000;
        const DCIN_9V_BIT                                      = 0b00001000;
        const DCIN_5V_TO_12V_BIT                               = 0b00000100;
        const DCIN_5V_TO_9V_BIT                                = 0b00000010;
        const DCIN_5V_BIT                                      = 0b00000001;
    }
}

bitflags! {
    pub struct WIPWR_STATUS: u8 {
        const WIPWR_STATUS_7_BIT                               = 0b10000000;
        const WIPWR_STATUS_6_BIT                               = 0b01000000;
        const WIPWR_STATUS_5_BIT                               = 0b00100000;
        const DCIN_WIPWR_OV_DG_BIT                             = 0b00010000;
        const DIV2_EN_DG_BIT                                   = 0b00001000;
        const SHUTDOWN_N_LATCH_BIT                             = 0b00000100;
        const CHG_OK_PIN_BIT                                   = 0b00000010;
        const WIPWR_CHARGING_ENABLED_BIT                       = 0b00000001;
    }
}

bitflags! {
    pub struct WIPWR_RANGE_STATUS: u8 {
        const WIPWR_RANGE_STATUS_MASK                          = 0b00011111;
    }
}

bitflags! {
    pub struct DCIN_INT_RT_STS_VAL: u8 {
        const WIPWR_VOLTAGE_RANGE_RT_STS_BIT                   = 0b10000000;
        const DCIN_ICL_CHANGE_RT_STS_BIT                       = 0b01000000;
        const DIV2_EN_DG_RT_STS_BIT                            = 0b00100000;
        const DCIN_PLUGIN_RT_STS_BIT                           = 0b00010000;
        const DCIN_OV_RT_STS_BIT                               = 0b00001000;
        const DCIN_UV_RT_STS_BIT                               = 0b00000100;
        const DCIN_LT_3P6V_RT_STS_BIT                          = 0b00000010;
        const DCIN_COLLAPSE_RT_STS_BIT                         = 0b00000001;
    }
}

bitflags! {
    pub struct DCIN_CMD_IL: u8 {
        const WIRELESS_CHG_DIS_BIT                             = 0b00001000;
        const SHDN_N_CLEAR_CMD_BIT                             = 0b00000100;
        const SHDN_N_SET_CMD_BIT                               = 0b00000010;
        const DCIN_SUSPEND_BIT                                 = 0b00000001;
    }
}

bitflags! {
    pub struct DC_SPARE: u8 {
        const DC_SPARE_MASK                                    = 0b00001111;
    }
}

bitflags! {
    pub struct DCIN_ADAPTER_ALLOW_CFG: u8 {
        const DCIN_ADAPTER_ALLOW_MASK                          = 0b00001111;
    }
}

bitflags! {
    pub struct DCIN_LOAD_CFG: u8 {
        const DCIN_OV_CH_LOAD_OPTION_BIT                       = 0b10000000;
    }
}

bitflags! {
    pub struct DCIN_CURRENT_LIMIT_CFG: u8 {
        const DCIN_CURRENT_LIMIT_MASK                          = 0b11111111;
    }
}

bitflags! {
    pub struct DCIN_AICL_OPTIONS_CFG: u8 {
        const SUSPEND_ON_COLLAPSE_DCIN_BIT                     = 0b10000000;
        const DCIN_AICL_HDC_EN_BIT                             = 0b01000000;
        const DCIN_AICL_START_AT_MAX_BIT                       = 0b00100000;
        const DCIN_AICL_RERUN_EN_BIT                           = 0b00010000;
        const DCIN_AICL_ADC_EN_BIT                             = 0b00001000;
        const DCIN_AICL_EN_BIT                                 = 0b00000100;
        const DCIN_HV_COLLAPSE_RESPONSE_BIT                    = 0b00000010;
        const DCIN_LV_COLLAPSE_RESPONSE_BIT                    = 0b00000001;
    }
}

bitflags! {
    pub struct DCIN_AICL_REF_SEL_CFG: u8 {
        const DCIN_CONT_AICL_THRESHOLD_CFG_MASK                = 0b00111111;
    }
}

bitflags! {
    pub struct DCIN_ICL_START_CFG: u8 {
        const DCIN_ICL_START_CFG_BIT                           = 0b00000001;
    }
}

bitflags! {
    pub struct DIV2_EN_GF_TIME_CFG: u8 {
        const DIV2_EN_GF_TIME_CFG_MASK                         = 0b00000011;
    }
}

bitflags! {
    pub struct WIPWR_IRQ_TMR_CFG: u8 {
        const WIPWR_IRQ_TMR_MASK                               = 0b00000111;
    }
}

bitflags! {
    pub struct ZIN_ICL_PT: u8 {
        const ZIN_ICL_PT_MASK                                  = 0b11111111;
    }
}

bitflags! {
    pub struct ZIN_ICL_LV: u8 {
        const ZIN_ICL_LV_MASK                                  = 0b11111111;
    }
}

bitflags! {
    pub struct ZIN_ICL_HV: u8 {
        const ZIN_ICL_HV_MASK                                  = 0b11111111;
    }
}

bitflags! {
    pub struct WI_PWR_OPTIONS: u8 {
        const CHG_OK_BIT                                       = 0b10000000;
        const WIPWR_UVLO_IRQ_OPT_BIT                           = 0b01000000;
        const BUCK_HOLDOFF_ENABLE_BIT                          = 0b00100000;
        const CHG_OK_HW_SW_SELECT_BIT                          = 0b00010000;
        const WIPWR_RST_ENABLE_BIT                             = 0b00001000;
        const DCIN_WIPWR_IRQ_SELECT_BIT                        = 0b00000100;
        const AICL_SWITCH_ENABLE_BIT                           = 0b00000010;
        const ZIN_ICL_ENABLE_BIT                               = 0b00000001;
    }
}

bitflags! {
    pub struct ZIN_ICL_PT_HV: u8 {
        const ZIN_ICL_PT_HV_MASK                               = 0b11111111;
    }
}

bitflags! {
    pub struct ZIN_ICL_MID_LV: u8 {
        const ZIN_ICL_MID_LV_MASK                              = 0b11111111;
    }
}

bitflags! {
    pub struct ZIN_ICL_MID_HV: u8 {
        const ZIN_ICL_MID_HV_MASK                              = 0b11111111;
    }
}

bitflags! {
    pub struct DC_ENG_SSUPPLY_CFG2: u8 {
        const ENG_SSUPPLY_IVREF_OTG_SS_MASK                    = 0b00000111;
    }
}

bitflags! {
    pub struct DC_ENG_SSUPPLY_CFG3: u8 {
        const ENG_SSUPPLY_HI_CAP_BIT                           = 0b01000000;
        const ENG_SSUPPLY_HI_RES_BIT                           = 0b00100000;
        const ENG_SSUPPLY_CFG_SKIP_TH_V0P2_BIT                 = 0b00001000;
        const ENG_SSUPPLY_CFG_SYSOV_TH_4P8_BIT                 = 0b00000100;
        const ENG_SSUPPLY_5V_OV_OPT_BIT                        = 0b00000001;
    }
}

bitflags! {
    pub struct REVISION1: u8 {
        const DIG_MINOR_MASK                                   = 0b11111111;
    }
}

bitflags! {
    pub struct REVISION2: u8 {
        const DIG_MAJOR_MASK                                   = 0b11111111;
    }
}

bitflags! {
    pub struct REVISION3: u8 {
        const ANA_MINOR_MASK                                   = 0b11111111;
    }
}

bitflags! {
    pub struct REVISION4: u8 {
        const ANA_MAJOR_MASK                                   = 0b11111111;
    }
}

bitflags! {
    pub struct TEMP_RANGE_STATUS: u8 {
        const TEMP_RANGE_STATUS_7_BIT                          = 0b10000000;
        const THERM_REG_ACTIVE_BIT                             = 0b01000000;
        const TLIM_BIT                                         = 0b00100000;
        const ALERT_LEVEL_BIT                                  = 0b00010000;
        const TEMP_ABOVE_RANGE_BIT                             = 0b00001000;
        const TEMP_WITHIN_RANGE_BIT                            = 0b00000100;
        const TEMP_BELOW_RANGE_BIT                             = 0b00000010;
        const THERMREG_DISABLED_BIT                            = 0b00000001;
        const TEMP_RANGE_MASK                                  = 0b00011110;
    }
}

bitflags! {
    pub struct ICL_STATUS: u8 {
        const INPUT_CURRENT_LIMIT_MASK                         = 0b11111111;
    }
}

bitflags! {
    pub struct ADAPTER_5V_ICL_STATUS: u8 {
        const ADAPTER_5V_ICL_MASK                              = 0b11111111;
    }
}

bitflags! {
    pub struct ADAPTER_9V_ICL_STATUS: u8 {
        const ADAPTER_9V_ICL_MASK                              = 0b11111111;
    }
}

bitflags! {
    pub struct AICL_STATUS: u8 {
        const AICL_STATUS_7_BIT                                = 0b10000000;
        const SOFT_ILIMIT_BIT                                  = 0b01000000;
        const HIGHEST_DC_BIT                                   = 0b00100000;
        const USBIN_CH_COLLAPSE_BIT                            = 0b00010000;
        const DCIN_CH_COLLAPSE_BIT                             = 0b00001000;
        const ICL_IMIN_BIT                                     = 0b00000100;
        const AICL_FAIL_BIT                                    = 0b00000010;
        const AICL_DONE_BIT                                    = 0b00000001;
    }
}

bitflags! {
    pub struct POWER_PATH_STATUS: u8 {
        const INPUT_SS_DONE_BIT                                = 0b10000000;
        const USBIN_SUSPEND_STS_BIT                            = 0b01000000;
        const DCIN_SUSPEND_STS_BIT                             = 0b00100000;
        const USE_USBIN_BIT                                    = 0b00010000;
        const USE_DCIN_BIT                                     = 0b00001000;
        const VALID_INPUT_POWER_SOURCE_STS_BIT                 = 0b00000001;
        const POWER_PATH_MASK                                  = 0b00000110;
    }
}

bitflags! {
    pub struct WDOG_STATUS: u8 {
        const WDOG_STATUS_7_BIT                                = 0b10000000;
        const WDOG_STATUS_6_BIT                                = 0b01000000;
        const WDOG_STATUS_5_BIT                                = 0b00100000;
        const WDOG_STATUS_4_BIT                                = 0b00010000;
        const WDOG_STATUS_3_BIT                                = 0b00001000;
        const WDOG_STATUS_2_BIT                                = 0b00000100;
        const WDOG_STATUS_1_BIT                                = 0b00000010;
        const BARK_BITE_STATUS_BIT                             = 0b00000001;
    }
}

bitflags! {
    pub struct SYSOK_REASON_STATUS: u8 {
        const SYSOK_REASON_DCIN_BIT                            = 0b00000010;
        const SYSOK_REASON_USBIN_BIT                           = 0b00000001;
    }
}

bitflags! {
    pub struct MISC_INT_RT_STS_VAL: u8 {
        const SWITCHER_POWER_OK_RT_STS_BIT                     = 0b10000000;
        const TEMPERATURE_CHANGE_RT_STS_BIT                    = 0b01000000;
        const INPUT_CURRENT_LIMITING_RT_STS_BIT                = 0b00100000;
        const HIGH_DUTY_CYCLE_RT_STS_BIT                       = 0b00010000;
        const AICL_DONE_RT_STS_BIT                             = 0b00001000;
        const AICL_FAIL_RT_STS_BIT                             = 0b00000100;
        const WDOG_BARK_RT_STS_BIT                             = 0b00000010;
        const WDOG_SNARL_RT_STS_BIT                            = 0b00000001;
    }
}

bitflags! {
    pub struct WDOG_RST: u8 {
        const WDOG_RST_BIT                                     = 0b00000001;
    }
}

bitflags! {
    pub struct AFP_MODE: u8 {
        const AFP_MODE_EN_BIT                                  = 0b00000001;
    }
}

bitflags! {
    pub struct GSM_PA_ON_ADJ_EN: u8 {
        const GSM_PA_ON_ADJ_EN_BIT                             = 0b00000001;
    }
}

bitflags! {
    pub struct BARK_BITE_WDOG_PET: u8 {
        const BARK_BITE_WDOG_PET_BIT                           = 0b00000001;
    }
}

bitflags! {
    pub struct PHYON_CMD: u8 {
        const PHYON_CMD_BIT                                    = 0b00000001;
    }
}

bitflags! {
    pub struct SHDN_CMD: u8 {
        const SHDN_CMD_BIT                                     = 0b00000001;
    }
}

bitflags! {
    pub struct FINISH_COPY_COMMAND: u8 {
        const START_COPY_BIT                                   = 0b00000001;
    }
}

bitflags! {
    pub struct WD_CFG: u8 {
        const WATCHDOG_TRIGGER_AFP_EN_BIT                      = 0b10000000;
        const BARK_WDOG_INT_EN_BIT                             = 0b01000000;
        const BITE_WDOG_INT_EN_BIT                             = 0b00100000;
        const WDOG_IRQ_SFT_BIT                                 = 0b00000100;
        const WDOG_TIMER_EN_ON_PLUGIN_BIT                      = 0b00000010;
        const WDOG_TIMER_EN_BIT                                = 0b00000001;
        const SFT_AFTER_WDOG_IRQ_MASK                          = 0b00011000;
    }
}

bitflags! {
    pub struct MISC_CFG: u8 {
        const GSM_PA_ON_ADJ_SEL_BIT                            = 0b00000001;
        const STAT_PARALLEL_1400MA_EN_CFG_BIT                  = 0b00001000;
        const TCC_DEBOUNCE_20MS_BIT                            = 0b00100000;
    }
}

bitflags! {
    pub struct SNARL_BARK_BITE_WD_CFG: u8 {
        const BITE_WDOG_DISABLE_CHARGING_CFG_BIT               = 0b10000000;
        const SNARL_WDOG_TIMEOUT_MASK                          = 0b01110000;
        const BARK_WDOG_TIMEOUT_MASK                           = 0b00001100;
        const BITE_WDOG_TIMEOUT_MASK                           = 0b00000011;
    }
}

bitflags! {
    pub struct PHYON_CFG: u8 {
        const USBPHYON_PUSHPULL_CFG_BIT                        = 0b00000010;
        const PHYON_SW_SEL_BIT                                 = 0b00000001;
    }
}

bitflags! {
    pub struct CHGR_TRIM_OPTIONS_7_0: u8 {
        const TLIM_DIS_TBIT_BIT                                = 0b00000001;
    }
}

bitflags! {
    pub struct CH_OV_OPTION_CFG: u8 {
        const OV_OPTION_TBIT_BIT                               = 0b00000001;
    }
}

bitflags! {
    pub struct AICL_CFG: u8 {
        const TREG_ALLOW_DECREASE_BIT                          = 0b00000010;
        const AICL_HIGH_DC_INC_BIT                             = 0b00000001;
    }
}

bitflags! {
    pub struct AICL_RERUN_TIME_CFG: u8 {
        const AICL_RERUN_TIME_MASK                             = 0b00000011;
    }
}

bitflags! {
    pub struct AICL_RERUN_TEMP_TIME_CFG: u8 {
        const AICL_RERUN_TEMP_TIME_MASK                        = 0b00000011;
    }
}

bitflags! {
    pub struct THERMREG_SRC_CFG: u8 {
        const SKIN_ADC_CFG_BIT                                 = 0b00001000;
        const THERMREG_SKIN_ADC_SRC_EN_BIT                     = 0b00000100;
        const THERMREG_DIE_ADC_SRC_EN_BIT                      = 0b00000010;
        const THERMREG_DIE_CMP_SRC_EN_BIT                      = 0b00000001;
    }
}

bitflags! {
    pub struct TREG_DIE_CMP_INC_CYCLE_TIME_CFG: u8 {
        const TREG_DIE_CMP_INC_CYCLE_TIME_MASK                 = 0b00000011;
    }
}

bitflags! {
    pub struct TREG_DIE_CMP_DEC_CYCLE_TIME_CFG: u8 {
        const TREG_DIE_CMP_DEC_CYCLE_TIME_MASK                 = 0b00000011;
    }
}

bitflags! {
    pub struct TREG_DIE_ADC_INC_CYCLE_TIME_CFG: u8 {
        const TREG_DIE_ADC_INC_CYCLE_TIME_MASK                 = 0b00000011;
    }
}

bitflags! {
    pub struct TREG_DIE_ADC_DEC_CYCLE_TIME_CFG: u8 {
        const TREG_DIE_ADC_DEC_CYCLE_TIME_MASK                 = 0b00000011;
    }
}

bitflags! {
    pub struct TREG_SKIN_ADC_INC_CYCLE_TIME_CFG: u8 {
        const TREG_SKIN_ADC_INC_CYCLE_TIME_MASK                = 0b00000011;
    }
}

bitflags! {
    pub struct TREG_SKIN_ADC_DEC_CYCLE_TIME_CFG: u8 {
        const TREG_SKIN_ADC_DEC_CYCLE_TIME_MASK                = 0b00000011;
    }
}

bitflags! {
    pub struct BUCK_OPTIONS_CFG: u8 {
        const CHG_EN_PIN_SUSPEND_CFG_BIT                       = 0b01000000;
        const INPUT_CURRENT_LIMIT_SOFTSTART_EN_BIT             = 0b00001000;
        const HV_HIGH_DUTY_CYCLE_PROTECT_EN_BIT                = 0b00000100;
        const BUCK_OC_PROTECT_EN_BIT                           = 0b00000010;
        const INPUT_MISS_POLL_EN_BIT                           = 0b00000001;
        const HICCUP_OPTIONS_MASK                              = 0b00110000;
    }
}

bitflags! {
    pub struct ICL_SOFTSTART_RATE_CFG: u8 {
        const ICL_SOFTSTART_RATE_MASK                          = 0b00000011;
    }
}

bitflags! {
    pub struct ICL_SOFTSTOP_RATE_CFG: u8 {
        const ICL_SOFTSTOP_RATE_MASK                           = 0b00000011;
    }
}

bitflags! {
    pub struct VSYS_MIN_SEL_CFG: u8 {
        const VSYS_MIN_SEL_MASK                                = 0b00000011;
    }
}

bitflags! {
    pub struct TRACKING_VOLTAGE_SEL_CFG: u8 {
        const TRACKING_VOLTAGE_SEL_BIT                         = 0b00000001;
    }
}

bitflags! {
    pub struct STAT_CFG: u8 {
        const STAT_SW_OVERRIDE_VALUE_BIT                       = 0b10000000;
        const STAT_SW_OVERRIDE_CFG_BIT                         = 0b01000000;
        const STAT_POLARITY_CFG_BIT                            = 0b00001000;
        const STAT_PARALLEL_CFG_BIT                            = 0b00000100;
        const STAT_FUNCTION_CFG_BIT                            = 0b00000010;
        const STAT_IRQ_PULSING_EN_BIT                          = 0b00000001;
        const STAT_PARALLEL_OFF_DG_CFG_MASK                    = 0b00110000;
    }
}

bitflags! {
    pub struct LBC_EN_CFG: u8 {
        const LBC_DURING_CHARGING_CFG_BIT                      = 0b00000010;
        const LBC_EN_BIT                                       = 0b00000001;
    }
}

bitflags! {
    pub struct LBC_PERIOD_CFG: u8 {
        const LBC_PERIOD_MASK                                  = 0b00000111;
    }
}

bitflags! {
    pub struct LBC_DUTY_CYCLE_CFG: u8 {
        const LBC_DUTY_CYCLE_MASK                              = 0b00000111;
    }
}

bitflags! {
    pub struct SYSOK_CFG: u8 {
        const SYSOK_PUSHPULL_CFG_BIT                           = 0b00100000;
        const SYSOK_B_OR_C_SEL_BIT                             = 0b00010000;
        const SYSOK_POL_BIT                                    = 0b00001000;
        const SYSOK_OPTIONS_MASK                               = 0b00000111;
    }
}

bitflags! {
    pub struct ENG_SDCDC_CFG7: u8 {
        const ENG_SDCDC_BST_SET_POINT_MASK                     = 0b11000000;
    }
}

pub fn register_flags(addr: u16, value: u8) -> Option<Box<dyn fmt::Debug>> {
    match addr {
        6400 => Some(Box::new(CHGR_FREQ_BASE_VAL::from_bits_truncate(value))),
        4102 => Some(Box::new(BATTERY_CHARGER_STATUS_1::from_bits_truncate(value))),
        4103 => Some(Box::new(BATTERY_CHARGER_STATUS_2::from_bits_truncate(value))),
        4104 => Some(Box::new(CHG_OPTION::from_bits_truncate(value))),
        4105 => Some(Box::new(BATTERY_CHARGER_STATUS_3::from_bits_truncate(value))),
        4106 => Some(Box::new(BATTERY_CHARGER_STATUS_4::from_bits_truncate(value))),
        4107 => Some(Box::new(BATTERY_CHARGER_STATUS_5::from_bits_truncate(value))),
        4108 => Some(Box::new(BATTERY_CHARGER_STATUS_6::from_bits_truncate(value))),
        4109 => Some(Box::new(BATTERY_CHARGER_STATUS_7::from_bits_truncate(value))),
        4110 => Some(Box::new(BATTERY_CHARGER_STATUS_8::from_bits_truncate(value))),
        4160 => Some(Box::new(STEP_CHG_SOC_VBATT_V::from_bits_truncate(value))),
        4161 => Some(Box::new(STEP_CHG_SOC_VBATT_V_UPDATE::from_bits_truncate(value))),
        4162 => Some(Box::new(CHARGING_ENABLE_CMD::from_bits_truncate(value))),
        4163 => Some(Box::new(ALLOW_FAST_CHARGING_CMD::from_bits_truncate(value))),
        4164 => Some(Box::new(QNOVO_PT_ENABLE_CMD::from_bits_truncate(value))),
        4176 => Some(Box::new(CHGR_CFG1::from_bits_truncate(value))),
        4177 => Some(Box::new(CHGR_CFG2::from_bits_truncate(value))),
        4178 => Some(Box::new(CHARGER_ENABLE_CFG::from_bits_truncate(value))),
        4179 => Some(Box::new(CFG::from_bits_truncate(value))),
        4180 => Some(Box::new(CHARGER_SPARE::from_bits_truncate(value))),
        4192 => Some(Box::new(PRE_CHARGE_CURRENT_CFG::from_bits_truncate(value))),
        4193 => Some(Box::new(FAST_CHARGE_CURRENT_CFG::from_bits_truncate(value))),
        4194 => Some(Box::new(CHARGE_CURRENT_TERMINATION_CFG::from_bits_truncate(value))),
        4195 => Some(Box::new(TCCC_CHARGE_CURRENT_TERMINATION_CFG::from_bits_truncate(value))),
        4196 => Some(Box::new(CHARGE_CURRENT_SOFTSTART_SETTING_CFG::from_bits_truncate(value))),
        4208 => Some(Box::new(FLOAT_VOLTAGE_CFG::from_bits_truncate(value))),
        4209 => Some(Box::new(AUTO_FLOAT_VOLTAGE_COMPENSATION_CFG::from_bits_truncate(value))),
        4210 => Some(Box::new(CHARGE_INHIBIT_THRESHOLD_CFG::from_bits_truncate(value))),
        4211 => Some(Box::new(RECHARGE_THRESHOLD_CFG::from_bits_truncate(value))),
        4212 => Some(Box::new(PRE_TO_FAST_CHARGE_THRESHOLD_CFG::from_bits_truncate(value))),
        4213 => Some(Box::new(FV_HYSTERESIS_CFG::from_bits_truncate(value))),
        4224 => Some(Box::new(FVC_CHARGE_INHIBIT_THRESHOLD_CFG::from_bits_truncate(value))),
        4225 => Some(Box::new(FVC_RECHARGE_THRESHOLD_CFG::from_bits_truncate(value))),
        4226 => Some(Box::new(FVC_PRE_TO_FAST_CHARGE_THRESHOLD_CFG::from_bits_truncate(value))),
        4227 => Some(Box::new(FVC_FULL_ON_THRESHOLD_CFG::from_bits_truncate(value))),
        4228 => Some(Box::new(FVC_CC_MODE_GLITCH_FILTER_SEL_CFG::from_bits_truncate(value))),
        4229 => Some(Box::new(FVC_TERMINATION_GLITCH_FILTER_SEL_CFG::from_bits_truncate(value))),
        4240 => Some(Box::new(JEITA_EN_CFG::from_bits_truncate(value))),
        4241 => Some(Box::new(JEITA_FVCOMP_CFG::from_bits_truncate(value))),
        4242 => Some(Box::new(JEITA_CCCOMP_CFG::from_bits_truncate(value))),
        4214 => Some(Box::new(FV_CAL_CFG::from_bits_truncate(value))),
        4215 => Some(Box::new(FV_ADJUST::from_bits_truncate(value))),
        4216 => Some(Box::new(FG_VADC_DISQ_THRESH::from_bits_truncate(value))),
        4217 => Some(Box::new(FG_IADC_DISQ_THRESH::from_bits_truncate(value))),
        4218 => Some(Box::new(FG_UPDATE_CFG_1::from_bits_truncate(value))),
        4219 => Some(Box::new(FG_UPDATE_CFG_1_SEL::from_bits_truncate(value))),
        4220 => Some(Box::new(FG_UPDATE_CFG_2::from_bits_truncate(value))),
        4221 => Some(Box::new(FG_UPDATE_CFG_2_SEL::from_bits_truncate(value))),
        4222 => Some(Box::new(FG_CHG_INTERFACE_CFG::from_bits_truncate(value))),
        4223 => Some(Box::new(FG_CHG_INTERFACE_CFG_SEL::from_bits_truncate(value))),
        4272 => Some(Box::new(CHGR_STEP_CHG_MODE_CFG::from_bits_truncate(value))),
        4273 => Some(Box::new(STEP_CHG_UPDATE_REQUEST_TIMEOUT_CFG::from_bits_truncate(value))),
        4274 => Some(Box::new(STEP_CHG_UPDATE_FAIL_TIMEOUT_CFG::from_bits_truncate(value))),
        4358 => Some(Box::new(RID_CC_CONTROL_23_16::from_bits_truncate(value))),
        4359 => Some(Box::new(RID_CC_CONTROL_15_8::from_bits_truncate(value))),
        4360 => Some(Box::new(RID_CC_CONTROL_7_0::from_bits_truncate(value))),
        4361 => Some(Box::new(OTG_STATUS::from_bits_truncate(value))),
        4368 => Some(Box::new(OTG_INT_RT_STS_VAL::from_bits_truncate(value))),
        4416 => Some(Box::new(CMD_OTG::from_bits_truncate(value))),
        4433 => Some(Box::new(BAT_UVLO_THRESHOLD_CFG::from_bits_truncate(value))),
        4434 => Some(Box::new(OTG_CURRENT_LIMIT_CFG::from_bits_truncate(value))),
        4435 => Some(Box::new(OTG_CFG::from_bits_truncate(value))),
        4544 => Some(Box::new(OTG_ENG_OTG_CFG::from_bits_truncate(value))),
        4624 => Some(Box::new(BATIF_INT_RT_STS_VAL::from_bits_truncate(value))),
        4672 => Some(Box::new(SHIP_MODE::from_bits_truncate(value))),
        4688 => Some(Box::new(BATOCP_THRESHOLD_CFG::from_bits_truncate(value))),
        4689 => Some(Box::new(BATOCP_INTRPT_DELAY_TMR_CFG::from_bits_truncate(value))),
        4690 => Some(Box::new(BATOCP_RESET_TMR_CFG::from_bits_truncate(value))),
        4704 => Some(Box::new(LOW_BATT_DETECT_EN_CFG::from_bits_truncate(value))),
        4705 => Some(Box::new(LOW_BATT_THRESHOLD_CFG::from_bits_truncate(value))),
        4706 => Some(Box::new(BAT_FET_CFG::from_bits_truncate(value))),
        4720 => Some(Box::new(BAT_MISS_SRC_CFG::from_bits_truncate(value))),
        4721 => Some(Box::new(BAT_MISS_ALG_OPTIONS_CFG::from_bits_truncate(value))),
        4722 => Some(Box::new(BAT_MISS_PIN_GF_CFG::from_bits_truncate(value))),
        4870 => Some(Box::new(USBIN_INPUT_STATUS::from_bits_truncate(value))),
        4871 => Some(Box::new(APSD_STATUS::from_bits_truncate(value))),
        4872 => Some(Box::new(APSD_RESULT_STATUS::from_bits_truncate(value))),
        4873 => Some(Box::new(QC_CHANGE_STATUS::from_bits_truncate(value))),
        4874 => Some(Box::new(QC_PULSE_COUNT_STATUS::from_bits_truncate(value))),
        4875 => Some(Box::new(TYPE_C_STATUS_1::from_bits_truncate(value))),
        4876 => Some(Box::new(TYPE_C_STATUS_2::from_bits_truncate(value))),
        4877 => Some(Box::new(TYPE_C_STATUS_3::from_bits_truncate(value))),
        4878 => Some(Box::new(TYPE_C_STATUS_4::from_bits_truncate(value))),
        4879 => Some(Box::new(TYPE_C_STATUS_5::from_bits_truncate(value))),
        4880 => Some(Box::new(USB_INT_RT_STS_VAL::from_bits_truncate(value))),
        4928 => Some(Box::new(USBIN_CMD_IL::from_bits_truncate(value))),
        4929 => Some(Box::new(CMD_APSD::from_bits_truncate(value))),
        4931 => Some(Box::new(CMD_HVDCP_2::from_bits_truncate(value))),
        4951 => Some(Box::new(USB_MISC2::from_bits_truncate(value))),
        4952 => Some(Box::new(TYPE_C_CFG::from_bits_truncate(value))),
        4953 => Some(Box::new(TYPE_C_CFG_2::from_bits_truncate(value))),
        4954 => Some(Box::new(TYPE_C_CFG_3::from_bits_truncate(value))),
        4955 => Some(Box::new(HVDCP_PULSE_COUNT_MAX::from_bits_truncate(value))),
        4960 => Some(Box::new(USBIN_ADAPTER_ALLOW_CFG::from_bits_truncate(value))),
        4962 => Some(Box::new(USBIN_OPTIONS_1_CFG::from_bits_truncate(value))),
        4963 => Some(Box::new(USBIN_OPTIONS_2_CFG::from_bits_truncate(value))),
        4964 => Some(Box::new(TAPER_TIMER_SEL_CFG::from_bits_truncate(value))),
        4965 => Some(Box::new(USBIN_LOAD_CFG::from_bits_truncate(value))),
        4966 => Some(Box::new(USBIN_ICL_OPTIONS::from_bits_truncate(value))),
        4967 => Some(Box::new(TYPE_C_INTRPT_ENB::from_bits_truncate(value))),
        4968 => Some(Box::new(TYPE_C_INTRPT_ENB_SOFTWARE_CTRL::from_bits_truncate(value))),
        4969 => Some(Box::new(USBIN_SOURCE_CHANGE_INTRPT_ENB::from_bits_truncate(value))),
        4976 => Some(Box::new(USBIN_CURRENT_LIMIT_CFG::from_bits_truncate(value))),
        4992 => Some(Box::new(USBIN_AICL_OPTIONS_CFG::from_bits_truncate(value))),
        4993 => Some(Box::new(USBIN_5V_AICL_THRESHOLD_CFG::from_bits_truncate(value))),
        4994 => Some(Box::new(USBIN_9V_AICL_THRESHOLD_CFG::from_bits_truncate(value))),
        4995 => Some(Box::new(USBIN_12V_AICL_THRESHOLD_CFG::from_bits_truncate(value))),
        4996 => Some(Box::new(USBIN_CONT_AICL_THRESHOLD_CFG::from_bits_truncate(value))),
        5126 => Some(Box::new(DCIN_INPUT_STATUS::from_bits_truncate(value))),
        5127 => Some(Box::new(WIPWR_STATUS::from_bits_truncate(value))),
        5128 => Some(Box::new(WIPWR_RANGE_STATUS::from_bits_truncate(value))),
        5136 => Some(Box::new(DCIN_INT_RT_STS_VAL::from_bits_truncate(value))),
        5184 => Some(Box::new(DCIN_CMD_IL::from_bits_truncate(value))),
        5208 => Some(Box::new(DC_SPARE::from_bits_truncate(value))),
        5216 => Some(Box::new(DCIN_ADAPTER_ALLOW_CFG::from_bits_truncate(value))),
        5221 => Some(Box::new(DCIN_LOAD_CFG::from_bits_truncate(value))),
        5232 => Some(Box::new(DCIN_CURRENT_LIMIT_CFG::from_bits_truncate(value))),
        5248 => Some(Box::new(DCIN_AICL_OPTIONS_CFG::from_bits_truncate(value))),
        5249 => Some(Box::new(DCIN_AICL_REF_SEL_CFG::from_bits_truncate(value))),
        5250 => Some(Box::new(DCIN_ICL_START_CFG::from_bits_truncate(value))),
        5264 => Some(Box::new(DIV2_EN_GF_TIME_CFG::from_bits_truncate(value))),
        5265 => Some(Box::new(WIPWR_IRQ_TMR_CFG::from_bits_truncate(value))),
        5266 => Some(Box::new(ZIN_ICL_PT::from_bits_truncate(value))),
        5267 => Some(Box::new(ZIN_ICL_LV::from_bits_truncate(value))),
        5268 => Some(Box::new(ZIN_ICL_HV::from_bits_truncate(value))),
        5269 => Some(Box::new(WI_PWR_OPTIONS::from_bits_truncate(value))),
        5270 => Some(Box::new(ZIN_ICL_PT_HV::from_bits_truncate(value))),
        5271 => Some(Box::new(ZIN_ICL_MID_LV::from_bits_truncate(value))),
        5272 => Some(Box::new(ZIN_ICL_MID_HV::from_bits_truncate(value))),
        5313 => Some(Box::new(DC_ENG_SSUPPLY_CFG2::from_bits_truncate(value))),
        5314 => Some(Box::new(DC_ENG_SSUPPLY_CFG3::from_bits_truncate(value))),
        5632 => Some(Box::new(REVISION1::from_bits_truncate(value))),
        5633 => Some(Box::new(REVISION2::from_bits_truncate(value))),
        5634 => Some(Box::new(REVISION3::from_bits_truncate(value))),
        5635 => Some(Box::new(REVISION4::from_bits_truncate(value))),
        5638 => Some(Box::new(TEMP_RANGE_STATUS::from_bits_truncate(value))),
        5639 => Some(Box::new(ICL_STATUS::from_bits_truncate(value))),
        5640 => Some(Box::new(ADAPTER_5V_ICL_STATUS::from_bits_truncate(value))),
        5641 => Some(Box::new(ADAPTER_9V_ICL_STATUS::from_bits_truncate(value))),
        5642 => Some(Box::new(AICL_STATUS::from_bits_truncate(value))),
        5643 => Some(Box::new(POWER_PATH_STATUS::from_bits_truncate(value))),
        5644 => Some(Box::new(WDOG_STATUS::from_bits_truncate(value))),
        5645 => Some(Box::new(SYSOK_REASON_STATUS::from_bits_truncate(value))),
        5648 => Some(Box::new(MISC_INT_RT_STS_VAL::from_bits_truncate(value))),
        5696 => Some(Box::new(WDOG_RST::from_bits_truncate(value))),
        5697 => Some(Box::new(AFP_MODE::from_bits_truncate(value))),
        5698 => Some(Box::new(GSM_PA_ON_ADJ_EN::from_bits_truncate(value))),
        5699 => Some(Box::new(BARK_BITE_WDOG_PET::from_bits_truncate(value))),
        5700 => Some(Box::new(PHYON_CMD::from_bits_truncate(value))),
        5701 => Some(Box::new(SHDN_CMD::from_bits_truncate(value))),
        5711 => Some(Box::new(FINISH_COPY_COMMAND::from_bits_truncate(value))),
        5713 => Some(Box::new(WD_CFG::from_bits_truncate(value))),
        5714 => Some(Box::new(MISC_CFG::from_bits_truncate(value))),
        5715 => Some(Box::new(SNARL_BARK_BITE_WD_CFG::from_bits_truncate(value))),
        5716 => Some(Box::new(PHYON_CFG::from_bits_truncate(value))),
        5717 => Some(Box::new(CHGR_TRIM_OPTIONS_7_0::from_bits_truncate(value))),
        5718 => Some(Box::new(CH_OV_OPTION_CFG::from_bits_truncate(value))),
        5728 => Some(Box::new(AICL_CFG::from_bits_truncate(value))),
        5729 => Some(Box::new(AICL_RERUN_TIME_CFG::from_bits_truncate(value))),
        5730 => Some(Box::new(AICL_RERUN_TEMP_TIME_CFG::from_bits_truncate(value))),
        5744 => Some(Box::new(THERMREG_SRC_CFG::from_bits_truncate(value))),
        5745 => Some(Box::new(TREG_DIE_CMP_INC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5746 => Some(Box::new(TREG_DIE_CMP_DEC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5747 => Some(Box::new(TREG_DIE_ADC_INC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5748 => Some(Box::new(TREG_DIE_ADC_DEC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5749 => Some(Box::new(TREG_SKIN_ADC_INC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5750 => Some(Box::new(TREG_SKIN_ADC_DEC_CYCLE_TIME_CFG::from_bits_truncate(value))),
        5760 => Some(Box::new(BUCK_OPTIONS_CFG::from_bits_truncate(value))),
        5761 => Some(Box::new(ICL_SOFTSTART_RATE_CFG::from_bits_truncate(value))),
        5762 => Some(Box::new(ICL_SOFTSTOP_RATE_CFG::from_bits_truncate(value))),
        5763 => Some(Box::new(VSYS_MIN_SEL_CFG::from_bits_truncate(value))),
        5764 => Some(Box::new(TRACKING_VOLTAGE_SEL_CFG::from_bits_truncate(value))),
        5776 => Some(Box::new(STAT_CFG::from_bits_truncate(value))),
        5777 => Some(Box::new(LBC_EN_CFG::from_bits_truncate(value))),
        5778 => Some(Box::new(LBC_PERIOD_CFG::from_bits_truncate(value))),
        5779 => Some(Box::new(LBC_DUTY_CYCLE_CFG::from_bits_truncate(value))),
        5780 => Some(Box::new(SYSOK_CFG::from_bits_truncate(value))),
        5830 => Some(Box::new(ENG_SDCDC_CFG7::from_bits_truncate(value))),
        _ => None,
    }
}

pub fn register_name(addr: u16) -> &'static str {
    match addr {
        4096 => "CHGR_BASE",
        4352 => "OTG_BASE",
        4608 => "BATIF_BASE",
        4864 => "USBIN_BASE",
        5120 => "DCIN_BASE",
        5632 => "MISC_BASE",
        6400 => "CHGR_FREQ_BASE",
        4102 => "BATTERY_CHARGER_STATUS_1_REG",
        4103 => "BATTERY_CHARGER_STATUS_2_REG",
        4104 => "CHG_OPTION_REG",
        4105 => "BATTERY_CHARGER_STATUS_3_REG",
        4106 => "BATTERY_CHARGER_STATUS_4_REG",
        4107 => "BATTERY_CHARGER_STATUS_5_REG",
        4108 => "BATTERY_CHARGER_STATUS_6_REG",
        4109 => "BATTERY_CHARGER_STATUS_7_REG",
        4110 => "BATTERY_CHARGER_STATUS_8_REG",
        4160 => "STEP_CHG_SOC_VBATT_V_REG",
        4161 => "STEP_CHG_SOC_VBATT_V_UPDATE_REG",
        4162 => "CHARGING_ENABLE_CMD_REG",
        4163 => "ALLOW_FAST_CHARGING_CMD_REG",
        4164 => "QNOVO_PT_ENABLE_CMD_REG",
        4176 => "CHGR_CFG1_REG",
        4177 => "CHGR_CFG2_REG",
        4178 => "CHARGER_ENABLE_CFG_REG",
        4179 => "CFG_REG",
        4180 => "CHARGER_SPARE_REG",
        4192 => "PRE_CHARGE_CURRENT_CFG_REG",
        4193 => "FAST_CHARGE_CURRENT_CFG_REG",
        4194 => "CHARGE_CURRENT_TERMINATION_CFG_REG",
        4195 => "TCCC_CHARGE_CURRENT_TERMINATION_CFG_REG",
        4196 => "CHARGE_CURRENT_SOFTSTART_SETTING_CFG_REG",
        4208 => "FLOAT_VOLTAGE_CFG_REG",
        4209 => "AUTO_FLOAT_VOLTAGE_COMPENSATION_CFG_REG",
        4210 => "CHARGE_INHIBIT_THRESHOLD_CFG_REG",
        4211 => "RECHARGE_THRESHOLD_CFG_REG",
        4212 => "PRE_TO_FAST_CHARGE_THRESHOLD_CFG_REG",
        4213 => "FV_HYSTERESIS_CFG_REG",
        4224 => "FVC_CHARGE_INHIBIT_THRESHOLD_CFG_REG",
        4225 => "FVC_RECHARGE_THRESHOLD_CFG_REG",
        4226 => "FVC_PRE_TO_FAST_CHARGE_THRESHOLD_CFG_REG",
        4227 => "FVC_FULL_ON_THRESHOLD_CFG_REG",
        4228 => "FVC_CC_MODE_GLITCH_FILTER_SEL_CFG_REG",
        4229 => "FVC_TERMINATION_GLITCH_FILTER_SEL_CFG_REG",
        4240 => "JEITA_EN_CFG_REG",
        4241 => "JEITA_FVCOMP_CFG_REG",
        4242 => "JEITA_CCCOMP_CFG_REG",
        4214 => "FV_CAL_CFG_REG",
        4215 => "FV_ADJUST_REG",
        4216 => "FG_VADC_DISQ_THRESH_REG",
        4217 => "FG_IADC_DISQ_THRESH_REG",
        4218 => "FG_UPDATE_CFG_1_REG",
        4219 => "FG_UPDATE_CFG_1_SEL_REG",
        4220 => "FG_UPDATE_CFG_2_REG",
        4221 => "FG_UPDATE_CFG_2_SEL_REG",
        4222 => "FG_CHG_INTERFACE_CFG_REG",
        4223 => "FG_CHG_INTERFACE_CFG_SEL_REG",
        4272 => "CHGR_STEP_CHG_MODE_CFG_REG",
        4273 => "STEP_CHG_UPDATE_REQUEST_TIMEOUT_CFG_REG",
        4274 => "STEP_CHG_UPDATE_FAIL_TIMEOUT_CFG_REG",
        4275 => "STEP_CHG_SOC_OR_BATT_V_TH1_REG",
        4276 => "STEP_CHG_SOC_OR_BATT_V_TH2_REG",
        4277 => "STEP_CHG_SOC_OR_BATT_V_TH3_REG",
        4278 => "STEP_CHG_SOC_OR_BATT_V_TH4_REG",
        4279 => "STEP_CHG_CURRENT_DELTA1_REG",
        4280 => "STEP_CHG_CURRENT_DELTA2_REG",
        4281 => "STEP_CHG_CURRENT_DELTA3_REG",
        4282 => "STEP_CHG_CURRENT_DELTA4_REG",
        4283 => "STEP_CHG_CURRENT_DELTA5_REG",
        4358 => "RID_CC_CONTROL_23_16_REG",
        4359 => "RID_CC_CONTROL_15_8_REG",
        4360 => "RID_CC_CONTROL_7_0_REG",
        4361 => "OTG_STATUS_REG",
        4368 => "OTG_INT_RT_STS",
        4416 => "CMD_OTG_REG",
        4433 => "BAT_UVLO_THRESHOLD_CFG_REG",
        4434 => "OTG_CURRENT_LIMIT_CFG_REG",
        4435 => "OTG_CFG_REG",
        4544 => "OTG_ENG_OTG_CFG_REG",
        4624 => "BATIF_INT_RT_STS",
        4672 => "SHIP_MODE_REG",
        4688 => "BATOCP_THRESHOLD_CFG_REG",
        4689 => "BATOCP_INTRPT_DELAY_TMR_CFG_REG",
        4690 => "BATOCP_RESET_TMR_CFG_REG",
        4704 => "LOW_BATT_DETECT_EN_CFG_REG",
        4705 => "LOW_BATT_THRESHOLD_CFG_REG",
        4706 => "BAT_FET_CFG_REG",
        4720 => "BAT_MISS_SRC_CFG_REG",
        4721 => "BAT_MISS_ALG_OPTIONS_CFG_REG",
        4722 => "BAT_MISS_PIN_GF_CFG_REG",
        4870 => "USBIN_INPUT_STATUS_REG",
        4871 => "APSD_STATUS_REG",
        4872 => "APSD_RESULT_STATUS_REG",
        4873 => "QC_CHANGE_STATUS_REG",
        4874 => "QC_PULSE_COUNT_STATUS_REG",
        4875 => "TYPE_C_STATUS_1_REG",
        4876 => "TYPE_C_STATUS_2_REG",
        4877 => "TYPE_C_STATUS_3_REG",
        4878 => "TYPE_C_STATUS_4_REG",
        4879 => "TYPE_C_STATUS_5_REG",
        4880 => "USB_INT_RT_STS",
        4912 => "QC_PULSE_COUNT_STATUS_1_REG",
        4928 => "USBIN_CMD_IL_REG",
        4929 => "CMD_APSD_REG",
        4931 => "CMD_HVDCP_2_REG",
        4951 => "USB_MISC2_REG",
        4952 => "TYPE_C_CFG_REG",
        4953 => "TYPE_C_CFG_2_REG",
        4954 => "TYPE_C_CFG_3_REG",
        4955 => "HVDCP_PULSE_COUNT_MAX_REG",
        4960 => "USBIN_ADAPTER_ALLOW_CFG_REG",
        4962 => "USBIN_OPTIONS_1_CFG_REG",
        4963 => "USBIN_OPTIONS_2_CFG_REG",
        4964 => "TAPER_TIMER_SEL_CFG_REG",
        4965 => "USBIN_LOAD_CFG_REG",
        4966 => "USBIN_ICL_OPTIONS_REG",
        4967 => "TYPE_C_INTRPT_ENB_REG",
        4968 => "TYPE_C_INTRPT_ENB_SOFTWARE_CTRL_REG",
        4969 => "USBIN_SOURCE_CHANGE_INTRPT_ENB_REG",
        4976 => "USBIN_CURRENT_LIMIT_CFG_REG",
        4992 => "USBIN_AICL_OPTIONS_CFG_REG",
        4993 => "USBIN_5V_AICL_THRESHOLD_CFG_REG",
        4994 => "USBIN_9V_AICL_THRESHOLD_CFG_REG",
        4995 => "USBIN_12V_AICL_THRESHOLD_CFG_REG",
        4996 => "USBIN_CONT_AICL_THRESHOLD_CFG_REG",
        5126 => "DCIN_INPUT_STATUS_REG",
        5127 => "WIPWR_STATUS_REG",
        5128 => "WIPWR_RANGE_STATUS_REG",
        5136 => "DCIN_INT_RT_STS",
        5184 => "DCIN_CMD_IL_REG",
        5208 => "DC_SPARE_REG",
        5216 => "DCIN_ADAPTER_ALLOW_CFG_REG",
        5221 => "DCIN_LOAD_CFG_REG",
        5232 => "DCIN_CURRENT_LIMIT_CFG_REG",
        5248 => "DCIN_AICL_OPTIONS_CFG_REG",
        5249 => "DCIN_AICL_REF_SEL_CFG_REG",
        5250 => "DCIN_ICL_START_CFG_REG",
        5264 => "DIV2_EN_GF_TIME_CFG_REG",
        5265 => "WIPWR_IRQ_TMR_CFG_REG",
        5266 => "ZIN_ICL_PT_REG",
        5267 => "ZIN_ICL_LV_REG",
        5268 => "ZIN_ICL_HV_REG",
        5269 => "WI_PWR_OPTIONS_REG",
        5270 => "ZIN_ICL_PT_HV_REG",
        5271 => "ZIN_ICL_MID_LV_REG",
        5272 => "ZIN_ICL_MID_HV_REG",
        5313 => "DC_ENG_SSUPPLY_CFG2_REG",
        5314 => "DC_ENG_SSUPPLY_CFG3_REG",
        5632 => "REVISION1_REG",
        5633 => "REVISION2_REG",
        5634 => "REVISION3_REG",
        5635 => "REVISION4_REG",
        5638 => "TEMP_RANGE_STATUS_REG",
        5639 => "ICL_STATUS_REG",
        5640 => "ADAPTER_5V_ICL_STATUS_REG",
        5641 => "ADAPTER_9V_ICL_STATUS_REG",
        5642 => "AICL_STATUS_REG",
        5643 => "POWER_PATH_STATUS_REG",
        5644 => "WDOG_STATUS_REG",
        5645 => "SYSOK_REASON_STATUS_REG",
        5648 => "MISC_INT_RT_STS",
        5696 => "WDOG_RST_REG",
        5697 => "AFP_MODE_REG",
        5698 => "GSM_PA_ON_ADJ_EN_REG",
        5699 => "BARK_BITE_WDOG_PET_REG",
        5700 => "PHYON_CMD_REG",
        5701 => "SHDN_CMD_REG",
        5711 => "FINISH_COPY_COMMAND_REG",
        5713 => "WD_CFG_REG",
        5714 => "MISC_CFG_REG",
        5715 => "SNARL_BARK_BITE_WD_CFG_REG",
        5716 => "PHYON_CFG_REG",
        5717 => "CHGR_TRIM_OPTIONS_7_0_REG",
        5718 => "CH_OV_OPTION_CFG_REG",
        5728 => "AICL_CFG_REG",
        5729 => "AICL_RERUN_TIME_CFG_REG",
        5730 => "AICL_RERUN_TEMP_TIME_CFG_REG",
        5744 => "THERMREG_SRC_CFG_REG",
        5745 => "TREG_DIE_CMP_INC_CYCLE_TIME_CFG_REG",
        5746 => "TREG_DIE_CMP_DEC_CYCLE_TIME_CFG_REG",
        5747 => "TREG_DIE_ADC_INC_CYCLE_TIME_CFG_REG",
        5748 => "TREG_DIE_ADC_DEC_CYCLE_TIME_CFG_REG",
        5749 => "TREG_SKIN_ADC_INC_CYCLE_TIME_CFG_REG",
        5750 => "TREG_SKIN_ADC_DEC_CYCLE_TIME_CFG_REG",
        5760 => "BUCK_OPTIONS_CFG_REG",
        5761 => "ICL_SOFTSTART_RATE_CFG_REG",
        5762 => "ICL_SOFTSTOP_RATE_CFG_REG",
        5763 => "VSYS_MIN_SEL_CFG_REG",
        5764 => "TRACKING_VOLTAGE_SEL_CFG_REG",
        5776 => "STAT_CFG_REG",
        5777 => "LBC_EN_CFG_REG",
        5778 => "LBC_PERIOD_CFG_REG",
        5779 => "LBC_DUTY_CYCLE_CFG_REG",
        5780 => "SYSOK_CFG_REG",
        5792 => "CFG_BUCKBOOST_FREQ_SELECT_BUCK_REG",
        5793 => "CFG_BUCKBOOST_FREQ_SELECT_BOOST_REG",
        5865 => "TM_IO_DTEST4_SEL",
        5830 => "ENG_SDCDC_CFG7_REG",
        6480 => "FREQ_CLK_DIV_REG",
        _ => "unknown",
    }
}

