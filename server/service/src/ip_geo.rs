use ip2region::{CachePolicy, Searcher};
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Mutex;

pub struct IpGeoInfo {
    pub country: String,
    pub province: String,
    pub isp: String,
}

pub struct IpSearcher {
    v4: Option<Searcher>,
    v6: Option<Searcher>,
}

lazy_static! {
    static ref SEARCHER: Mutex<IpSearcher> = Mutex::new(IpSearcher { v4: None, v6: None });
}

/// 初始化 IP 搜索器
///
/// # Arguments
/// * `v4_path` - IPv4 数据库文件路径
/// * `v6_path` - IPv6 数据库文件路径
///
/// # Returns
/// * `Ok(())` - 至少一个数据库加载成功
/// * `Err(String)` - 两个数据库都加载失败
pub fn init_searcher(v4_path: &str, v6_path: &str) -> Result<(), String> {
    let mut v4_searcher = None;
    let mut v6_searcher = None;
    let mut errors = vec![];

    // 尝试加载 IPv4 数据库
    match Searcher::new(v4_path.to_owned(), CachePolicy::VectorIndex) {
        Ok(s) => {
            eprintln!("Loaded IPv4 database: {}", v4_path);
            v4_searcher = Some(s);
        }
        Err(e) => {
            eprintln!("Warning: Failed to load IPv4 database: {}", e);
            errors.push(format!("IPv4: {}", e));
        }
    }

    // 尝试加载 IPv6 数据库
    match Searcher::new(v6_path.to_owned(), CachePolicy::VectorIndex) {
        Ok(s) => {
            eprintln!("Loaded IPv6 database: {}", v6_path);
            v6_searcher = Some(s);
        }
        Err(e) => {
            eprintln!("Warning: Failed to load IPv6 database: {}", e);
            errors.push(format!("IPv6: {}", e));
        }
    }

    // 至少一个数据库加载成功才能继续
    if v4_searcher.is_none() && v6_searcher.is_none() {
        return Err(format!(
            "Failed to load both databases: {}",
            errors.join(", ")
        ));
    }

    let mut searcher = SEARCHER.lock().unwrap();
    searcher.v4 = v4_searcher;
    searcher.v6 = v6_searcher;

    Ok(())
}

/// 判断 IP 地址类型
fn is_ipv6(ip: &str) -> bool {
    ip.parse::<std::net::Ipv6Addr>().is_ok()
}

fn is_ipv4(ip: &str) -> bool {
    ip.parse::<std::net::Ipv4Addr>().is_ok()
}

/// 解析 IP 地址获取地理位置信息
pub fn parse_ip(ip: &str) -> Option<IpGeoInfo> {
    let searcher = SEARCHER.lock().unwrap();

    // 根据 IP 类型选择对应的数据库
    let result = if is_ipv6(ip) {
        searcher.v6.as_ref()?.search(ip).ok()?
    } else if is_ipv4(ip) {
        searcher.v4.as_ref()?.search(ip).ok()?
    } else {
        return None;
    };

    // 官方库返回格式: "国家|省份|城市|运营商|国家代码"
    // 例如: "中国|江苏省|0|联通|CN"
    let parts: Vec<&str> = result.split('|').collect();
    if parts.len() < 5 {
        return None;
    }

    Some(IpGeoInfo {
        country: parts[0].to_string(),
        province: parts[1].to_string(),
        isp: normalize_isp(parts[3]),
    })
}

/// 标准化运营商名称
fn normalize_isp(isp: &str) -> String {
    match isp {
        "联通" => "联通".to_string(),
        "电信" => "电信".to_string(),
        "移动" => "移动".to_string(),
        "铁通" => "铁通".to_string(),
        "广电" => "广电".to_string(),
        _ => "其他".to_string(),
    }
}

/// 检查是否为申请者机器名称格式
/// 格式：省份(2-4字) + 运营商(2-4字) + 3位数字
pub fn is_applicant_machine(name: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^[\u4e00-\u9fa5]{2,4}(联通|电信|移动|铁通|广电)\d{3}$").unwrap();
    }
    RE.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_applicant_machine() {
        assert!(is_applicant_machine("北京联通001"));
        assert!(is_applicant_machine("上海电信002"));
        assert!(is_applicant_machine("内蒙古移动010"));
        assert!(!is_applicant_machine("test-server"));
        assert!(!is_applicant_machine("北京联通01")); // 只有2位数字
        assert!(!is_applicant_machine("北京联通0001")); // 4位数字
    }

    #[test]
    fn test_ip_version_check() {
        assert!(is_ipv4("192.168.1.1"));
        assert!(is_ipv4("1.2.3.4"));
        assert!(!is_ipv4("2001:db8::1"));

        assert!(is_ipv6("2001:db8::1"));
        assert!(is_ipv6("::1"));
        assert!(!is_ipv6("192.168.1.1"));
    }
}
