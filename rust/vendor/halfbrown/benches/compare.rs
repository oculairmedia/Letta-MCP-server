use criterion::*;

const NAMES: [&str; 162] = [
    // Some data taken from the twitter json
    "contributors",
    "coordinates",
    "created_at",
    "entities",
    "favorite_count",
    "favorited",
    "geo",
    "id",
    "id_str",
    "in_reply_to_screen_name",
    "in_reply_to_status_id",
    "in_reply_to_status_id_str",
    "in_reply_to_user_id",
    "in_reply_to_user_id_str",
    "lang",
    "metadata",
    "place",
    "possibly_sensitive",
    "retweet_count",
    "retweeted",
    "retweeted_status",
    "source",
    "text",
    "truncated",
    "user",
    "completed_in",
    "count",
    "max_id",
    "max_id_str",
    "next_results",
    "query",
    "refresh_url",
    "since_id",
    "since_id_str",
    // Some random names
    "Molly",
    "Queenie",
    "Fredrick",
    "Elwanda",
    "Philip",
    "Idella",
    "Cinderella",
    "Edith",
    "Halina",
    "Marchelle",
    "Era",
    "Louann",
    "Sheryll",
    "Arlinda",
    "Keira",
    "Nickie",
    "Shondra",
    "Andy",
    "Kelli",
    "Crissy",
    "Sherita",
    "Samara",
    "Brock",
    "Bridget",
    "Mauricio",
    "Marcus",
    // some more json keys
    "data",
    "name",
    "rows",
    "rows_per_beat",
    "rows_per_measure",
    // Names again
    "Jeannetta",
    "Vickey",
    "Marco",
    "Branda",
    "Patricia",
    "Alexis",
    "Yoko",
    "Milford",
    "Sandra",
    "Cherie",
    // Back to keys
    "contributors_enabled",
    "created_at",
    "default_profile",
    "default_profile_image",
    "description",
    "entities",
    "favourites_count",
    "follow_request_sent",
    "followers_count",
    "following",
    "friends_count",
    "geo_enabled",
    "id",
    "id_str",
    "is_translation_enabled",
    "is_translator",
    "lang",
    "listed_count",
    "location",
    "name",
    "notifications",
    "profile_background_color",
    "profile_background_image_url",
    "profile_background_image_url_https",
    "profile_background_tile",
    "profile_banner_url",
    "profile_image_url",
    "profile_image_url_https",
    "profile_link_color",
    "profile_sidebar_border_color",
    "profile_sidebar_fill_color",
    "profile_text_color",
    "profile_use_background_image",
    "protected",
    "screen_name",
    "statuses_count",
    "time_zone",
    "url",
    "utc_offset",
    "verified",
    // Names
    "Dulcie",
    "Nancey",
    "Johnson",
    "Sibyl",
    "Janice",
    "Stevie",
    "Reatha",
    "Norbert",
    "Jessi",
    "Kristen",
    "Tarah",
    "Narcisa",
    "Iva",
    "Aleen",
    // more keys
    "default_filter_cutoff",
    "default_filter_cutoff_enabled",
    "default_filter_mode",
    "default_filter_resonance",
    "default_filter_resonance_enabled",
    "default_pan",
    "duplicate_check_type",
    "duplicate_note_action",
    "fadeout",
    "global_volume",
    "graph_insert",
    "legacy_filename",
    "midi_bank",
    "midi_channel",
    "midi_drum_set",
    "midi_program",
    "name",
    "new_note_action",
    "note_map",
    "panning_envelope",
    "pitch_envelope",
    "pitch_pan_center",
    "pitch_pan_separation",
    "pitch_to_tempo_lock",
    "random_cutoff_weight",
    "random_pan_weight",
    "random_resonance_weight",
    "random_volume_weight",
    "sample_map",
    "tuning",
    "volume_envelope",
    "volume_ramp_down",
    "volume_ramp_up",
];

#[derive(Default)]
struct BenchInput {
    num_inserts: usize,
    initial_cap: usize,
}

impl std::fmt::Display for BenchInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "inserts={}/capacity={}",
            self.num_inserts, self.initial_cap
        )
    }
}

impl BenchInput {
    fn new(num_inserts: usize) -> Self {
        Self {
            num_inserts,
            initial_cap: 0,
        }
    }

    fn new_with_capacity(num_inserts: usize, initial_cap: usize) -> Self {
        Self {
            num_inserts,
            initial_cap,
        }
    }
}

fn bench_group(b: &mut Criterion, name: &str, bench_input: BenchInput) {
    let mut group = b.benchmark_group(name);

    let data1: Vec<&'static str> = NAMES
        .iter()
        .cloned()
        .take(bench_input.num_inserts)
        .collect();
    group.bench_function("halfbrown", |b| {
        b.iter_batched(
            || data1.clone(),
            |data| {
                let mut m = halfbrown::HashMap::with_capacity(bench_input.initial_cap);
                for e in data {
                    m.insert(e, e);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("halfbrown(nocheck)", |b| {
        b.iter_batched(
            || data1.clone(),
            |data| {
                let mut m = halfbrown::HashMap::with_capacity(bench_input.initial_cap);
                for e in data {
                    m.insert_nocheck(e, e);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("hashbrown", |b| {
        b.iter_batched(
            || data1.clone(),
            |data| {
                let mut m = hashbrown::HashMap::with_capacity(bench_input.initial_cap);
                for e in data {
                    m.insert(e, e);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("std", |b| {
        b.iter_batched(
            || data1.clone(),
            |data| {
                let mut m = std::collections::HashMap::with_capacity(bench_input.initial_cap);
                for e in data {
                    m.insert(e, e);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_capacity(b: &mut Criterion) {
    bench_group(
        b,
        "insert(5) with capacity",
        BenchInput::new_with_capacity(5, 5),
    );
    bench_group(
        b,
        "insert(9) with capacity",
        BenchInput::new_with_capacity(9, 9),
    );
    bench_group(
        b,
        "insert(17) with capacity",
        BenchInput::new_with_capacity(17, 17),
    );
    bench_group(
        b,
        "insert(33) with capacity",
        BenchInput::new_with_capacity(33, 33),
    );
    bench_group(
        b,
        "insert(65) with capacity",
        BenchInput::new_with_capacity(65, 65),
    );
    bench_group(
        b,
        "insert(129) with capacity",
        BenchInput::new_with_capacity(129, 129),
    );
}

fn bench_alloc(b: &mut Criterion) {
    bench_group(b, "insert(5)", BenchInput::new(5));
    bench_group(b, "insert(9)", BenchInput::new(9));
    bench_group(b, "insert(17)", BenchInput::new(17));
    bench_group(b, "insert(33)", BenchInput::new(33));
    bench_group(b, "insert(65)", BenchInput::new(65));
    bench_group(b, "insert(129)", BenchInput::new(129));
}

criterion_group!(capacity, bench_capacity);
criterion_group!(alloc, bench_alloc);

criterion_main!(capacity, alloc);
