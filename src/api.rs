use gloo_net::http::Request;

pub async fn fetch_hanzi_pairs() -> Vec<String> {
    let resp = Request::get(
        "https://raw.githubusercontent.com/matturche/Shuangshuang/refs/heads/main/data/hanzi_pairs.txt"
    )
    .send()
    .await.expect("Failed send request for hanzi pairs");
    let text = resp.text().await.expect("Failed to get text from response");
    let lines: Vec<String> = text.lines().map(str::to_owned).collect();
    lines
}
