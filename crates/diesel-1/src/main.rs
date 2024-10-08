use diesel::{prelude::*, sql_types::Integer};

use self::schema::translations;

mod model;
mod schema;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = translations)]
pub struct Translation {
  word_id: Integer,
  translation_id: Integer,
  language: model::Language,
}

fn main() {
  let database_url =
    std::env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
  let conn =
    &mut PgConnection::establish(&database_url)
      .unwrap_or_else(|e| {
        panic!(
          "Error connecting to {}: {}",
          database_url, e
        )
      });

  let _ =
    diesel::insert_into(translations::table)
      .values(&Translation {
        word_id: 1,
        translation_id: 1,
        language: model::Language::En,
      })
      .execute(conn);

  let t = translations::table
    .select((
      translations::word_id,
      translations::translation_id,
      translations::language,
    ))
    .get_results::<Translation>(conn)
    .expect("select");
  println!("{t:?}");
}
