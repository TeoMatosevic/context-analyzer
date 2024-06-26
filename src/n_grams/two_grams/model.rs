use crate::{
    db::{GET_BY_FIRST_2, GET_BY_SECOND_2, GET_FREQ_2},
    n_grams::{Printable, Queryable},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a two-gram.
///
/// # Fields
///
/// * `word1` - The first word of the two-gram.
/// * `word2` - The second word of the two-gram.
///
/// # Implements
///
/// * `Queryable` - Provides methods to query the database.
/// * `Printable` - Provides method for printing.   
#[derive(Clone, Serialize, Deserialize)]
pub struct TwoGramInput {
    pub word1: String,
    pub word2: String,
}

impl TwoGramInput {
    /// Creates a new `TwoGramInput` from the given query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query that contains the two-gram.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `TwoGramInput` if the query is valid, otherwise a `String` with the error message.
    pub fn from(query: &HashMap<String, String>) -> Result<TwoGramInput, String> {
        let word1 = match query.get("word1") {
            Some(word1) => word1,
            None => return Err("word1 is required".to_string()),
        };

        let word2 = match query.get("word2") {
            Some(word2) => word2,
            None => return Err("word2 is required".to_string()),
        };

        Ok(TwoGramInput {
            word1: word1.to_string(),
            word2: word2.to_string(),
        })
    }
}

impl Queryable for TwoGramInput {
    fn to_vec(&self) -> Vec<&str> {
        vec![&self.word1, &self.word2]
    }

    fn get_query(&self, index: Option<i32>) -> Result<&str, String> {
        match index {
            Some(index) => match index {
                1 => Ok(GET_BY_SECOND_2),
                2 => Ok(GET_BY_FIRST_2),
                _ => Err("Invalid index".to_string()),
            },
            None => Ok(GET_FREQ_2),
        }
    }

    fn get_input(&self, index: i32) -> Result<Vec<&String>, String> {
        match index {
            1 => Ok(vec![&self.word2]),
            2 => Ok(vec![&self.word1]),
            _ => Err("Invalid index".to_string()),
        }
    }

    fn get_word(&self, index: i32) -> Result<&String, String> {
        match index {
            1 => Ok(&self.word1),
            2 => Ok(&self.word2),
            _ => Err("Invalid index".to_string()),
        }
    }
}

impl Printable for TwoGramInput {
    fn print(&self) -> String {
        format!("{} {}", self.word1, self.word2)
    }
}

/// Validates the indexes.
///
/// # Arguments
///
/// * `indexes` - The indexes to validate.
///
/// # Returns
///
/// A `Result` containing `()` if the indexes are valid, otherwise a `String` with the error message.
pub fn validate(indexes: &Vec<i32>) -> Result<(), String> {
    let mut new = vec![];
    for index in indexes {
        if *index < 1 || *index > 2 {
            return Err("Invalid index".to_string());
        }
        if new.contains(index) {
            return Err("Invalid index".to_string());
        }
        new.push(*index);
    }
    if new.len() != indexes.len() {
        return Err("Invalid index".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_gram_input_from() {
        let mut query = HashMap::new();

        query.insert("word1".to_string(), "word1".to_string());
        query.insert("word2".to_string(), "word2".to_string());

        let result = TwoGramInput::from(&query).unwrap();

        assert_eq!(result.word1, "word1");
    }

    #[test]
    fn test_two_gram_input_from_missing_word1() {
        let mut query = HashMap::new();

        query.insert("word2".to_string(), "word2".to_string());

        let result = TwoGramInput::from(&query);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_two_gram_to_vec() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.to_vec();

        assert_eq!(result, vec!["word1", "word2"]);
    }

    #[test]
    fn test_two_gram_get_query() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.get_query(Some(1)).unwrap();

        assert_eq!(result, GET_BY_SECOND_2);
    }

    #[test]
    fn test_two_gram_get_query_freq() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.get_query(None).unwrap();

        assert_eq!(result, GET_FREQ_2);
    }

    #[test]
    fn test_two_gram_get_input() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.get_input(1).unwrap();

        assert_eq!(result, vec![&"word2"]);
    }

    #[test]
    fn test_two_gram_get_word() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.get_word(1).unwrap();

        assert_eq!(result, &"word1");
    }

    #[test]
    fn test_two_gram_print() {
        let input = TwoGramInput {
            word1: "word1".to_string(),
            word2: "word2".to_string(),
        };

        let result = input.print();

        assert_eq!(result, "word1 word2");
    }

    #[test]
    fn test_validate() {
        let indexes = vec![1, 2];

        let result = validate(&indexes);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_validate_fail_index_out_of_bounds() {
        let indexes = vec![1, 3];

        let result = validate(&indexes);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_validate_fail_index_duplicate() {
        let indexes = vec![1, 1];

        let result = validate(&indexes);

        assert_eq!(result.is_err(), true);
    }
}
