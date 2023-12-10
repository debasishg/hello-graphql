#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    fields: Vec<String>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Self {
            name, 
            fields: Vec::new(),
        }
    }
    pub fn add_field(&mut self, field: String) {  // Exclusive borrowed read-write access to self
        self.fields.push(field);
    }
}

#[derive(Debug)]
pub struct GraphQLQuery {
    tables: Vec<Table>,
}

impl GraphQLQuery {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
        }
    }

    pub fn add_table (&mut self, table: Table) {
        self.tables.push(table);
    }
}

fn generate_sql_query(table: Table) -> String {
    let mut sql_query = String::from("SELECT ");
    for field in table.fields {
        sql_query.push_str(&field);
        sql_query.push_str(", ");
    }
    sql_query.pop();
    sql_query.pop();
    sql_query.push_str(" FROM ");
    sql_query.push_str(&table.name);
    sql_query.push_str(";");
    sql_query
}

// generate a sql select statement for GraphQLQuery assuming the next table in the vector is a join
fn generate_sql_join_query(query: GraphQLQuery) -> String {
    let mut sql_query = String::from("SELECT ");
    let mut table = query.tables[0].name.clone();
    let all_columns = query.tables[0].fields.clone().into_iter().chain(query.tables[1].fields.clone());
    for column in all_columns {
        sql_query.push_str(&column);
        sql_query.push_str(", ");
    }
    sql_query.pop();
    sql_query.pop();
    sql_query.push_str(" FROM ");
    sql_query.push_str(&table);
    sql_query.push_str(" JOIN ");
    table = query.tables[1].name.clone();
    sql_query.push_str(&table);
    sql_query.push_str(" ON ");
    sql_query.push_str(&query.tables[0].name);
    sql_query.push_str(".");
    sql_query.push_str(&query.tables[0].fields[0]);
    sql_query.push_str(" = ");
    sql_query.push_str(&query.tables[1].name);
    sql_query.push_str(".");
    sql_query.push_str(&query.tables[1].fields[0]);
    sql_query.push_str(";");
    sql_query
}

pub fn generate_query(query: GraphQLQuery) -> String {
    if query.tables.len() == 1 {
        generate_sql_query(query.tables[0].clone())
    } else {
        generate_sql_join_query(query)
    }
}

pub fn strip_after_space(input: &str) -> String {
  if let Some(index) = input.find(' ') {
      let stripped = &input[..index];
      String::from(stripped)
  } else {
      // If there is no space, return the original string
      String::from(input)
  }
}