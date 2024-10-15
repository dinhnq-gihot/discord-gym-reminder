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
    musculatures (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Int4,
        user_id -> Varchar,
        #[max_length = 50]
        channel_id -> Varchar,
        #[max_length = 20]
        day -> Varchar,
        start_time -> Time,
    }
}

diesel::table! {
    schedules_musculatures (schedule_id, musculature_id) {
        schedule_id -> Int4,
        musculature_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        point -> Int4,
        level -> Int4,
    }
}

diesel::joinable!(exercises -> musculatures (musculature_id));
diesel::joinable!(schedules -> users (user_id));
diesel::joinable!(schedules_musculatures -> musculatures (musculature_id));
diesel::joinable!(schedules_musculatures -> schedules (schedule_id));

diesel::allow_tables_to_appear_in_same_query!(
    exercises,
    musculatures,
    schedules,
    schedules_musculatures,
    users,
);
