use esp_idf_svc::sys::wifi_config_t;
use esp_idf_svc::sys::{
    esp_err_t, wifi_prov_event_handler_t, wifi_prov_mgr_config_t, wifi_prov_mgr_deinit,
    wifi_prov_mgr_init, wifi_prov_mgr_is_provisioned, wifi_prov_mgr_start_provisioning,
    wifi_prov_mgr_wait, wifi_prov_scheme_ble, wifi_prov_scheme_ble_event_cb_free_btdm,
    wifi_prov_security_WIFI_PROV_SECURITY_1, ESP_OK,
};
use esp_idf_svc::sys::{esp_wifi_get_config, wifi_prov_mgr_reset_provisioning};

pub fn reset_provisioning() -> anyhow::Result<()> {
    unsafe { esp_ok(wifi_prov_mgr_reset_provisioning()) }
}

pub fn wifi_is_provisioned() -> anyhow::Result<bool> {
    let mut result = false;
    unsafe {
        esp_ok(wifi_prov_mgr_is_provisioned(&mut result))?;
    }
    Ok(result)
}

pub fn start_wifi_provisioning(pop: &str, service_name: &str) -> anyhow::Result<()> {
    unsafe {
        let mut cfg = wifi_prov_mgr_config_t {
            scheme: wifi_prov_scheme_ble,
            scheme_event_handler: wifi_prov_event_handler_t {
                event_cb: Some(wifi_prov_scheme_ble_event_cb_free_btdm),
                user_data: core::ptr::null_mut() as *mut core::ffi::c_void,
            },
            app_event_handler: wifi_prov_event_handler_t {
                event_cb: None,
                user_data: core::ptr::null_mut() as *mut core::ffi::c_void,
            },
        };

        log::info!("Wifi prov init...");
        esp_ok(wifi_prov_mgr_init(cfg))?;

        let pop_c = std::ffi::CString::new(pop)?;
        let service_name_c = std::ffi::CString::new(service_name)?;
        let service_key_c = std::ffi::CString::new("")?; // empty means open

        log::info!("Wifi prov start...");
        esp_ok(wifi_prov_mgr_start_provisioning(
            wifi_prov_security_WIFI_PROV_SECURITY_1,
            pop_c.as_ptr() as *const _ as *mut _,
            service_name_c.as_ptr(),
            service_key_c.as_ptr(),
        ))?;

        log::info!("Wifi prov wait...!");
        wifi_prov_mgr_wait();

        log::info!("Wifi prov deinit...");
        wifi_prov_mgr_deinit()
    }

    Ok(())
}

pub fn foo() {
    unsafe {}
}

fn esp_ok(res: esp_err_t) -> anyhow::Result<()> {
    if res == ESP_OK {
        Ok(())
    } else {
        Err(anyhow::anyhow!("ESP-IDF error: {}", res))
    }
}
