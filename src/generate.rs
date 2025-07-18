pub fn run() -> (crate::puzzle::Square, crate::puzzle::Square) {
    let puzzles = crate::puzzle::load();
    let answers = crate::puzzle::new(&puzzles);
    let client = reqwest::blocking::Client::new();
    let mut prompt = std::fs::read_to_string("prompt.txt")
        .expect("Failed to read prompt.txt.");
    prompt = prompt.replace('\n', " ");
    let prompt: String = prompt
        .trim()
        .chars()
        .filter(|&c| c.is_ascii() && !c.is_ascii_control())
        .collect();

    let mut across: Vec<String> = vec![];
    let mut down: Vec<String> = vec![];
    for word in &answers.across {
        across.push(crate::llm::chat(&client, &(prompt.clone() + word)));
    }
    for word in &answers.down {
        down.push(crate::llm::chat(&client, &(prompt.clone() + word)));
    }
    let questions = crate::puzzle::Square::new(&across, &down);

    (questions, answers)
}
