#[derive(Serialize, Deserialize)]
pub struct Page {
    pub main_blocks: Vec<Main>,
    pub note_blocks: Vec<Note>,
    pub links_blocks: Vec<Links>
}

#[derive(Serialize, Deserialize)]
pub struct Main { meta: BlockMeta }

#[derive(Serialize, Deserialize)]
pub struct Note { meta: BlockMeta, note: String }

#[derive(Serialize, Deserialize)]
pub struct Links { meta: BlockMeta, links: Vec<Link> }

#[derive(Serialize, Deserialize)]
pub struct BlockMeta {
    posx: i32,
    posy: i32,
    title: Option<String>
}

pub fn main_block(posx: i32, posy: i32, title: Option<String>) -> Main {
    Main {
        meta: BlockMeta {
            posx: posx,
            posy: posy,
            title: title
        }
    }
}

pub fn note_block(note: String, posx: i32, posy: i32, title: Option<String>) -> Note {
    Note {
        meta: BlockMeta {
            posx: posx,
            posy: posy,
            title: title
        },
        note: note
    }
}

pub fn links_block(links: Vec<Link>, posx: i32, posy: i32, title: Option<String>) -> Links {
    Links {
        meta: BlockMeta {
            posx: posx,
            posy: posy,
            title: title
        },
        links: links
    }
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    title: String,
    url: String,
    description: Option<String>
}
