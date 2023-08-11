#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum NtFeature {
    NTF_NAVBAR_SWITCH,
    NTF_SCREENSHOT_SOUND,
    NTF_BRIGHTNESS_LEVELCUST,
    NTF_SYSTEM_POWER_TRACKER,
    NTF_GAMING_MODE,
    NTF_CONN_LOCATION_TRACKER,
    NTF_CONN_NFC_TRACKER,
    NTF_CONN_NFC_MUTUAL_WLC,
    NTF_CONN_NFC_LIGHTS,
    NTF_SLEEP_TIGHT,
    NTF_POP_UP_VIEW,
    NTF_DYNAMIC_THERMAL_CONFIG,
    NTF_BATTERY_CHARGE_CONFIG,
    NTF_DISPLAY_VRR,
    NTF_ARCSOFT_FACE_RECOGNITION,
    NTF_DUAL_APPS,
    NTF_SHOW_DUAL_SA_SWITCH,
    NTF_NETWORK_LIMIT,
    NTF_GAME_MODE_TOUCH_SAMPLE_RATE_ENHANCE,
    NTF_APP_LOCKER,
    NTF_ESSENTIAL_NOTIFICATION,
    NTF_ADVANCED_THERMAL_MITIGATION,
    NTF_WB_VOICE_MODEL,
    NTF_STATUSBAR_NETWORK_SPEED,
    NTF_DUAL_LIGHT_SENSOR,
    NTF_PRIVACY_ICON_CAMERA_BOKEH,
    NTF_SCREEN_ON_OFF_ANIMATION,
    NTF_BLOCK_BENCHMARK,
    NTF_BATTERY_HEALTH,
    NTF_NETWORK_WIFI_AP_TEMPERATURE,
    NTF_HDR_PEAK_BRIGHTNESS,
    NTF_GAME_COLOR_PLUS,
    NTF_SENSOR_BACK_LIGHT_EXTRA_SOURCE,
    NTF_MONITOR_CHARGE_SERVICE,
    NTF_FEATURE_UNKNOWN,
}

impl From<u8> for NtFeature {
    fn from(value: u8) -> Self {
        match value {
            0 => NtFeature::NTF_NAVBAR_SWITCH,
            1 => NtFeature::NTF_SCREENSHOT_SOUND,
            2 => NtFeature::NTF_BRIGHTNESS_LEVELCUST,
            3 => NtFeature::NTF_SYSTEM_POWER_TRACKER,
            4 => NtFeature::NTF_GAMING_MODE,
            5 => NtFeature::NTF_CONN_LOCATION_TRACKER,
            6 => NtFeature::NTF_CONN_NFC_TRACKER,
            7 => NtFeature::NTF_CONN_NFC_MUTUAL_WLC,
            8 => NtFeature::NTF_CONN_NFC_LIGHTS,
            9 => NtFeature::NTF_SLEEP_TIGHT,
            10 => NtFeature::NTF_POP_UP_VIEW,
            11 => NtFeature::NTF_DYNAMIC_THERMAL_CONFIG,
            12 => NtFeature::NTF_BATTERY_CHARGE_CONFIG,
            13 => NtFeature::NTF_DISPLAY_VRR,
            14 => NtFeature::NTF_ARCSOFT_FACE_RECOGNITION,
            15 => NtFeature::NTF_DUAL_APPS,
            16 => NtFeature::NTF_SHOW_DUAL_SA_SWITCH,
            17 => NtFeature::NTF_NETWORK_LIMIT,
            18 => NtFeature::NTF_GAME_MODE_TOUCH_SAMPLE_RATE_ENHANCE,
            19 => NtFeature::NTF_APP_LOCKER,
            20 => NtFeature::NTF_ESSENTIAL_NOTIFICATION,
            21 => NtFeature::NTF_ADVANCED_THERMAL_MITIGATION,
            22 => NtFeature::NTF_WB_VOICE_MODEL,
            23 => NtFeature::NTF_STATUSBAR_NETWORK_SPEED,
            24 => NtFeature::NTF_DUAL_LIGHT_SENSOR,
            25 => NtFeature::NTF_PRIVACY_ICON_CAMERA_BOKEH,
            26 => NtFeature::NTF_SCREEN_ON_OFF_ANIMATION,
            27 => NtFeature::NTF_BLOCK_BENCHMARK,
            28 => NtFeature::NTF_BATTERY_HEALTH,
            29 => NtFeature::NTF_NETWORK_WIFI_AP_TEMPERATURE,
            30 => NtFeature::NTF_HDR_PEAK_BRIGHTNESS,
            31 => NtFeature::NTF_GAME_COLOR_PLUS,
            32 => NtFeature::NTF_SENSOR_BACK_LIGHT_EXTRA_SOURCE,
            33 => NtFeature::NTF_MONITOR_CHARGE_SERVICE,
            _ => NtFeature::NTF_FEATURE_UNKNOWN,
        }
    }
}

impl std::str::FromStr for NtFeature {
    type Err = NtFeature;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NTF_NAVBAR_SWITCH" => Ok(NtFeature::NTF_NAVBAR_SWITCH),
            "NTF_SCREENSHOT_SOUND" => Ok(NtFeature::NTF_SCREENSHOT_SOUND),
            "NTF_BRIGHTNESS_LEVELCUST" => Ok(NtFeature::NTF_BRIGHTNESS_LEVELCUST),
            "NTF_SYSTEM_POWER_TRACKER" => Ok(NtFeature::NTF_SYSTEM_POWER_TRACKER),
            "NTF_GAMING_MODE" => Ok(NtFeature::NTF_GAMING_MODE),
            "NTF_CONN_LOCATION_TRACKER" => Ok(NtFeature::NTF_CONN_LOCATION_TRACKER),
            "NTF_CONN_NFC_TRACKER" => Ok(NtFeature::NTF_CONN_NFC_TRACKER),
            "NTF_CONN_NFC_MUTUAL_WLC" => Ok(NtFeature::NTF_CONN_NFC_MUTUAL_WLC),
            "NTF_CONN_NFC_LIGHTS" => Ok(NtFeature::NTF_CONN_NFC_LIGHTS),
            "NTF_SLEEP_TIGHT" => Ok(NtFeature::NTF_SLEEP_TIGHT),
            "NTF_POP_UP_VIEW" => Ok(NtFeature::NTF_POP_UP_VIEW),
            "NTF_DYNAMIC_THERMAL_CONFIG" => Ok(NtFeature::NTF_DYNAMIC_THERMAL_CONFIG),
            "NTF_BATTERY_CHARGE_CONFIG" => Ok(NtFeature::NTF_BATTERY_CHARGE_CONFIG),
            "NTF_DISPLAY_VRR" => Ok(NtFeature::NTF_DISPLAY_VRR),
            "NTF_ARCSOFT_FACE_RECOGNITION" => Ok(NtFeature::NTF_ARCSOFT_FACE_RECOGNITION),
            "NTF_DUAL_APPS" => Ok(NtFeature::NTF_DUAL_APPS),
            "NTF_SHOW_DUAL_SA_SWITCH" => Ok(NtFeature::NTF_SHOW_DUAL_SA_SWITCH),
            "NTF_NETWORK_LIMIT" => Ok(NtFeature::NTF_NETWORK_LIMIT),
            "NTF_GAME_MODE_TOUCH_SAMPLE_RATE_ENHANCE" => {
                Ok(NtFeature::NTF_GAME_MODE_TOUCH_SAMPLE_RATE_ENHANCE)
            }
            "NTF_APP_LOCKER" => Ok(NtFeature::NTF_APP_LOCKER),
            "NTF_ESSENTIAL_NOTIFICATION" => Ok(NtFeature::NTF_ESSENTIAL_NOTIFICATION),
            "NTF_ADVANCED_THERMAL_MITIGATION" => Ok(NtFeature::NTF_ADVANCED_THERMAL_MITIGATION),
            "NTF_WB_VOICE_MODEL" => Ok(NtFeature::NTF_WB_VOICE_MODEL),
            "NTF_STATUSBAR_NETWORK_SPEED" => Ok(NtFeature::NTF_STATUSBAR_NETWORK_SPEED),
            "NTF_DUAL_LIGHT_SENSOR" => Ok(NtFeature::NTF_DUAL_LIGHT_SENSOR),
            "NTF_PRIVACY_ICON_CAMERA_BOKEH" => Ok(NtFeature::NTF_PRIVACY_ICON_CAMERA_BOKEH),
            "NTF_SCREEN_ON_OFF_ANIMATION" => Ok(NtFeature::NTF_SCREEN_ON_OFF_ANIMATION),
            "NTF_BLOCK_BENCHMARK" => Ok(NtFeature::NTF_BLOCK_BENCHMARK),
            "NTF_BATTERY_HEALTH" => Ok(NtFeature::NTF_BATTERY_HEALTH),
            "NTF_NETWORK_WIFI_AP_TEMPERATURE" => Ok(NtFeature::NTF_NETWORK_WIFI_AP_TEMPERATURE),
            "NTF_HDR_PEAK_BRIGHTNESS" => Ok(NtFeature::NTF_HDR_PEAK_BRIGHTNESS),
            "NTF_GAME_COLOR_PLUS" => Ok(NtFeature::NTF_GAME_COLOR_PLUS),
            "NTF_SENSOR_BACK_LIGHT_EXTRA_SOURCE" => {
                Ok(NtFeature::NTF_SENSOR_BACK_LIGHT_EXTRA_SOURCE)
            }
            "NTF_MONITOR_CHARGE_SERVICE" => Ok(NtFeature::NTF_MONITOR_CHARGE_SERVICE),
            _ => Err(NtFeature::NTF_FEATURE_UNKNOWN),
        }
    }
}