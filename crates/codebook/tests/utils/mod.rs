use std::sync::Arc;

use codebook::Codebook;

pub fn get_processor() -> Codebook {
    let config = Arc::new(codebook_config::CodebookConfig::default());
    config
        .add_ignore("**/ignore.txt")
        .expect("Should ignore file");
    let dict = Codebook::new(config).unwrap();
    dict
}

#[allow(dead_code)]
pub fn init_logging() {
    let _ = env_logger::builder().is_test(true).try_init();
}
