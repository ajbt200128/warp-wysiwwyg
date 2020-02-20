use barrel::{Migration, backend::Pg, types};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table("users", |t|{
        t.add_column("name", types::varchar(255));
        t.add_column("age",types::integer());
    });
    m.make::<Pg>()
}
