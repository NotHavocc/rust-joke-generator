use serde_json::Value;

#[tokio::main]
async fn main() {
    let url = "https://v2.jokeapi.dev/joke/Programming";
    let response = reqwest::get(url).await.expect("Failed to fetch URL");
    let json = response.text().await.expect("error");
    let mut jokes: Value = serde_json::from_str(&json).expect("error");
    //println!("{}", jokes);

    if jokes["type"] == "twopart" {
        println!("S: {} \nD: {}", jokes["setup"], jokes["delivery"]);
    } else if jokes["type"] == "single" {
        if let Value::String(ref mut s) = &mut jokes["joke"] {
            *s = s
                .replace("\n", " ")
                .replace("\r", " ")
                .replace("\"", "")
                .replace("\\", " ")
                .replace("\n-", " ")
                .trim()
                .to_string();
        }
        println!("{}", jokes["joke"]);
    }
}
