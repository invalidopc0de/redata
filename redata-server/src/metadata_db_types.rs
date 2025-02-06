use sea_query::*;

// For example Character table with column id, character, font_size...
pub enum MetadataRun {
    Table,
    Uuid,
    Name,
}

// Mapping between Enum variant and its corresponding string value
impl Iden for MetadataRun {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "redata_runs",
                Self::Uuid => "uuid",
                Self::Name => "name",
            }
        ).unwrap();
    }
}
