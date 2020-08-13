use diesel::Queryable;
use uuid::Uuid;

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
