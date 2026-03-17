#[cfg(feature = "server")]
mod server_impl {
    use std::sync::{OnceLock, Mutex};
    use crate::models::HighScore;

    // In-memory store for high scores
    pub static DATA: OnceLock<Mutex<Vec<HighScore>>> = OnceLock::new();

    pub fn get_data() -> &'static Mutex<Vec<HighScore>> {
        DATA.get_or_init(|| {
            Mutex::new(vec![
                HighScore { id: 1, player_name: "Alice".to_string(), score: 100, time_seconds: 45 },
                HighScore { id: 2, player_name: "Bob".to_string(), score: 80, time_seconds: 60 },
            ])
        })
    }

    pub async fn init_db() {
        // No-op for in-memory, but keeps the API consistent
        get_data();
        println!("In-memory database initialized.");
    }
}

#[cfg(feature = "server")]
pub use server_impl::*;

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db() {
        init_db().await;
        let data = get_data().lock().unwrap();
        assert!(data.len() >= 2);
    }

    #[test]
    fn test_get_data_is_singleton() {
        let data1 = get_data();
        let data2 = get_data();
        assert!(std::ptr::eq(data1, data2));
    }
}
