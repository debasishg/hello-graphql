//! Models used by the parser

use inflector::Inflector;

/// structure of a Table
/// a table has a name and a sequence of fields
#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    fields: Vec<String>,
}

/// methods
impl Table {
    /// returns a Table with empty fields given a name
    /// 
    /// ```
    /// use hello_graphql::model::Table;
    /// let table = Table::new("users".to_string());
    /// assert_eq!(table.name(), "users");
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name, 
            fields: Vec::new(),
        }
    }

    /// add a field to the table
    /// 
    /// ```
    /// use hello_graphql::model::Table;
    /// let mut table = Table::new("users".to_string());
    /// table.add_field("id".to_string());
    /// assert_eq!(table.check_field_exists("id"), true);
    /// ```
    pub fn add_field(&mut self, field: String) {  // Exclusive borrowed read-write access to self
        self.fields.push(field);
    }

    /// return the table name as a read-only string
    pub fn name(&self) -> &String { // Shared borrowed read-only access to self
        &self.name
    }

    /// checks if the field exists in the table
    pub fn check_field_exists(&self, field: &str) -> bool {
        self.fields.iter().any(|f| f == field)
    }

}

#[derive(Debug)]
pub struct GraphQLQuery {
    tables: Vec<Table>,
}

// methods
impl GraphQLQuery {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
        }
    }

    pub fn add_table (&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_join_columns(&self) -> Option<String> {
        let pkey = self.tables[0].fields.iter().position(|field| field.to_string() == "id");
        match pkey {
            Some(_pkey) => {
                let table_name_singularized = self.tables[0].name.to_string().to_lowercase().to_singular();
                let foreign_key = format!("{}_id", table_name_singularized);
                let join_condition = format!("{}.{} = {}.{}", self.tables[0].name, "id", self.tables[1].name, foreign_key);
                Some(join_condition)
            }
            None => None
        }
    }
}

/// generate a sql query for a single table
fn generate_sql_qry(table: Table) -> String {
    let fields = 
        table
            .fields
            .iter()
            .map(|field| field.to_string())
            .collect::<Vec<_>>()
            .join(",");

    let str = format!("SELECT {} FROM {};", fields, table.name);
    str
}

/// generate a sql query for a join
fn generate_sql_join_qry(query: GraphQLQuery) -> String {
    let all_columns = 
        query
            .tables[0]
            .fields
            .iter()
            .chain(query.tables[1].fields.iter());

    let all_columns_str = 
        all_columns
            .map(|field| field.to_string())
            .collect::<Vec<_>>()
            .join(",");

    let join_condition = query.get_join_columns().unwrap();

    let str = format!("SELECT {} FROM {} JOIN {} ON {};", 
        all_columns_str, 
        query.tables[0].name, 
        query.tables[1].name, 
        join_condition);
    str
}

/// generate a sql query for a single table or a join
pub fn generate_query(query: GraphQLQuery) -> String {
    if query.tables.len() == 1 {
        generate_sql_qry(query.tables[0].clone())
    } else {
        generate_sql_join_qry(query)
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