pub struct DataSource {
    pub id: DataSourceId,
    pub kind: DataSourceKind,
    pub name: String,
}

pub struct DataSourceId(pub i64);

pub enum DataSourceKind {
    Device,
    Manual,
    Import,
    System,
}
