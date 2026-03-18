use rust_weather::weather::WeatherData;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("❌ 使用方法: cargo run -- <城市名> <API_KEY>");
        eprintln!("📍 示例: cargo run -- London your_api_key_here");
        std::process::exit(1);
    }

    let city = &args[1];
    let api_key = &args[2];

    println!("🔍 正在获取 {} 的天气数据...", city);

    match WeatherData::fetch_weather(api_key, city).await {
        Ok(weather) => {
            println!("\n✅ 天气数据获取成功！");
            println!("📍 城市: {}", city);
            println!("🌡️  温度: {:.1}°C", weather.temperature);
            println!("☁️  天气: {}", weather.description);
        }
        Err(e) => {
            eprintln!("❌ 错误: {}", e);
            eprintln!("请检查:");
            eprintln!("  1. API Key 是否正确");
            eprintln!("  2. 城市名称是否正确（使用英文）");
            eprintln!("  3. 网络连接是否正常");
            std::process::exit(1);
        }
    }
}
