/*
id_user serial constraint PK_devtest_id_user primary key,
user_name varchar(30) not null,
user_lastname varchar(30) not null,
user_phone varchar(30) not null,
user_email varchar(30) not null
 */
pub struct Users {
    id_user: i64,
    user_name: String,
    user_lastname: String,
    user_phone: String,
    user_email: String,
}
