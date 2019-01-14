#[macro_export]
macro_rules! blocks {
    ($( $variant:ident {$( $field:ident:$type:ty ),*}) ,*) =>{
        #[derive(Serialize, Deserialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        pub enum Block {
            $(
                $variant {
                    posx: i32,
                    posy: i32,
                    title: Option<String>,
                    $(
                        $field: $type,
                    )*
                },
            )*
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub blocks: Vec<Block>
}

blocks!(
    Main {},
    Note { note: String },
    Links { links: Vec<Link> },
    Image { links: Vec<Link> },
    Audio { links: Vec<Link> },
    Video { links: Vec<Link> },
    List { items: Vec<String> }
);

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub url: String,
    pub description: Option<String>
}
