// @generated automatically by Diesel CLI.

use super::schema::conversation_list;

diesel::table! {
    association_state (inbox_id, sequence_id) {
        inbox_id -> Text,
        sequence_id -> BigInt,
        state -> Binary,
    }
}

diesel::table! {
    consent_records (entity_type, entity) {
        entity_type -> Integer,
        state -> Integer,
        entity -> Text,
        consented_at_ns -> BigInt,
    }
}

diesel::table! {
    group_intents (id) {
        id -> Integer,
        kind -> Integer,
        group_id -> Binary,
        data -> Binary,
        state -> Integer,
        payload_hash -> Nullable<Binary>,
        post_commit_data -> Nullable<Binary>,
        publish_attempts -> Integer,
        staged_commit -> Nullable<Binary>,
        published_in_epoch -> Nullable<BigInt>,
        should_push -> Bool,
    }
}

diesel::table! {
    group_messages (id) {
        id -> Binary,
        group_id -> Binary,
        decrypted_message_bytes -> Binary,
        sent_at_ns -> BigInt,
        kind -> Integer,
        sender_installation_id -> Binary,
        sender_inbox_id -> Text,
        delivery_status -> Integer,
        content_type -> Integer,
        version_minor -> Integer,
        version_major -> Integer,
        authority_id -> Text,
        reference_id -> Nullable<Binary>,
        sequence_id -> Nullable<BigInt>,
        originator_id -> Nullable<BigInt>,
    }
}

diesel::table! {
    groups (id) {
        id -> Binary,
        created_at_ns -> BigInt,
        membership_state -> Integer,
        installations_last_checked -> BigInt,
        added_by_inbox_id -> Text,
        welcome_id -> Nullable<BigInt>,
        rotated_at_ns -> BigInt,
        conversation_type -> Integer,
        dm_id -> Nullable<Text>,
        last_message_ns -> Nullable<BigInt>,
        message_disappear_from_ns -> Nullable<BigInt>,
        message_disappear_in_ns -> Nullable<BigInt>,
        paused_for_version -> Nullable<Text>,
        maybe_forked -> Bool,
        fork_details -> Text,
        sequence_id -> Nullable<BigInt>,
        originator_id -> Nullable<BigInt>,
    }
}

diesel::table! {
    identity (rowid) {
        inbox_id -> Text,
        installation_keys -> Binary,
        credential_bytes -> Binary,
        rowid -> Nullable<Integer>,
    }
}

diesel::table! {
    identity_cache (identity, identity_kind) {
        inbox_id -> Text,
        identity -> Text,
        identity_kind -> Integer,
    }
}

diesel::table! {
    identity_updates (inbox_id, sequence_id) {
        inbox_id -> Text,
        sequence_id -> BigInt,
        server_timestamp_ns -> BigInt,
        payload -> Binary,
    }
}

diesel::table! {
    key_package_history (id) {
        id -> Integer,
        key_package_hash_ref -> Binary,
        created_at_ns -> BigInt,
    }
}

diesel::table! {
    openmls_key_store (key_bytes) {
        key_bytes -> Binary,
        value_bytes -> Binary,
    }
}

diesel::table! {
    openmls_key_value (version, key_bytes) {
        version -> Integer,
        key_bytes -> Binary,
        value_bytes -> Binary,
    }
}

diesel::table! {
    processed_device_sync_messages (message_id) {
        message_id -> Binary,
    }
}

diesel::table! {
    refresh_state (entity_id, entity_kind) {
        entity_id -> Binary,
        entity_kind -> Integer,
        cursor -> BigInt,
    }
}

diesel::table! {
    user_preferences (id) {
        id -> Integer,
        hmac_key -> Nullable<Binary>,
        hmac_key_cycled_at_ns -> Nullable<BigInt>,
    }
}

diesel::joinable!(group_intents -> groups (group_id));
diesel::joinable!(group_messages -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    association_state,
    consent_records,
    group_intents,
    group_messages,
    groups,
    identity,
    identity_cache,
    identity_updates,
    key_package_history,
    openmls_key_store,
    openmls_key_value,
    processed_device_sync_messages,
    refresh_state,
    user_preferences,
    conversation_list
);
