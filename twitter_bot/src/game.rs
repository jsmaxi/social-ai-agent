use llm_chain::parameters;
use llm_chain::step::Step;
use llm_chain::{executor, prompt};
use llm_chain_openai::chatgpt::Executor;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    player_health: i32,
    opponent_health: i32,
    mana: i32,
    hand: Vec<String>,
    board: BoardState,
}

#[derive(Serialize, Deserialize, Debug)]
struct BoardState {
    player: Vec<String>,
    opponent: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameRules {
    win_condition: String,
    mana_rules: String,
    card_types: Vec<CardType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardType {
    name: String,
    description: String,
}

async fn load_rules_from_file(file_path: &str) -> GameRules {
    let data: String = fs::read_to_string(file_path)
        .await
        .expect("Failed to read rules file");
    serde_json::from_str(&data).expect("Failed to parse rules JSON")
}

async fn create_llm_chain() -> Executor {
    let exec: Executor = executor!().unwrap(); // New ChatGPT executor
    exec
}

fn generate_prompt(state: &GameState, rules: &GameRules) -> String {
    let rules_json: String = serde_json::to_string(rules).expect("Failed to serialize rules");
    let state_json: String = serde_json::to_string(state).expect("Failed to serialize state");

    format!(
        "You are playing a TCG. Here are the rules:\n{}\n\nCurrent game state:\n{}\nWhat is the best move? \
        Only use the rules and game state provided. Do not use any external knowledge or data. Give a short answer.",
        rules_json, state_json
    )
}

async fn get_ai_move(exec: Executor, p: String) -> String {
    let step = Step::for_prompt_template(prompt!(p));

    let res = step.run(&parameters!(), &exec).await.unwrap();

    let content = res.to_immediate().await.unwrap().as_content();

    content.to_string()
}

fn remove_prefix_from_string(str: String) -> String {
    let prefix = "Assistant: ";
    str.strip_prefix(prefix).unwrap_or(&str).to_string()
}

pub async fn play(rules_file_path: &str) {
    let rules: GameRules = load_rules_from_file(rules_file_path).await;

    let state: GameState = GameState {
        player_health: 20,
        opponent_health: 25,
        mana: 5,
        hand: vec![
            "Fireball".to_string(),
            "Knight".to_string(),
            "Healing Potion".to_string(),
        ],
        board: BoardState {
            player: vec!["Knight".to_string()],
            opponent: vec!["Dragon".to_string()],
        },
    };

    let chain = create_llm_chain().await;

    let prompt = generate_prompt(&state, &rules);

    let ai_move = get_ai_move(chain, prompt).await;

    let ai_move_processed = remove_prefix_from_string(ai_move);

    println!("AI's move: {}", ai_move_processed);
}
