CREATE TABLE IF NOT EXISTS program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    kind INTEGER NOT NULL,
    location_filename TEXT,
    location_url TEXT,
    location_image_pull_policy TEXT,
    location_username TEXT,
    location_password TEXT,
    map_owner_id INTEGER,
    map_pin_path TEXT NOT NULL,
    program_bytes BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS kernel_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    bpfman_prog_id INTEGER REFERENCES program_data (id),
    name TEXT,
    program_type BLOB NOT NULL,
    loaded_at TEXT NOT NULL,
    tag TEXT NOT NULL,
    gpl_compatible BOOLEAN NOT NULL,
    btf_id BLOB,
    bytes_xlated BLOB NOT NULL,
    jited BOOLEAN NOT NULL,
    bytes_jited BLOB NOT NULL,
    bytes_memlock BLOB NOT NULL,
    verified_insns BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS xdp_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    priority INTEGER NOT NULL,
    iface TEXT NOT NULL,
    current_position INTEGER NOT NULL,
    if_index INTEGER NOT NULL,
    attached BOOLEAN NOT NULL,
    proceed_on TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tc_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    priority INTEGER NOT NULL,
    iface TEXT NOT NULL,
    current_position INTEGER NOT NULL,
    if_index INTEGER NOT NULL,
    attached BOOLEAN NOT NULL,
    direction TEXT NOT NULL,
    proceed_on TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tracepoint_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kprobe_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    fn_name TEXT NOT NULL,
    offset
        TEXT NOT NULL,
        retprobe BOOLEAN NOT NULL,
        container_pid INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS uprobe_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    fn_name TEXT NOT NULL,
    offset
        TEXT NOT NULL,
        retprobe BOOLEAN NOT NULL,
        container_pid INTEGER NOT NULL,
        pid INTEGER NOT NULL,
        target TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fentry_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    fn_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fexit_program_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    fn_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS global_data (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS metadata (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maps (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    bpfman_prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE,
    kernel_map_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS maps_to_programs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    map_id INTEGER NOT NULL REFERENCES maps (id) ON DELETE CASCADE,
    prog_id INTEGER NOT NULL REFERENCES program_data (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS images (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    registry TEXT NOT NULL,
    repository TEXT NOT NULL,
    name TEXT NOT NULL,
    tag TEXT,
    digest TEXT,
    manifest TEXT NOT NULL,
    bytecode BLOB NOT NULL
)
