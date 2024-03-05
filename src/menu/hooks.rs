use std::ops::Deref;

use once_cell::sync::Lazy;

use crate::{game::scaleform::value::{Value, ValueType}, hash, logging::Logger, make_hook, patterns, utils};

const UPDATE_OPTION: Lazy<u32> = Lazy::new(|| hash::gui_hash("UpdateOption"));
const UPDATE_COLOUR_OPTION: Lazy<u32> = Lazy::new(|| hash::gui_hash("UpdateColorOption"));
const OPTIONS_ACCEPT: Lazy<u32> = Lazy::new(|| hash::gui_hash("OptionsAccept"));

make_hook!(
    HOOK_ExternalInterface_LobbyManager,
    utils::scan(patterns::EXTERNALINTERFACE_LOBBY).unwrap(),
    (_movie: *const (), method_hash: u32, args: *mut [Value; 0], nargs: u32) {
        let mut opt_id = None;
        if !args.is_null() {
            let first = (&*args).get_unchecked(0);
            if first.get_type() == ValueType::UInt {
                opt_id = Some(first.get_uint());
            }
        }

        // Ignore native options
        if opt_id == None || opt_id < Some(1000) {
            return HOOK_ExternalInterface_LobbyManager.call(_movie, method_hash, args, nargs);
        }

        let args_arr = &mut *args;
        if method_hash == *UPDATE_OPTION || method_hash == *UPDATE_COLOUR_OPTION {
            let id_value = args_arr.get_unchecked(0);
            let changed_value = args_arr.get_unchecked(1);
            handle_update_option(id_value.get_uint(), changed_value.get_number());
            // Prevents crashing due to nonexistent id as far as game is concerned
            args_arr.get_unchecked_mut(0).set_uint(0);
            return HOOK_ExternalInterface_LobbyManager.call(_movie, *UPDATE_OPTION, args, nargs);
            
        }
        if method_hash == *OPTIONS_ACCEPT {
            let id_value = args_arr.get_unchecked(0);
            handle_button_clicked(id_value.get_uint());
        }

        HOOK_ExternalInterface_LobbyManager.call(_movie, method_hash, args, nargs);
    }
);

fn handle_update_option(id: u32, value: f64) {
    super::emit_value_callback(id, value);
}

fn handle_button_clicked(id: u32) {
    super::emit_button_callback(id);
}

pub(crate) unsafe fn enable() {
    HOOK_ExternalInterface_LobbyManager.enable().unwrap();
}