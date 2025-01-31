use envie::Envie;
use std::env;

pub struct EnvSync {
    envie: Envie,
}

impl EnvSync {
    /// The newly created 'EnvSync' instance loads the '.env' file and manages environmental variables in memory.
    pub fn new() -> Result<Self, String> {
        let envie = Envie::load()?;
        Ok(Self { envie })
    }

    /// Load environment variables from the '.env' file and synchronize them with your system environment.
    pub fn reload(&mut self) -> Result<(), String> {
        self.envie.reload()?;
        self.sync_with_system()?;
        Ok(())
    }

    /// Synchronizes the system environment variable with the environment variable in memory.
    fn sync_with_system(&self) -> Result<(), String> {
        self.envie.export_to_system_env()
    }

    /// Gets values from system environment variables.
    pub fn get_system_env(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// Gets the value from within memory.
    pub fn get_mem_env(&self, key: &str) -> Option<String> {
        self.envie.get(key)
    }

    /// Sets values for both memory and system environment variables.
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.envie.set(key, value)?;
        env::set_var(key, value);
        Ok(())
    }

    /// Remove values from system environment variables and also from the '.env' file.
    pub fn remove(&mut self, key: &str) -> Result<(), String> {
        self.envie.remove(key)?;
        env::remove_var(key);
        Ok(())
    }

    /// Export all in-memory environment variables to system environment variables.
    pub fn export_all_to_system(&self) -> Result<(), String> {
        self.envie.export_to_system_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reload() {
        let mut env_sync = EnvSync::new().unwrap();
        env_sync.set("TEST_KEY", "test_value").unwrap();
        assert_eq!(env_sync.get_mem_env("TEST_KEY"), Some("test_value".to_string()));
        env_sync.reload().unwrap();
        assert_eq!(env_sync.get_system_env("TEST_KEY"), Some("test_value".to_string()));
    }

    #[test]
    fn test_set_and_get() {
        let mut env_sync = EnvSync::new().unwrap();
        env_sync.set("NEW_KEY", "new_value").unwrap();
        assert_eq!(env_sync.get_mem_env("NEW_KEY"), Some("new_value".to_string()));
        assert_eq!(env_sync.get_system_env("NEW_KEY"), Some("new_value".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut env_sync = EnvSync::new().unwrap();
        env_sync.set("REMOVE_KEY", "remove_value").unwrap();
        env_sync.remove("REMOVE_KEY").unwrap();
        assert_eq!(env_sync.get_mem_env("REMOVE_KEY"), None);
        assert_eq!(env_sync.get_system_env("REMOVE_KEY"), None);
    }

    #[test]
    fn test_export_all_to_system() {
        let env_sync = EnvSync::new().unwrap();
        env_sync.export_all_to_system().unwrap();
        assert_eq!(env::var("EXAMPLE_KEY").is_ok(), true);
    }
}
