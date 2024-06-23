//! TODO

//---------------------------------------------------------------------------------------------------- Use

use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::api::client::filter::Filter;

use crate::constants::CONFIG;

//----------------------------------------------------------------------------------------------------
/// Set the custom panic hook.
#[cold]
#[inline(never)]
pub fn sync_settings() -> SyncSettings {
    let filter = {
        let mut account_data_filter = Filter::empty();
        account_data_filter.senders = Some(CONFIG.allowed_users.clone());

        let presence_filter = Filter::ignore_all();

        let mut filter_definition =
            matrix_sdk::ruma::api::client::filter::FilterDefinition::empty();
        filter_definition.account_data = account_data_filter;
        filter_definition.presence = presence_filter;

        matrix_sdk::ruma::api::client::sync::sync_events::v3::Filter::FilterDefinition(
            filter_definition,
        )
    };

    SyncSettings::new().filter(filter).full_state(false)
}
