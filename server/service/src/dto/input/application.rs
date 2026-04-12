/// 申请加入请求
#[derive(Debug)]
pub struct CreateApplicationRequest {
    pub ip: String,
    pub province: String,
    pub isp: String,
}
