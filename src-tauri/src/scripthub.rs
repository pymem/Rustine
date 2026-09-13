// this is for the scripthub fetching specific scripts for games, u dont really need to touch this unless u know a better way 
#![allow(dead_code)]

pub fn is_valid_game_id(game_id: u64) -> bool {
    game_id != 0 && game_id != u64::MAX
}

pub fn rscripts_url(query: &str, page: u32, game_id: Option<u64>) -> String {
    let page = page.max(1);
    match (query.trim().is_empty(), game_id.filter(|id| is_valid_game_id(*id))) {
        (true, Some(id)) => format!("https://rscripts.net/api/v2/scripts?page={page}&gameId={id}"),
        (true, None) => format!("https://rscripts.net/api/v2/scripts?page={page}"),
        (false, Some(id)) => format!(
            "https://rscripts.net/api/v2/scripts?q={}&page={page}&gameId={id}",
            url_encode(query)
        ),
        (false, None) => format!(
            "https://rscripts.net/api/v2/scripts?q={}&page={page}",
            url_encode(query)
        ),
    }
}

pub fn scriptblox_url(query: &str, page: u32) -> String {
    let page = page.max(1);
    if query.trim().is_empty() {
        "https://scriptblox.com/api/script/trending".to_string()
    } else {
        format!(
            "https://scriptblox.com/api/script/search?q={}&page={page}",
            url_encode(query)
        )
    }
}

fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_game_ids() {
        assert!(!is_valid_game_id(0));
        assert!(!is_valid_game_id(u64::MAX));
        assert!(is_valid_game_id(1818));
    }

    #[test]
    fn rscripts_url_scopes_game() {
        assert!(rscripts_url("", 1, Some(1818)).contains("gameId=1818"));
        assert!(!rscripts_url("", 1, Some(0)).contains("gameId="));
        assert!(!rscripts_url("", 1, None).contains("gameId="));
    }
}
