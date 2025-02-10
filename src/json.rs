use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
struct Node {
    v: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    nodes: Vec<Node>,
}

#[cfg(test)]
mod test {
    use crate::json::Node;

    #[test]
    fn test_node() {
        let s = include_str!("../assets/tree.json");
        let tree: Node = serde_json::from_str(s).unwrap();
        println!("{:#?}", tree);
        let json = serde_json::to_string(&tree).unwrap();
        println!("{}", json);
        let tree2: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(tree2, tree);
    }
}
