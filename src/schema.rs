table! {
    links (shortUrl) {
        shortUrl -> Text,
        redirectUrl -> Nullable<Text>,
        photoUrl -> Nullable<Text>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}
