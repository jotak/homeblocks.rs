#[macro_export]
macro_rules! block_meta {
    ($struct:ident {$( $field:ident:$type:ty ),*}) =>{
        #[derive(Serialize, Deserialize)]
        pub struct $struct {
            kind: String,
            posx: i32,
            posy: i32,
            title: Option<String>,
            $(
                $field: $type,
            )*
        }
    };
}
block_meta!(Main {});
block_meta!(Note { note: String });
block_meta!(Links { links: Vec<Link> });

use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};

pub struct Page {
    pub main_blocks: Vec<Main>,
    pub note_blocks: Vec<Note>,
    pub links_blocks: Vec<Links>
}

impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Keep legacy model for json, which is all blocks inside a single array
        let mut state = serializer.serialize_struct("page", 1)?;
        let mut seq = serializer.serialize_seq(Some(self.main_blocks.len() + self.note_blocks.len() + self.links_blocks.len()))?;
        for e in self.main_blocks {
            seq.serialize_element(&e)?;
        }
        for e in self.note_blocks {
            seq.serialize_element(&e)?;
        }
        for e in self.links_blocks {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Page {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MyMapVisitor::new())
    }
}

pub fn main_block(posx: i32, posy: i32, title: Option<String>) -> Main {
    Main {
        kind: String::from("main"),
        posx: posx,
        posy: posy,
        title: title
    }
}

pub fn note_block(note: String, posx: i32, posy: i32, title: Option<String>) -> Note {
    Note {
        kind: String::from("note"),
        posx: posx,
        posy: posy,
        title: title,
        note: note
    }
}

pub fn links_block(links: Vec<Link>, posx: i32, posy: i32, title: Option<String>) -> Links {
    Links {
        kind: String::from("links"),
        posx: posx,
        posy: posy,
        title: title,
        links: links
    }
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    title: String,
    url: String,
    description: Option<String>
}
