use core::fmt;

use serde_json::Value as JsonValue;
use time::PrimitiveDateTime;

/// DataSource 表示：
/// 一条健康数据的来源背景
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSource {
    /// 系统内稳定的数据来源标识
    pub id: DataSourceId,

    /// 来源类型（设备 / 手工 / 导入 / 系统）
    pub kind: DataSourceKind,

    /// 来源名称（人类可读）
    pub name: String,

    /// 可选的来源元信息（JSON）
    pub metadata: Option<JsonValue>,

    /// 创建时间（审计用途）
    pub created_at: PrimitiveDateTime,
}

/// 创建 DataSource 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDataSource {
    pub kind: DataSourceKind,
    pub name: String,
    pub metadata: Option<JsonValue>,
}

/// 查询 DataSource 的输入参数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListDataSource {
    pub kind: Option<DataSourceKind>,
}

/// DataSource 的强类型 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataSourceId(pub i64);

/// DataSource 的语义类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSourceKind {
    Device,
    Manual,
    Import,
    System,

    /// 未来扩展用
    Other(String),
}

impl fmt::Display for DataSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSourceKind::Device => write!(f, "device"),
            DataSourceKind::Manual => write!(f, "manual"),
            DataSourceKind::Import => write!(f, "import"),
            DataSourceKind::System => write!(f, "system"),
            DataSourceKind::Other(value) => write!(f, "{}", value),
        }
    }
}

impl From<&str> for DataSourceKind {
    fn from(value: &str) -> Self {
        match value {
            "device" => DataSourceKind::Device,
            "manual" => DataSourceKind::Manual,
            "import" => DataSourceKind::Import,
            "system" => DataSourceKind::System,
            other => DataSourceKind::Other(other.to_string()),
        }
    }
}

impl From<String> for DataSourceKind {
    fn from(value: String) -> Self {
        DataSourceKind::from(value.as_str())
    }
}
