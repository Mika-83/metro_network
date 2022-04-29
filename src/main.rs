use basic::model::{read_node, read, read_edge};

fn main() {
    read_node(read("data/eki.csv"));
    read_edge(read("data/ekikan.csv"));
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2+1, 3)
    }
}
