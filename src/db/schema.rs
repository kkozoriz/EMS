// @generated automatically by Diesel CLI.

diesel::table! {
    notification_recipients (id) {
        id -> Int4,
        notification_id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        delivery_status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notification_templates (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        subject -> Varchar,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        template_id -> Int4,
        #[max_length = 50]
        status -> Varchar,
        sent_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(notification_recipients -> notifications (notification_id));
diesel::joinable!(notification_recipients -> users (user_id));
diesel::joinable!(notifications -> notification_templates (template_id));

diesel::allow_tables_to_appear_in_same_query!(
    notification_recipients,
    notification_templates,
    notifications,
    users,
);
