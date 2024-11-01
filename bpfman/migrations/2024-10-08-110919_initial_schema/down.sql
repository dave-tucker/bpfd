-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS program_data;

DROP TABLE IF EXISTS kernel_program_data;

DROP TABLE IF EXISTS xdp_program_data;

DROP TABLE IF EXISTS tc_program_data;

DROP TABLE IF EXISTS tracepoint_program_data;

DROP TABLE IF EXISTS kprobe_program_data;

DROP TABLE IF EXISTS tracepoint_program_data;

DROP TABLE IF EXISTS kprobe_program_data;

DROP TABLE IF EXISTS uprobe_program_data;

DROP TABLE IF EXISTS fentry_program_data;

DROP TABLE IF EXISTS fexit_program_data;

DROP TABLE IF EXISTS global_data;

DROP TABLE IF EXISTS metadata;

DROP TABLE IF EXISTS maps;

DROP TABLE IF EXISTS maps_to_programs;

DROP TABLE IF EXISTS images;
