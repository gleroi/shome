table! {
    routes (route_id) {
        route_id -> Text,
        agency_id -> Nullable<Text>,
        route_short_name -> Nullable<Text>,
        route_long_name -> Nullable<Text>,
        route_desc -> Nullable<Text>,
        route_type -> Nullable<Integer>,
        route_url -> Nullable<Text>,
        route_color -> Nullable<Text>,
        route_text_color -> Nullable<Text>,
    }
}

table! {
    trips (trip_id) {
        route_id -> Nullable<Text>,
        service_id -> Nullable<Integer>,
        trip_id -> Text,
        trip_headsign -> Nullable<Text>,
        direction_id -> Nullable<Integer>,
        block_id -> Nullable<Text>,
        shape_id -> Nullable<Text>,
    }
}

joinable!(trips -> routes (route_id));

allow_tables_to_appear_in_same_query!(
    routes,
    trips,
);
