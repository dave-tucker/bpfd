// @generated automatically by Diesel CLI.

diesel::table! {
    fentry_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        fn_name -> Text,
    }
}

diesel::table! {
    fexit_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        fn_name -> Text,
    }
}

diesel::table! {
    global_data (id) {
        id -> Integer,
        prog_id -> Integer,
        key -> Text,
        value -> Binary,
    }
}

diesel::table! {
    images (id) {
        id -> Integer,
        registry -> Text,
        repository -> Text,
        name -> Text,
        tag -> Nullable<Text>,
        digest -> Nullable<Text>,
        manifest -> Text,
        bytecode -> Binary,
    }
}

diesel::table! {
    kernel_program_data (id) {
        id -> Integer,
        bpfman_prog_id -> Nullable<Integer>,
        name -> Nullable<Text>,
        program_type -> Binary,
        loaded_at -> Text,
        tag -> Text,
        gpl_compatible -> Bool,
        btf_id -> Nullable<Binary>,
        bytes_xlated -> Binary,
        jited -> Bool,
        bytes_jited -> Binary,
        bytes_memlock -> Binary,
        verified_insns -> Binary,
    }
}

diesel::table! {
    kprobe_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        fn_name -> Text,
        offset -> Text,
        retprobe -> Bool,
        container_pid -> Integer,
    }
}

diesel::table! {
    maps (id) {
        id -> Integer,
        name -> Text,
        bpfman_prog_id -> Integer,
        kernel_map_id -> Integer,
    }
}

diesel::table! {
    maps_to_programs (id) {
        id -> Integer,
        map_id -> Integer,
        prog_id -> Integer,
    }
}

diesel::table! {
    metadata (id) {
        id -> Integer,
        prog_id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    program_data (id) {
        id -> Integer,
        name -> Text,
        kind -> Integer,
        location_filename -> Nullable<Text>,
        location_url -> Nullable<Text>,
        location_image_pull_policy -> Nullable<Text>,
        location_username -> Nullable<Text>,
        location_password -> Nullable<Text>,
        map_owner_id -> Nullable<Integer>,
        map_pin_path -> Text,
        program_bytes -> Binary,
    }
}

diesel::table! {
    tc_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        priority -> Integer,
        iface -> Text,
        current_position -> Integer,
        if_index -> Integer,
        attached -> Bool,
        direction -> Text,
        proceed_on -> Text,
    }
}

diesel::table! {
    tracepoint_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    uprobe_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        fn_name -> Text,
        offset -> Text,
        retprobe -> Bool,
        container_pid -> Integer,
        pid -> Integer,
        target -> Text,
    }
}

diesel::table! {
    xdp_program_data (id) {
        id -> Integer,
        prog_id -> Integer,
        priority -> Integer,
        iface -> Text,
        current_position -> Integer,
        if_index -> Integer,
        attached -> Bool,
        proceed_on -> Text,
    }
}

diesel::joinable!(fentry_program_data -> program_data (prog_id));
diesel::joinable!(fexit_program_data -> program_data (prog_id));
diesel::joinable!(global_data -> program_data (prog_id));
diesel::joinable!(kernel_program_data -> program_data (bpfman_prog_id));
diesel::joinable!(kprobe_program_data -> program_data (prog_id));
diesel::joinable!(maps -> program_data (bpfman_prog_id));
diesel::joinable!(maps_to_programs -> maps (map_id));
diesel::joinable!(maps_to_programs -> program_data (prog_id));
diesel::joinable!(metadata -> program_data (prog_id));
diesel::joinable!(tc_program_data -> program_data (prog_id));
diesel::joinable!(tracepoint_program_data -> program_data (prog_id));
diesel::joinable!(uprobe_program_data -> program_data (prog_id));
diesel::joinable!(xdp_program_data -> program_data (prog_id));

diesel::allow_tables_to_appear_in_same_query!(
    fentry_program_data,
    fexit_program_data,
    global_data,
    images,
    kernel_program_data,
    kprobe_program_data,
    maps,
    maps_to_programs,
    metadata,
    program_data,
    tc_program_data,
    tracepoint_program_data,
    uprobe_program_data,
    xdp_program_data,
);
