use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::str::FromStr;

use hello_graphql::model::{Table, GraphQLQuery};
use hello_graphql::util::{strip_after_space, DbUrl};
use hello_graphql::sql::new_sql_generator;

#[derive(Parser)]
#[grammar = "graphqlquery.pest"]

pub struct GraphQLParser;

fn main() {
    let unparsed_file = fs::read_to_string("schema.graphql").expect("cannot read file");

    let graphql = GraphQLParser::parse(Rule::graphql, &unparsed_file)
        .expect("unsuccessful parse") 
        .next()
        .unwrap(); 

    let mut graphql_query = GraphQLQuery::new();

    for table_description in graphql.into_inner() {
        let mut table: &str;
        match table_description.as_rule() {
            Rule::table_description => {
              for table_field_list in table_description.into_inner() {
                  let mut inner_rules = table_field_list.into_inner(); 
                  let inner = inner_rules.next();
                  if inner.is_none() {
                      continue;
                  }
                  table = inner.unwrap().as_str();

                  let mut table_struct = Table::new(strip_after_space(table));

                  for field in inner_rules {
                      match field.as_rule() {
                          Rule::table => {
                              graphql_query.add_table(table_struct);
                              table = field.as_str();

                              table_struct = Table::new(strip_after_space(table));
                          }
                          Rule::field => {
                              table_struct.add_field(field.as_str().to_string());
                          }
                          _ => unreachable!(),
                      }
                  }
                  graphql_query.add_table(table_struct);
              }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }

    }
    // generate sql
    match DbUrl::from_str("postgresql://localhost:5432") {
        Ok(db_url) => {
            match new_sql_generator(&db_url) {
                Ok(sql_generator) => {
                    println!("sql {:?}", sql_generator.generate_sql(graphql_query));
                }
                Err(err) => {
                    println!("error generating sql {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("error forming db url {:?}", err);
        }
    }
}