use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
struct Node {
    #[serde(rename = "@v")]
    v: u32,
    #[serde(rename = "$value", skip_serializing_if = "Vec::is_empty", default)]
    nodes: Vec<Node>,
}

#[cfg(test)]
mod test {
    use crate::xml::Node;

    #[test]
    fn test_node() {
        let s = include_str!("../assets/tree.xml");
        let tree: Node = quick_xml::de::from_str(s).unwrap();
        println!("{:#?}", tree);
        let json = quick_xml::se::to_string(&tree).unwrap();
        println!("{}", json);
        let tree2: Node = quick_xml::de::from_str(&json).unwrap();
        assert_eq!(tree2, tree);
    }
}
