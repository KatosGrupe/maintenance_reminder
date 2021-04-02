use diesel::sql_types::Text;

sql_function!{
    fn crypt(password_guess: Text, hash: Text) -> Text;
}
