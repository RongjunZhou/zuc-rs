use zuc::{ZucConfig, ZucResult};
use zuc::bash::encrypt_bash;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_file_encryption() -> ZucResult<()> {
    // 创建测试文件
    let test_file = "test_encryption.txt";
    let test_content = "Hello, ZUC encryption!";
    
    // 写入测试内容
    let mut file = File::create(test_file)?;
    file.write_all(test_content.as_bytes())?;
    
    // 加密文件
    encrypt_bash::encrypt_file(test_file)?;
    
    // 解密文件
    encrypt_bash::decrypt_file(test_file)?;
    
    // 验证内容
    let content = fs::read_to_string(test_file)?;
    assert_eq!(content, test_content);
    
    // 清理
    fs::remove_file(test_file)?;
    
    Ok(())
}

#[test]
fn test_config_default() {
    let config = ZucConfig::default();
    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.encryption.count, 0x66035492);
}

#[test]
fn test_config_from_env() -> ZucResult<()> {
    // 设置环境变量
    std::env::set_var("ZUC_HOST", "0.0.0.0");
    std::env::set_var("ZUC_PORT", "9090");
    
    let config = ZucConfig::from_env()?;
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 9090);
    
    // 清理环境变量
    std::env::remove_var("ZUC_HOST");
    std::env::remove_var("ZUC_PORT");
    
    Ok(())
}