use core::fmt;

use time::PrimitiveDateTime;

/// Subject 表示：
/// 一个可以被健康事实（Observation）指向的“存在主体”
///
/// 注意：
/// - Subject 不是 User
/// - Subject 不包含任何业务资料
/// - Subject 只负责“身份锚点”
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    /// 系统内的稳定身份标识
    pub id: SubjectId,

    /// 主体类型（人 / 设备 / 成员 / 未来扩展）
    pub kind: SubjectKind,

    /// 创建时间（用于审计、排序，不参与业务推理）
    pub created_at: PrimitiveDateTime,
}

/// Subject 的强类型 ID
/// 目的是避免在 domain 中随意传递裸整数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubjectId(pub i64);

/// Subject 的语义类型
///
/// 这是一个“可演进的枚举”，
/// 而不是数据库 enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubjectKind {
    User,
    Member,
    Device,

    /// 未来扩展用
    Other(String),
}

impl fmt::Display for SubjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubjectKind::User => write!(f, "user"),
            SubjectKind::Member => write!(f, "member"),
            SubjectKind::Device => write!(f, "device"),
            SubjectKind::Other(value) => write!(f, "{}", value),
        }
    }
}

impl From<&str> for SubjectKind {
    fn from(value: &str) -> Self {
        match value {
            "user" => SubjectKind::User,
            "member" => SubjectKind::Member,
            "device" => SubjectKind::Device,
            other => SubjectKind::Other(other.to_string()),
        }
    }
}
impl From<String> for SubjectKind {
    fn from(value: String) -> Self {
        SubjectKind::from(value.as_str())
    }
}
