use crate::model::{Node, Edge};

#[warn(dead_code)]
fn romaji_to_kanji(lst: &Vec<Node>, name: &String) -> String {
    if lst.is_empty() {
        "".to_string()
    } else if &lst[0].roman == name {
        lst[0].kanji.clone()
    } else {
        romaji_to_kanji(&lst[1..].to_vec(), name)
    }
}

fn get_ekikan_kyori(lst: &Vec<Edge>, tail: &String, head: &String) -> f32 {
    if lst.is_empty() {
        f32::INFINITY
    } else if (tail, head) == (&lst[0].tail, &lst[0].head) || (tail, head) == (&lst[0].head, &lst[0].tail) {
        lst[0].dist
    } else {
        get_ekikan_kyori(&lst[1..].to_vec(), tail, head)
    }

}

fn kyori_no_hyouji(lst_node: &Vec<Node>, lst_edge: &Vec<Edge>, tail: &String, head: &String) -> String {
    let tail_kanji = romaji_to_kanji(lst_node, tail);
    let head_kanji = romaji_to_kanji(lst_node, head);
    if tail_kanji == "".to_string() && head_kanji == "".to_string() {
        format!("{}と{}という駅は存在しません", tail, head)
    } else if tail_kanji == "".to_string() {
       format!("{}という駅は存在しません", tail)
    } else if head_kanji == "".to_string() {
        format!("{}という駅は存在しません", head)
    } else {
        let dist = get_ekikan_kyori(lst_edge, &tail_kanji, &head_kanji);
        if !f32::is_finite(dist) {
            format!("{}と{}はつながっていません", tail_kanji, head_kanji)
        } else {
            format!("{}から{}までは{:.1}kmです", tail_kanji, head_kanji, dist)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    #[test]
    fn test_romaji_to_kanji_1() {
        let lst = read_node(read("data/eki.csv"));
        assert_eq!(romaji_to_kanji(&lst, &String::from("osaka")), "".to_string())
    }
    #[test]
    fn test_romaji_to_kanji_2() {
        let lst = read_node(read("data/eki.csv"));
        assert_eq!(romaji_to_kanji(&lst, &String::from("hongosanchome")), "本郷三丁目".to_string())
    }
    #[test]
    fn test_get_ekikan_kyori_1(){
        let lst = read_edge(read("data/ekikan.csv"));
        let tail = String::from("大阪");
        let head = String::from("代々木公園");
        assert_eq!(get_ekikan_kyori(&lst, &tail, &head), f32::INFINITY)
    }
    #[test]
    fn test_get_ekikan_kyori_2(){
        let lst = read_edge(read("data/ekikan.csv"));
        let tail = String::from("代々木公園");
        let head = String::from("代々木上原");
        assert_eq!(get_ekikan_kyori(&lst, &tail ,&head), 1.0)
    }
    #[test]
    fn test_kyori_no_hyouji_1() {
        let lst_node = read_node(read("data/eki.csv"));
        let lst_edge = read_edge(read("data/ekikan.csv"));
        let tail = String::from("osaka");
        let head = String::from("yoyogikouen");
        assert_eq!(kyori_no_hyouji(&lst_node, &lst_edge, &tail, &head), format!("{}という駅は存在しません", tail))
    }
    #[test]
    fn test_kyori_no_hyouji_2() {
        let lst_node = read_node(read("data/eki.csv"));
        let lst_edge = read_edge(read("data/ekikan.csv"));
        let tail = String::from("yoyogiuehara");
        let head = String::from("yoyogikouen");
        assert_eq!(kyori_no_hyouji(&lst_node, &lst_edge, &tail, &head), "代々木上原から代々木公園までは1.0kmです".to_string())
    }
    #[test]
    fn test_kyori_no_hyouji_3() {
        let lst_node = read_node(read("data/eki.csv"));
        let lst_edge = read_edge(read("data/ekikan.csv"));
        let tail = String::from("osaka");
        let head = String::from("kyoto");
        assert_eq!(kyori_no_hyouji(&lst_node, &lst_edge, &tail, &head), "osakaとkyotoという駅は存在しません".to_string())
    }
    #[test]
    fn test_kyori_no_hyouji_4() {
        let lst_node = read_node(read("data/eki.csv"));
        let lst_edge = read_edge(read("data/ekikan.csv"));
        let tail = String::from("yoyogiuehara");
        let head = String::from("meijijinguumae");
        assert_eq!(kyori_no_hyouji(&lst_node, &lst_edge, &tail, &head), "代々木上原と明治神宮前はつながっていません".to_string())
    }
}