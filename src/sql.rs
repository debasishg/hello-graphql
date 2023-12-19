use crate::model::{GraphQLQuery, Table};
use crate::util::DbUrl;

pub trait SqlGenerator {
    fn generate_sql(&self, query: GraphQLQuery) -> String;
}

struct PostGresSql;
struct MySQL;

impl SqlGenerator for PostGresSql {
    fn generate_sql(&self, query: GraphQLQuery) -> String {
        if query.is_single_table_query() {
            generate_sql_qry(query.head_table().clone())
        } else {
            generate_sql_join_qry(query)
        }
    }
}

impl SqlGenerator for MySQL {
    fn generate_sql(&self, query: GraphQLQuery) -> String {
        if query.is_single_table_query() {
            generate_sql_qry(query.head_table().clone())
        } else {
            generate_sql_join_qry(query)
        }
    }
}

/// generate a sql query for a single table
fn generate_sql_qry(table: Table) -> String {
    let str = format!("SELECT {} FROM {};", table.all_fields(), table.name());
    str
}

/// generate a sql query for a join
fn generate_sql_join_qry(query: GraphQLQuery) -> String {
    let all_columns = query.all_columns();

    let all_columns_str = 
        all_columns
            .iter()
            .map(|field| field.to_string())
            .collect::<Vec<_>>()
            .join(",");

    let join_condition = query.get_join_columns().unwrap();

    let str = format!("SELECT {} FROM {} JOIN {} ON {};", 
        all_columns_str, 
        query.tables()[0].name(), 
        query.tables()[1].name(), 
        join_condition);
    str
}

// &str would also have worked as the argument type, since we have DeRef trait implemented for DbUrl
pub fn new_sql_generator(db_url: &DbUrl) -> Result<Box<dyn SqlGenerator>, &str> {
    if db_url.starts_with("postgresql://") {
        Ok(Box::new(PostGresSql))
    } else if db_url.starts_with("mysql://") {
        Ok(Box::new(MySQL))
    } else {
        Err("Unsupported database".into())
    }
}
