impl SubscriberRepository {

    pub fn list_all_as_string() -> Vec<String> {
        return NOTIFICATIONS.read().unwrap()
            .iter().map(|f| format!("{}", f.clone())).collect();
    }
}
