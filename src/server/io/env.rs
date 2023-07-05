use std::env;

pub fn ensure_env(var_name: &str) -> String {
    env::var(var_name)
        .unwrap_or_else(|_| panic!("{}", &format!("Missing {} environment variable.", var_name)))
}
