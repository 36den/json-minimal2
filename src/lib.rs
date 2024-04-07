use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String,Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null
}

impl Json {
    pub fn new_object() -> Json {
        Json::Object(HashMap::new())
    }

    pub fn new_array() -> Json {
        Json::Array(Vec::new())
    }

    pub fn string_from(value: &str) -> Json {
        Json::String(String::from(value))
    }

    pub fn is_object(&self) -> bool {
        match self {
            Json::Object(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Json::Array(_) => true,
            _ => false
        }
    }

    // maybe it would be better to just panic
    pub fn insert(&mut self, name: &str, value: Json) {
        match self {
            Json::Object(name_value_pairs) => {

                name_value_pairs.insert(name.to_owned(),value);

            },
            _ => {}
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&Json> {
        match self {
            Json::Object(name_value_pairs) => {
                name_value_pairs.get(name)
            },
            _ => {
                None
            }
        }
    }

    pub fn remove(&mut self, name: &str) {
        match self {
            Json::Object(name_value_pairs) => {

                name_value_pairs.remove(name);

            },
            _ => {}
        }
    }

    pub fn push(&mut self, new_value: Json) {
        match self {
            Json::Array(values) => {

                values.push(new_value);

            },
            _ => {}
        }
    }

    pub fn pull(&self, index: usize) -> Option<&Json> {
        match self {
            Json::Array(values) => {
                values.get(index)
            },
            _ => {
                None
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Json::Object(name_value_pairs) => {
                let mut output = String::new();

                output.push_str("{");

                for (name,value) in name_value_pairs {
                    output.push_str(&format!("\"{}\":{},",name,value.to_string()));
                }

                output.pop();
                
                output.push_str("}");

                output

            },
            Json::Array(values) => {
                let mut output = String::new();

                output.push_str("[");

                for value in values {
                    output.push_str(&value.to_string());
                    output.push_str(",");
                }

                output.pop();

                output.push_str("]");

                output
            },
            Json::String(value) => {
                String::from(&format!("\"{}\"",value))
            },
            Json::Number(value) => {
                value.to_string()
            },
            Json::Bool(value) => {
                if *value == true {
                    return String::from("true");
                } else {
                    return String::from("false");
                }
            },
            Json::Null => {
                String::from("null")
            }
        }
    }

    pub fn parse(input: &str) -> Result<Json,()> {
        let mut input: Vec<char> = input.chars().collect();

        let mut index: usize = 0;

        while index < input.len() {

            let c = &input[index];

            if !c.is_ascii_whitespace() {

                if *c == '{' {
                    return Self::parse_object(&mut input, &mut index);
                }

                if *c == '[' {
                    return Self::parse_array(&mut input, &mut index);
                }

                if *c == '\"' {
                    return Self::parse_string(&mut input, &mut index);
                }

                if c.is_ascii_digit() {
                    return Self::parse_number(&mut input, &mut index);
                }

                if *c == 't' {
                    return Self::parse_true(&mut input, &mut index);
                }
                
                if *c == 'f' {
                    return Self::parse_false(&mut input, &mut index);
                }

                if *c == 'n' {
                    return Self::parse_null(&mut input, &mut index);
                }

            }

            index += 1;
        }

        
            
        return Err(());

    }

    fn parse_object(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {

        *index += 1;

        let mut object = HashMap::new();

        while *index < input.len() {
            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c != '\"' {
                    return Err(());
                } else {
                    break;
                }
            }

            *index += 1;
        }
        
        while *index < input.len(){

            let name = Self::parse_string(input,index)?;

            let c = input[*index];

            if c == ':' {
                *index += 1;
            } else {
                return Err(());
            }

            let value = Self::parse_value(input,index)?;

            let c = input[*index];

            if c == ',' {

                match name {
                    Json::String(name) => {
                        object.insert(name,value);
                    },
                    _ => {
                        unreachable!()
                    }
                }

                *index += 1;

            } else if c == '}' {

                match name {
                    Json::String(name) => {
                        object.insert(name,value);
                    },
                    _ => {
                        unreachable!()
                    }
                }

                *index += 1;

                return Ok(Json::Object(object));
            } else {
                return Err(());
            }
        }

        Err(())

    }

    fn parse_array(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {

        *index += 1;

        let mut array = Vec::<Json>::new();
        
        while *index < input.len(){

            let value = Self::parse_value(input,index)?;

            array.push(value);

            let c = input[*index];

            if c == ',' {
                *index += 1;
            } else if c == ']' {
                *index += 1;
                return Ok(Json::Array(array));
            } else {
                return Err(());
            }
        }

        Err(())
    }

    fn parse_value(input: &mut Vec<char>, index: &mut usize)  -> Result<Json,()> {
        while *index < input.len() {

            let c = &input[*index];

            if !c.is_ascii_whitespace() {

                if *c == '{' {
                    return Self::parse_object(input, index);
                }

                if *c == '[' {
                    return Self::parse_array(input, index);
                }

                if *c == '\"' {
                    return Self::parse_string(input, index);
                }

                if c.is_ascii_digit() {
                    return Self::parse_number(input, index);
                }

                if *c == 't' {
                    return Self::parse_true(input, index);
                }
                
                if *c == 'f' {
                    return Self::parse_false(input, index);
                }

                if *c == 'n' {
                    return Self::parse_null(input, index);
                }

            }

            *index += 1;
        }
   
        return Err(());

    }

    fn parse_string(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {

        *index += 1;

        let mut string = String::new();

        while *index < input.len() {

            let c = input[*index];

            if c != '\"' {
                string.push(c);
            } else {
                break;
            }

            *index += 1;
        }

        while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == '}' || c == ']' || c == ':' {
                    return Ok(Json::String(string));
                }
            }
    
            *index += 1;
        }

        Ok(Json::String(string))
    }

    fn parse_number(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {
        

        let mut number = String::new();

        while *index < input.len() {
            let c = input[*index];

            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                number.push(c);
            } else {
                break;
            }

            *index += 1;
        }

        if let Ok(number) = number.parse::<f64>() {

            while *index < input.len() {
                let c = input[*index];
    
                if !c.is_ascii_whitespace() {
                    if c == ',' || c == ']' || c == '}' {
                        return Ok(Json::Number(number));
                    }
                }
    
                *index += 1;
            }

            return Ok(Json::Number(number));

        } else {
            return Err(());
        }
    }

    fn parse_true(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'r' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'u' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'e' {
            return Err(());
        }

        while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == ']' || c == '}' {
                    return Ok(Json::Bool(true));
                }
            }

            *index += 1;
        }

        Ok(Json::Bool(true))

    }

    fn parse_false(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {
        
        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'a' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'l' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 's' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'e' {
            return Err(());
        }

        while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == ']' || c == '}' {
                    return Ok(Json::Bool(false));
                }
            }

            *index += 1;
        }

        Ok(Json::Bool(false))

    }

    fn parse_null(input: &mut Vec<char>, index: &mut usize) -> Result<Json,()> {
        
        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'u' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'l' {
            return Err(());
        }

        *index += 1;

        if *index >= input.len() {
            return Err(());
        }

        if input[*index] != 'l' {
            return Err(());
        }

        while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == ']' || c == '}' {
                    return Ok(Json::Null);
                }
            }

            *index += 1;
        }

        Ok(Json::Null)

        // Same as above
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_object = Json::new_object();

        my_object.insert("Greeting",Json::String(String::from("Hello, you!")));
        my_object.insert("Amount of days in a week",Json::Number(7.0));

        let mut days_of_the_week = Json::new_array();

        days_of_the_week.push(Json::string_from("Monday"));
        days_of_the_week.push(Json::string_from("Tuesday"));
        days_of_the_week.push(Json::string_from("Wednesday"));
        days_of_the_week.push(Json::string_from("Thursday"));
        days_of_the_week.push(Json::string_from("Friday"));
        days_of_the_week.push(Json::string_from("Saturday"));
        days_of_the_week.push(Json::string_from("Sunday"));

        let mut sub_object = Json::new_object();

        sub_object.insert("Comment", Json::string_from("Have I forgotten anything?"));

        days_of_the_week.push(sub_object);

        my_object.insert("Days of the week",days_of_the_week);

        my_object.insert("True or false",Json::Bool(true));

        my_object.insert("Forgotten",Json::Null);

        let mut sub_object = Json::new_object();

        sub_object.insert("Comment", Json::string_from("This is a comment"));

        my_object.insert("Other",sub_object);

        println!("{}",my_object.to_string());

        let parsed = Json::parse(&my_object.to_string());

        assert_eq!(Ok(my_object),parsed);
    }

    #[test]
    fn it_works2() {
        let mut json = r#"{
            "First": "Line",
            "Second": "Line",
            "Third": "Line"
        }"#;

        let parsed = Json::parse(json);

        let mut compare = Json::new_object();

        compare.insert("First", Json::string_from("Line"));
        compare.insert("Second", Json::string_from("Line"));
        compare.insert("Third", Json::string_from("Line"));

        println!("{:?}",parsed);

        assert_eq!(Ok(compare),parsed);
    }

    #[test]
    fn parse_object() {
        let json = "  {   \"Greeting\"   :   \"Hello, world!\"   } " ;

        let parsed = Json::parse(json);

        let mut check = HashMap::new();
        check.insert("Greeting".to_string(),Json::string_from("Hello, world!"));

        assert_eq!(Ok(Json::Object(check)), parsed);
    }

    #[test]
    fn parse_array() {
        let json = "  [ \"Hello, world!\", 42, true, null ]  ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Array(vec![Json::string_from("Hello, world!"), Json::Number(42.0), Json::Bool(true), Json::Null])),parsed);
    }

    #[test]
    fn parse_string() {
        let json = "  \"Hello, world!\"   ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::string_from("Hello, world!")),parsed);
    }

    #[test]
    fn parse_number() {
        let json = " 1.42 ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Number(1.42)),parsed);

        let json = " 2e2 ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Number(200.0)),parsed);

        let json = " 2E2 ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Number(200.0)),parsed);
    }

    #[test]
    fn parse_bool_true() {
        let json = "  true  ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Bool(true)),parsed);
    }

    #[test]
    fn parse_bool_false() {
        let json = "  false  ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Bool(false)),parsed);
    }

    #[test]
    fn parse_null() {
        let json = "  null  ";

        let parsed = Json::parse(json);

        assert_eq!(Ok(Json::Null),parsed);
    }

}
