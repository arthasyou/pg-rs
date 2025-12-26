use thiserror::Error;

/// demo-db 的业务错误（demo 用）
///
/// 约定：
/// - 这里只放“业务层能感知的错误”
/// - 不暴露数据库 / SQL / 连接细节
/// - demo 阶段错误种类保持最少
#[derive(Debug, Error)]
pub enum DemoDbError {
    /// 输入参数非法
    #[error("invalid input")]
    InvalidInput,

    /// 依赖的对象不存在（如 subject / metric）
    #[error("{0} not found")]
    NotFound(&'static str),

    /// 内部错误（demo 兜底）
    #[error("internal error")]
    Internal,
}

pub type Result<T> = std::result::Result<T, DemoDbError>;
