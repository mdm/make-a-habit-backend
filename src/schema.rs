table! {
    habits (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        start -> Integer,
        duration -> Integer,
        done_count -> Integer,
        done_streak -> Integer,
        active -> Integer,
    }
}

table! {
    recurrences (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Integer,
        day_of_week -> Nullable<Integer>,
        day_of_month -> Nullable<Integer>,
        week_of_month -> Nullable<Integer>,
        day_of_year -> Nullable<Integer>,
        week_of_year -> Nullable<Integer>,
        month_of_year -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    habits,
    recurrences,
);
