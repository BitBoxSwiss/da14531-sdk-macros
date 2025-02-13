use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, FieldValue, Member, Path, Token};

pub struct AppBondDbCallbacks {
    app_bdb_init: Option<Path>,
    app_bdb_get_size: Option<Path>,
    app_bdb_add_entry: Option<Path>,
    app_bdb_remove_entry: Option<Path>,
    app_bdb_search_entry: Option<Path>,
    app_bdb_get_number_of_stored_irks: Option<Path>,
    app_bdb_get_stored_irks: Option<Path>,
    app_bdb_get_device_info_from_slot: Option<Path>,
}

macro_rules! path_try {
    ($value:ident) => {
        match $value {
            Expr::Path(path) => Some(path.path),
            _ => panic!("Unexpected expression: {:?}", $value),
        }
    };
}

impl Parse for AppBondDbCallbacks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<FieldValue, Token![,]> =
            input.parse_terminated(FieldValue::parse)?;

        let mut callbacks = Self {
            app_bdb_init: None,
            app_bdb_get_size: None,
            app_bdb_add_entry: None,
            app_bdb_remove_entry: None,
            app_bdb_search_entry: None,
            app_bdb_get_number_of_stored_irks: None,
            app_bdb_get_stored_irks: None,
            app_bdb_get_device_info_from_slot: None,
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
                "app_bdb_init" => callbacks.app_bdb_init = path_try!(value),
                "app_bdb_get_size" => callbacks.app_bdb_get_size = path_try!(value),
                "app_bdb_add_entry" => callbacks.app_bdb_add_entry = path_try!(value),
                "app_bdb_remove_entry" => callbacks.app_bdb_remove_entry = path_try!(value),
                "app_bdb_search_entry" => callbacks.app_bdb_search_entry = path_try!(value),
                "app_bdb_get_number_of_stored_irks" => {
                    callbacks.app_bdb_get_number_of_stored_irks = path_try!(value)
                }
                "app_bdb_get_stored_irks" => callbacks.app_bdb_get_stored_irks = path_try!(value),
                "app_bdb_get_device_info_from_slot" => {
                    callbacks.app_bdb_get_device_info_from_slot = path_try!(value)
                }
                _ => {
                    panic!("Unexpected field: {} = {:?}", key, value);
                }
            }
        }

        Ok(callbacks)
    }
}

impl AppBondDbCallbacks {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let mut callback_wrappers = Vec::new();
        let mut struct_fields = Vec::new();

        if let Some(fun) = &self.app_bdb_init {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_init() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_init: Some(__app_bdb_init)));
        } else {
            struct_fields.push(quote!(
                app_bdb_init: Some(da14531_sdk::bindings::default_app_bdb_init)
            ));
        }
        if let Some(fun) = &self.app_bdb_get_size {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_get_size() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_get_size: Some(__app_bdb_get_size)));
        } else {
            struct_fields.push(quote!(
                app_bdb_get_size: Some(da14531_sdk::bindings::default_app_bdb_get_size)
            ));
        }
        if let Some(fun) = &self.app_bdb_add_entry {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_add_entry() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_add_entry: Some(__app_bdb_add_entry)));
        } else {
            struct_fields.push(quote!(
                app_bdb_add_entry: Some(da14531_sdk::bindings::default_app_bdb_add_entry)
            ));
        }
        if let Some(fun) = &self.app_bdb_remove_entry {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_remove_entry() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_remove_entry: Some(__app_bdb_remove_entry)));
        } else {
            struct_fields.push(quote!(
                app_bdb_remove_entry: Some(da14531_sdk::bindings::default_app_bdb_remove_entry)
            ));
        }
        if let Some(fun) = &self.app_bdb_search_entry {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_search_entry() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_search_entry: Some(__app_bdb_search_entry)));
        } else {
            struct_fields.push(quote!(
                app_bdb_search_entry: Some(da14531_sdk::bindings::default_app_bdb_search_entry)
            ));
        }
        if let Some(fun) = &self.app_bdb_get_number_of_stored_irks {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_get_number_of_stored_irks() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_get_number_of_stored_irks: Some(__app_bdb_get_number_of_stored_irks)));
        } else {
            struct_fields.push(quote!(
                app_bdb_get_number_of_stored_irks: Some(da14531_sdk::bindings::default_app_bdb_get_number_of_stored_irks)
            ));
        }
        if let Some(fun) = &self.app_bdb_get_stored_irks {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_get_stored_irks() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_get_stored_irks: Some(__app_bdb_get_stored_irks)));
        } else {
            struct_fields.push(quote!(
                app_bdb_get_stored_irks: Some(da14531_sdk::bindings::default_app_bdb_get_stored_irks)
            ));
        }
        if let Some(fun) = &self.app_bdb_get_device_info_from_slot {
            callback_wrappers.push(quote!(
                extern "C" fn __app_bdb_get_device_info_from_slot() {
                    let param = unsafe {&*param};
                    #fun(conidx, param);
                }
            ));
            struct_fields.push(quote!(app_bdb_get_device_info_from_slot: Some(__app_bdb_get_device_info_from_slot)));
        } else {
            struct_fields.push(quote!(
                app_bdb_get_device_info_from_slot: Some(da14531_sdk::bindings::default_app_bdb_get_device_info_from_slot)
            ));
        }

        quote!(
            #(#callback_wrappers)*

            #[export_name = "user_app_bond_db_callbacks"]
            pub static USER_APP_BOND_DB_CALLBACKS: da14531_sdk::app_modules::AppBondDbCallbacks =
                da14531_sdk::app_modules::AppBondDbCallbacks {
                #(#struct_fields),*
            };
        )
    }
}
