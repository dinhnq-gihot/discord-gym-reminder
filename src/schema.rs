// @generated automatically by Diesel CLI.

diesel::table! {
    exercises (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        impact -> Varchar,
        #[max_length = 255]
        level -> Varchar,
        description -> Text,
        video -> Array<Nullable<Text>>,
        #[max_length = 255]
        male_weight -> Varchar,
        #[max_length = 255]
        female_weight -> Varchar,
        musculature_id -> Int4,
    }
}

diesel::table! {
    musculature (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Int4,
        #[max_length = 50]
        user_id -> Varchar,
        #[max_length = 50]
        channel_id -> Varchar,
        #[max_length = 20]
        day -> Varchar,
        start_time -> Time,
        musculatures -> Array<Nullable<Text>>,
    }
}

diesel::joinable!(exercises -> musculature (musculature_id));

diesel::allow_tables_to_appear_in_same_query!(
    exercises,
    musculature,
    schedules,
);
