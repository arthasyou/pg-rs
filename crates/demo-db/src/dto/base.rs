use pg_tables::table::dto::Range as TableRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range<T> {
    pub from: Option<T>,
    pub to: Option<T>,
}

impl<T> From<Range<T>> for TableRange<T> {
    fn from(r: Range<T>) -> Self {
        TableRange {
            from: r.from,
            to: r.to,
        }
    }
}

impl<T> From<TableRange<T>> for Range<T> {
    fn from(r: TableRange<T>) -> Self {
        Range {
            from: r.from,
            to: r.to,
        }
    }
}
