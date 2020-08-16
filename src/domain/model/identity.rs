use diesel::Queryable;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct UserId(Uuid);

impl Into<Uuid> for UserId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl Queryable<diesel::sql_types::Uuid, diesel::pg::Pg> for UserId {
    type Row = Uuid;

    fn build(row: Self::Row) -> Self {
        UserId(row)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PostId(Uuid);

impl Into<Uuid> for PostId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl Queryable<diesel::sql_types::Uuid, diesel::pg::Pg> for PostId {
    type Row = Uuid;

    fn build(row: Self::Row) -> Self {
        PostId(row)
    }
}
