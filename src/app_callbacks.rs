use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, FieldValue, Member, Path, Token};

pub struct AppCallbacks {
    app_on_connection: Option<Path>,
    app_on_disconnect: Option<Path>,
    app_on_connect_failed: Option<Path>,
    app_on_update_params_rejected: Option<Path>,
    app_on_update_params_complete: Option<Path>,
    app_on_set_dev_config_complete: Option<Path>,
    app_on_adv_nonconn_complete: Option<Path>,
    app_on_adv_undirect_complete: Option<Path>,
    app_on_adv_direct_complete: Option<Path>,
    app_on_db_init_complete: Option<Path>,
    app_on_scanning_completed: Option<Path>,
    app_on_adv_report_ind: Option<Path>,
    app_on_get_dev_name: Option<Path>,
    app_on_get_dev_appearance: Option<Path>,
    app_on_get_dev_slv_pref_params: Option<Path>,
    app_on_set_dev_info: Option<Path>,
    app_on_data_length_change: Option<Path>,
    app_on_update_params_request: Option<Path>,
    app_on_generate_static_random_addr: Option<Path>,
    app_on_svc_changed_cfg_ind: Option<Path>,
    app_on_get_peer_features: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_pairing_request: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_tk_exch: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_irk_exch: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_csrk_exch: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_ltk_exch: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_pairing_succeeded: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_encrypt_ind: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_encrypt_req_ind: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_security_req_ind: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_addr_solved_ind: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_addr_resolve_failed: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_ral_cmp_evt: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_ral_size_ind: Option<Path>,
    #[cfg(feature = "app_security")]
    app_on_ral_addr_ind: Option<Path>,
}

macro_rules! path_try {
    ($value:ident) => {
        match $value {
            Expr::Path(path) => Some(path.path),
            _ => panic!("Unexpected expression: {:?}", $value),
        }
    };
}

impl Parse for AppCallbacks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut callbacks = Self {
            app_on_connection: None,
            app_on_disconnect: None,
            app_on_connect_failed: None,
            app_on_update_params_rejected: None,
            app_on_update_params_complete: None,
            app_on_set_dev_config_complete: None,
            app_on_adv_nonconn_complete: None,
            app_on_adv_undirect_complete: None,
            app_on_adv_direct_complete: None,
            app_on_db_init_complete: None,
            app_on_scanning_completed: None,
            app_on_adv_report_ind: None,
            app_on_get_dev_name: None,
            app_on_get_dev_appearance: None,
            app_on_get_dev_slv_pref_params: None,
            app_on_set_dev_info: None,
            app_on_data_length_change: None,
            app_on_update_params_request: None,
            app_on_generate_static_random_addr: None,
            app_on_svc_changed_cfg_ind: None,
            app_on_get_peer_features: None,
            #[cfg(feature = "app_security")]
            app_on_pairing_request: None,
            #[cfg(feature = "app_security")]
            app_on_tk_exch: None,
            #[cfg(feature = "app_security")]
            app_on_irk_exch: None,
            #[cfg(feature = "app_security")]
            app_on_csrk_exch: None,
            #[cfg(feature = "app_security")]
            app_on_ltk_exch: None,
            #[cfg(feature = "app_security")]
            app_on_pairing_succeeded: None,
            #[cfg(feature = "app_security")]
            app_on_encrypt_ind: None,
            #[cfg(feature = "app_security")]
            app_on_encrypt_req_ind: None,
            #[cfg(feature = "app_security")]
            app_on_security_req_ind: None,
            #[cfg(feature = "app_security")]
            app_on_addr_solved_ind: None,
            #[cfg(feature = "app_security")]
            app_on_addr_resolve_failed: None,
            #[cfg(feature = "app_security")]
            app_on_ral_cmp_evt: None,
            #[cfg(feature = "app_security")]
            app_on_ral_size_ind: None,
            #[cfg(feature = "app_security")]
            app_on_ral_addr_ind: None,
        };

        for field in fields {
            let key = match field.member {
                Member::Named(name) => name,
                Member::Unnamed(unnamed) => {
                    panic!("Unexpected unnamed field: {:?}", unnamed);
                }
            };
            let value = field.expr;
            match key.to_string().as_str() {
                "app_on_connection" => {
                    callbacks.app_on_connection = path_try!(value);
                }
                "app_on_disconnect" => {
                    callbacks.app_on_disconnect = path_try!(value);
                }
                "app_on_connect_failed" => {
                    callbacks.app_on_connect_failed = path_try!(value);
                }
                "app_on_update_params_rejected" => {
                    callbacks.app_on_update_params_rejected = path_try!(value);
                }
                "app_on_update_params_complete" => {
                    callbacks.app_on_update_params_complete = path_try!(value);
                }
                "app_on_set_dev_config_complete" => {
                    callbacks.app_on_set_dev_config_complete = path_try!(value);
                }
                "app_on_adv_nonconn_complete" => {
                    callbacks.app_on_adv_nonconn_complete = path_try!(value);
                }
                "app_on_adv_undirect_complete" => {
                    callbacks.app_on_adv_undirect_complete = path_try!(value);
                }
                "app_on_adv_direct_complete" => {
                    callbacks.app_on_adv_direct_complete = path_try!(value);
                }
                "app_on_db_init_complete" => {
                    callbacks.app_on_db_init_complete = path_try!(value);
                }
                "app_on_scanning_completed" => {
                    callbacks.app_on_scanning_completed = path_try!(value);
                }
                "app_on_adv_report_ind" => {
                    callbacks.app_on_adv_report_ind = path_try!(value);
                }
                "app_on_get_dev_name" => {
                    callbacks.app_on_get_dev_name = path_try!(value);
                }
                "app_on_get_dev_appearance" => {
                    callbacks.app_on_get_dev_appearance = path_try!(value);
                }
                "app_on_get_dev_slv_pref_params" => {
                    callbacks.app_on_get_dev_slv_pref_params = path_try!(value);
                }
                "app_on_set_dev_info" => {
                    callbacks.app_on_set_dev_info = path_try!(value);
                }
                "app_on_data_length_change" => {
                    callbacks.app_on_data_length_change = path_try!(value);
                }
                "app_on_update_params_request" => {
                    callbacks.app_on_update_params_request = path_try!(value);
                }
                "app_on_generate_static_random_addr" => {
                    callbacks.app_on_generate_static_random_addr = path_try!(value);
                }
                "app_on_svc_changed_cfg_ind" => {
                    callbacks.app_on_svc_changed_cfg_ind = path_try!(value);
                }
                "app_on_get_peer_features" => {
                    callbacks.app_on_get_peer_features = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_pairing_request" => {
                    callbacks.app_on_pairing_request = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_tk_exch" => {
                    callbacks.app_on_tk_exch = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_irk_exch" => {
                    callbacks.app_on_irk_exch = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_csrk_exch" => {
                    callbacks.app_on_csrk_exch = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_ltk_exch" => {
                    callbacks.app_on_ltk_exch = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_pairing_succeeded" => {
                    callbacks.app_on_pairing_succeeded = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_encrypt_ind" => {
                    callbacks.app_on_encrypt_ind = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_encrypt_req_ind" => {
                    callbacks.app_on_encrypt_req_ind = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_security_req_ind" => {
                    callbacks.app_on_security_req_ind = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_addr_solved_ind" => {
                    callbacks.app_on_addr_solved_ind = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_addr_resolve_failed" => {
                    callbacks.app_on_addr_resolve_failed = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_ral_cmp_evt" => {
                    callbacks.app_on_ral_cmp_evt = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_ral_size_ind" => {
                    callbacks.app_on_ral_size_ind = path_try!(value);
                }
                #[cfg(feature = "app_security")]
                "app_on_ral_addr_ind" => {
                    callbacks.app_on_ral_addr_ind = path_try!(value);
                }
                _ => {
                    panic!("Unexpected field: {} = {:?}", key, value);
                }
            }
        }

        Ok(callbacks)
    }
}

impl AppCallbacks {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut callback_wrappers = Vec::new();
        let mut struct_fields = Vec::new();

        if let Some(app_on_connection) = &self.app_on_connection {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_connection(conidx: u8, param: *const da14531_sdk::ble_stack::host::gap::gapc::task::GapcConnectionReqInd) {
                    let param = unsafe {&*param};
                    #app_on_connection(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_on_connection: Some(__app_on_connection)));
        } else {
            struct_fields.push(quote!(
                app_on_connection: Some(da14531_sdk::bindings::default_app_on_connection)
            ));
        }
        if let Some(app_on_disconnect) = &self.app_on_disconnect {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_disconnect(param: *const da14531_sdk::ble_stack::host::gap::gapc::task::GapcDisconnectInd) {
                    let param = unsafe {&*param};
                    #app_on_disconnect(param);
                }
            ));
            struct_fields.push(quote!(app_on_disconnect: Some(__app_on_disconnect)));
        } else {
            struct_fields.push(quote!(app_on_disconnect: None));
        }
        if let Some(app_on_connect_failed) = &self.app_on_connect_failed {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_connect_failed() {
                    #app_on_connect_failed();
                }
            ));
            struct_fields.push(quote!(app_on_connect_failed: Some(__app_on_connect_failed)));
        } else {
            struct_fields.push(quote!(app_on_connect_failed: None));
        }
        if let Some(app_on_update_params_rejected) = &self.app_on_update_params_rejected {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_update_params_rejected() {
                    #app_on_update_params_rejected();
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_rejected: Some(__app_on_update_params_rejected)
            ));
        } else {
            struct_fields.push(quote!(app_on_update_params_rejected: None));
        }
        if let Some(app_on_update_params_complete) = &self.app_on_update_params_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_update_params_complete() {
                    #app_on_update_params_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_complete: Some(__app_on_update_params_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_update_params_complete: None));
        }
        if let Some(app_on_set_dev_config_complete) = &self.app_on_set_dev_config_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_set_dev_config_complete() {
                    #app_on_set_dev_config_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_set_dev_config_complete: Some(__app_on_set_dev_config_complete)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_set_dev_config_complete:
                    Some(da14531_sdk::bindings::default_app_on_set_dev_config_complete)
            ));
        }
        if let Some(app_on_adv_nonconn_complete) = &self.app_on_adv_nonconn_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_adv_nonconn_complete() {
                    #app_on_adv_nonconn_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_nonconn_complete: Some(__app_on_adv_nonconn_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_nonconn_complete: None));
        }
        if let Some(app_on_adv_undirect_complete) = &self.app_on_adv_undirect_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_adv_undirect_complete(status: u8) {
                    #app_on_adv_undirect_complete(status);
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_undirect_complete: Some(__app_on_adv_undirect_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_undirect_complete: None));
        }
        if let Some(app_on_adv_direct_complete) = &self.app_on_adv_direct_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_adv_direct_complete() {
                    #app_on_adv_direct_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_adv_direct_complete: Some(__app_on_adv_direct_complete)
            ));
        } else {
            struct_fields.push(quote!(app_on_adv_direct_complete: None));
        }
        if let Some(app_on_db_init_complete) = &self.app_on_db_init_complete {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_db_init_complete() {
                    #app_on_db_init_complete();
                }
            ));
            struct_fields.push(quote!(
                app_on_db_init_complete: Some(__app_on_db_init_complete)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_db_init_complete:
                    Some(da14531_sdk::bindings::default_app_on_db_init_complete)
            ));
        }
        if let Some(app_on_scanning_completed) = &self.app_on_scanning_completed {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_scanning_completed() {
                    #app_on_scanning_completed();
                }
            ));
            struct_fields.push(quote!(
                app_on_scanning_completed: Some(__app_on_scanning_completed)
            ));
        } else {
            struct_fields.push(quote!(app_on_scanning_completed: None));
        }
        if let Some(app_on_adv_report_ind) = &self.app_on_adv_report_ind {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_adv_report_ind() {
                    #app_on_adv_report_ind();
                }
            ));
            struct_fields.push(quote!(app_on_adv_report_ind: Some(__app_on_adv_report_ind)));
        } else {
            struct_fields.push(quote!(app_on_adv_report_ind: None));
        }
        if let Some(app_on_get_dev_name) = &self.app_on_get_dev_name {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_get_dev_name(device_name: *mut da14531_sdk::bindings::app_device_name) {
                    let device_name = unsafe {&mut *device_name};
                    #app_on_get_dev_name(device_name);
                }
            ));
            struct_fields.push(quote!(app_on_get_dev_name: Some(__app_on_get_dev_name)));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_name: Some(da14531_sdk::bindings::default_app_on_get_dev_name)
            ));
        }
        if let Some(app_on_get_dev_appearance) = &self.app_on_get_dev_appearance {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_get_dev_appearance(appearance: *mut u16) {
                    let appearance = unsafe { &mut *appearance };
                    #app_on_get_dev_appearance(appearance);
                }
            ));
            struct_fields.push(quote!(
                app_on_get_dev_appearance: Some(__app_on_get_dev_appearance)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_appearance:
                    Some(da14531_sdk::bindings::default_app_on_get_dev_appearance)
            ));
        }
        if let Some(app_on_get_dev_slv_pref_params) = &self.app_on_get_dev_slv_pref_params {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_get_dev_slv_pref_params() {
                    #app_on_get_dev_slv_pref_params();
                }
            ));
            struct_fields.push(quote!(
                app_on_get_dev_slv_pref_params: Some(__app_on_get_dev_slv_pref_params)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_get_dev_slv_pref_params:
                    Some(da14531_sdk::bindings::default_app_on_get_dev_slv_pref_params)
            ));
        }
        if let Some(app_on_set_dev_info) = &self.app_on_set_dev_info {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_set_dev_info() {
                    #app_on_set_dev_info();
                }
            ));
            struct_fields.push(quote!(app_on_set_dev_info: Some(__app_on_set_dev_info)));
        } else {
            struct_fields.push(quote!(
                app_on_set_dev_info: Some(da14531_sdk::bindings::default_app_on_set_dev_info)
            ));
        }
        if let Some(app_on_data_length_change) = &self.app_on_data_length_change {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_data_length_change(conn_id: u8, pkt: *mut gapc_le_pkt_size_ind) {
                    let pkt = unsafe {&mut *pkt};
                    #app_on_data_length_change(conn_id, pkt);
                }
            ));
            struct_fields.push(quote!(
                app_on_data_length_change: Some(__app_on_data_length_change)
            ));
        } else {
            struct_fields.push(quote!(app_on_data_length_change: None));
        }
        if let Some(app_on_update_params_request) = &self.app_on_update_params_request {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_update_params_request(param: *const gapc_param_update_req_ind ,cfm: *mut gapc_param_update_cfm) {
                    let param = unsafe {&*param};
                    let cfm = unsafe {&mut *cfm};
                    #app_on_update_params_request(param, cfm);
                }
            ));
            struct_fields.push(quote!(
                app_on_update_params_request: Some(__app_on_update_params_request)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_update_params_request:
                    Some(da14531_sdk::bindings::default_app_update_params_request)
            ));
        }
        if let Some(app_on_generate_static_random_addr) = &self.app_on_generate_static_random_addr {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_generate_static_random_addr(addr: *mut da14531_sdk::platform::core_modules::common::BDAddr) {
                    let addr = unsafe {&mut *addr};
                    #app_on_generate_static_random_addr(addr);
                }
            ));
            struct_fields.push(quote!(
                app_on_generate_static_random_addr: Some(__app_on_generate_static_random_addr)
            ));
        } else {
            struct_fields.push(quote!(
                app_on_generate_static_random_addr:
                    Some(da14531_sdk::bindings::default_app_generate_static_random_addr)
            ));
        }
        if let Some(app_on_svc_changed_cfg_ind) = &self.app_on_svc_changed_cfg_ind {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_svc_changed_cfg_ind() {
                    #app_on_svc_changed_cfg_ind();
                }
            ));
            struct_fields.push(quote!(
                app_on_svc_changed_cfg_ind: Some(__app_on_svc_changed_cfg_ind)
            ));
        } else {
            struct_fields.push(quote!(app_on_svc_changed_cfg_ind: None));
        }
        if let Some(app_on_get_peer_features) = &self.app_on_get_peer_features {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_get_peer_features(conn_id: u8, features: *const gapc_peer_features_ind) {
                    let features = unsafe {&*features};
                    #app_on_get_peer_features(conn_id, features);
                }
            ));
            struct_fields.push(quote!(
                app_on_get_peer_features: Some(__app_on_get_peer_features)
            ));
        } else {
            struct_fields.push(quote!(app_on_get_peer_features: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_pairing_request) = &self.app_on_pairing_request {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_pairing_request(conn_id: u8, param: *const da14531_sdk::bindings::gapc_bond_req_ind) {
                    let param = unsafe {&*param};
                    #app_on_pairing_request(conn_id, param);
                }
            ));
            struct_fields.push(quote!(
                app_on_pairing_request: Some(__app_on_pairing_request)
            ));
        } else {
            struct_fields.push(quote!(app_on_pairing_request: Some(da14531_sdk::bindings::default_app_on_pairing_request)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_tk_exch) = &self.app_on_tk_exch {
            callback_wrappers.push(quote!(
                extern "C" fn __app_on_tk_exch(conn_id: u8, param: *const da14531_sdk::bindings::gapc_bond_req_ind) {
                    let param = unsafe {&*param};
                    #app_on_tk_exch(conn_id, param);
                }
            ));
            struct_fields.push(quote!(
                app_on_tk_exch: Some(__app_on_tk_exch)
            ));
        } else {
            struct_fields
                .push(quote!(app_on_tk_exch: Some(da14531_sdk::bindings::default_app_on_tk_exch)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_irk_exch) = &self.app_on_irk_exch {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_irk_exch(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_irk_exch(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_irk_exch: Some(__app_on_irk_exch)
            //));
        } else {
            struct_fields.push(quote!(app_on_irk_exch: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_csrk_exch) = &self.app_on_csrk_exch {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_csrk_exch(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_csrk_exch(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_csrk_exch: Some(__app_on_csrk_exch)
            //));
        } else {
            struct_fields.push(
                quote!(app_on_csrk_exch: Some(da14531_sdk::bindings::default_app_on_csrk_exch)),
            );
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_ltk_exch) = &self.app_on_ltk_exch {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_ltk_exch(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_ltk_exch(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_ltk_exch: Some(__app_on_ltk_exch)
            //));
        } else {
            struct_fields.push(quote!(app_on_ltk_exch: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_pairing_succeeded) = &self.app_on_pairing_succeeded {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_pairing_succeeded(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_pairing_succeeded(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_pairing_succeeded: Some(__app_on_pairing_succeeded)
            //));
        } else {
            struct_fields.push(quote!(app_on_pairing_succeeded: Some(da14531_sdk::bindings::default_app_on_pairing_succeeded)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_encrypt_ind) = &self.app_on_encrypt_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_encrypt_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_encrypt_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_encrypt_ind: Some(__app_on_encrypt_ind)
            //));
        } else {
            struct_fields.push(quote!(app_on_encrypt_ind: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_encrypt_req_ind) = &self.app_on_encrypt_req_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_encrypt_req_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_encrypt_req_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_encrypt_req_ind: Some(__app_on_encrypt_req_ind)
            //));
        } else {
            struct_fields
                .push(quote!(app_on_encrypt_req_ind: Some(da14531_sdk::bindings::default_app_on_encrypt_req_ind)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_security_req_ind) = &self.app_on_security_req_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_security_req_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_security_req_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_security_req_ind: Some(__app_on_security_req_ind)
            //));
        } else {
            struct_fields.push(quote!(app_on_security_req_ind: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_addr_solved_ind) = &self.app_on_addr_solved_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_addr_solved_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_addr_solved_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_addr_solved_ind: Some(__app_on_addr_solved_ind)
            //));
        } else {
            struct_fields.push(quote!(app_on_addr_solved_ind: Some(da14531_sdk::bindings::default_app_on_addr_solved_ind)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_addr_resolve_failed) = &self.app_on_addr_resolve_failed {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_addr_resolve_failed(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_addr_resolve_failed(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_addr_resolve_failed: Some(__app_on_addr_resolve_failed)
            //));
        } else {
            struct_fields.push(quote!(app_on_addr_resolve_failed: Some(da14531_sdk::bindings::default_app_on_addr_resolve_failed)));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_ral_cmp_evt) = &self.app_on_ral_cmp_evt {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_ral_cmp_evt(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_ral_cmp_evt(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_ral_cmp_evt: Some(__app_on_ral_cmp_evt)
            //));
        } else {
            struct_fields.push(
                quote!(app_on_ral_cmp_evt: Some(da14531_sdk::bindings::default_app_on_ral_cmp_evt)),
            );
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_ral_size_ind) = &self.app_on_ral_size_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_ral_size_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_ral_size_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_ral_size_ind: Some(__app_on_ral_size_ind)
            //));
        } else {
            struct_fields.push(quote!(app_on_ral_size_ind: None));
        }
        #[cfg(feature = "app_security")]
        if let Some(app_on_ral_addr_ind) = &self.app_on_ral_addr_ind {
            //callback_wrappers.push(quote!(
            //    extern "C" fn __app_on_ral_addr_ind(conn_id: u8, features: *const gapc_peer_features_ind) {
            //        let features = unsafe {&*features};
            //        #app_on_ral_addr_ind(conn_id, features);
            //    }
            //));
            //struct_fields.push(quote!(
            //    app_on_ral_addr_ind: Some(__app_on_ral_addr_ind)
            //));
        } else {
            struct_fields.push(quote!(app_on_ral_addr_ind: None));
        }

        quote!(
            #(#callback_wrappers)*

            #[export_name = "user_app_callbacks"]
            pub static USER_APP_CALLBACKS: da14531_sdk::app_modules::AppCallbacks =
                da14531_sdk::app_modules::AppCallbacks {
                #(#struct_fields),*
            };
        )
    }
}
