use rayon::prelude::*;

use std::ffi::CStr;
use std::os::raw::{c_char, c_uint};

fn _word_count(text_corpus: &str, searched_word: &str) -> usize {
    text_corpus
        .par_split_whitespace()
        // .split_whitespace()
        .filter(|word| word == &searched_word)
        .count()
}

#[no_mangle]
pub unsafe extern "C" fn word_count(
    text_corpus: *const c_char,
    searched_word: *const c_char,
) -> c_uint {
    let text_corpus = CStr::from_ptr(text_corpus).to_str().unwrap();
    let searched_word = CStr::from_ptr(searched_word).to_str().unwrap();
    _word_count(text_corpus, searched_word) as c_uint
}

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn word_count_pyo3(text_corpus: &str, searched_word: &str) -> PyResult<usize> {
    Ok(_word_count(text_corpus, searched_word))
}

#[pymodule]
fn lib_word_count_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(word_count_pyo3))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!(_word_count("marcu", "marcus"), 0);
        assert_eq!(_word_count("", "marcus"), 0);
        assert_eq!(_word_count("marcus", ""), 0);
        assert_eq!(_word_count("marcus", "marcus"), 1);
        assert_eq!(_word_count("marcus marcus", "marcus"), 2);
    }
}
