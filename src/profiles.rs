mod blocks;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    title: String,
    page: blocks::Page
}

pub fn login_profile() -> Profile {
    let page = blocks::Page {
        main_blocks: vec![],
        note_blocks: vec![
            blocks::note_block(String::from("<h3>Welcome to Homeblocks.net</h3><br/>Build your homepage, block after block!"), -1, -1, Option::None)
        ],
        links_blocks: vec![
            blocks::links_block(vec![], 0, 0, Option::from(String::from("Login")))
        ]
    };
    Profile {
        title: String::from("login"),
        page: page
    }
    // value page = Page([
    //     LinksBlock(
    //         authProviders.map(([String, String] p) => Link(p[0], p[1], p[0])).sequence(), 0, 0, "Login"),
    //     NoteBlock("<h3>Welcome to Homeblocks.net</h3><br/>Build your homepage, block after block!", -1, -1, "")
    // ]);

    // return Object {
    //         "title" -> "login",
    //         "page" -> page.json()
    // };
}
