mod text_analysis;
mod errors;

use std::{io, thread};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use text_analysis::{CountWords, CommonWordFinder, ConcordanceFinder, TextAnalysis, TextAnalysisResultType};
use errors::TextAnalysisError;

fn main() {
    println!("Enter pathfile for textfile:");

    // File path to the text file
    let mut file_path: String = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");

    // The ownership of the String returned by read_file_contents is transferred to 'contents'
    match read_file_contents(file_path) {
        Ok(contents)  => {
            let contents: Arc<String> = Arc::new(contents); // Arc allows multiple threads to share ownership of the contents
            loop {
                println!("Enter a command (type 'quit' or 'q' to exit):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                match perform_action(&input, &contents) {
                    Ok(_) => continue,
                    Err(TextAnalysisError::QuitCommand) => break,
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => eprintln!("Error reading file: {:?}", err)
    }
}

// Function for reading the contents of a file
fn read_file_contents(file_path: String) -> Result<String, TextAnalysisError> {
    match fs::read_to_string(file_path.trim()) {
        Ok(contents) => Ok(contents),
        Err(_err) => Err(TextAnalysisError::FileReadError)
    }
}

// Function for performing an action based on user input
fn perform_action (input: &str, contents: &Arc<String>) -> Result<(), TextAnalysisError> {
    match input.trim().to_lowercase().as_str() {
        "contents" => {
            // Prints the contents in the file
            println!("Contents: \n{:?}", &contents);
        }
        "count" => {
            // CountWords borrows contents, without taking ownership of the data.
            // This is done with Arc::clone(&contents) to increment the reference count.
            // The data is therefore accessible to other parts of the program.
            let count_word_fn: CountWords = CountWords::new(Arc::clone(&contents));
            let count_result:TextAnalysisResultType<Option<String>> = count_word_fn.get_result();

            let count: String = match count_result {
                Ok(Some(count)) => count,
                Ok(None) => {
                    println!("No words found");
                    return Ok(());
                },
                Err(_err) => return Err(TextAnalysisError::WordCountError)
            };
            println!("Count: {count}");
        }
        "common" => {
            // CommonWordFinder borrows contents, using Arc::clone to increment the reference count.
            let common_words_fn: CommonWordFinder = CommonWordFinder::new(Arc::clone(&contents));
            let common_words_result: TextAnalysisResultType<Option<HashMap<String, i32>>> = common_words_fn.get_result();
            match common_words_result {
                Ok(Some(common_words)) => println!("Common Words: {:?}", common_words),
                Ok(None) =>  println!("No common words found"),
                Err(_err) => return Err(TextAnalysisError::CommonWordError)
            }

        }
        "concorde" => {
            // ConcordanceFinder borrows contents, using Arc::clone to increment the reference count.
            let concorde_fn: ConcordanceFinder = ConcordanceFinder::new(Arc::clone(&contents), 2, 2);

            let concorde_finder_result = concorde_fn.get_result();
            match concorde_finder_result{
                Ok(Some(concorde_result)) => {
                    for (word, count) in concorde_result.iter() {
                        println!("{}: {}", word, count)
                    }
                }
                Ok(None) => {
                    println!("Concordance is empty");
                }
                Err(_err) => return Err(TextAnalysisError::ConcordanceError)
            }
        }
        "all" => {
            // Spawn a thread to count words
            let count_word_fn = CountWords::new(Arc::clone(&contents));
            let count_thread_job = thread::spawn(move || -> TextAnalysisResultType<Option<String>> {
                count_word_fn.get_result()
            });

            // Spawn a thread to find common words
            let common_words_fn = CommonWordFinder::new(Arc::clone(&contents));
            let common_thread_job = thread::spawn(move || -> TextAnalysisResultType<Option<HashMap<String, i32>>> {
                common_words_fn.get_result()
            });

            // Spawn a thread to find concordance
            let concorde_fn = ConcordanceFinder::new(Arc::clone(&contents), 2, 2);
            let concorde_thread_job = thread::spawn(move || -> TextAnalysisResultType<Option<HashMap<String, usize>>> {
                concorde_fn.get_result()
            });

            // Wait for the count thread to finish and print the result
            match count_thread_job.join() {
                Ok(result) => match result {
                    Ok(Some(count)) => println!("Count: {}", count),
                    Ok(None) => println!("No words found"),
                    Err(_err) => return Err(TextAnalysisError::WordCountError),
                },
                Err(_err) => return Err(TextAnalysisError::FailedToJoinThreadError),
            }

            // Wait for the common words thread to finish and print the result
            match common_thread_job.join() {
                Ok(result) => match result {
                    Ok(Some(common)) => println!("Common words: {:?}", common),
                    Ok(None) => println!("No words found"),
                    Err(_err) => return Err(TextAnalysisError::CommonWordError)
                },
                Err(_err) => return Err(TextAnalysisError::FailedToJoinThreadError)
            }

            // Wait for the concordance thread to finish and print the result
            match concorde_thread_job.join() {
                Ok(result) => match result {
                    Ok(Some(concorde_result)) => {
                        for (word, count) in concorde_result.iter() {
                            println!("{}: {}", word, count)
                        }
                    }
                    Ok(None) => {
                        println!("Concordance is empty");
                    }
                    Err(_err) => return Err(TextAnalysisError::ConcordanceError)
                },
                Err(_err) => return Err(TextAnalysisError::FailedToJoinThreadError)
            }

        }
        "quit" | "q" => {
            println!("Shutting down.");
            return Err(TextAnalysisError::QuitCommand);
        }
        _ => {}
    }
    Ok(())
}

