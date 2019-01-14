mod blocks;
mod oauth;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    title: String,
    page: blocks::Page
}

pub fn login_profile() -> Profile {
    oauth::PROVIDERS.with(|providers| {
        let page = blocks::Page {
            blocks: vec![
                blocks::Block::Note {
                    posx: -1,
                    posy: -1,
                    title: Option::None,
                    note: String::from("<h3>Welcome to Homeblocks.net</h3><br/>Build your homepage, block after block!")
                },
                blocks::Block::Links {
                    posx: 0,
                    posy: 0,
                    title: Option::from(String::from("Login")),
                    links: providers.iter().map(|p| {
                        blocks::Link {
                            url: p.auth_url(),
                            title: String::from("Login with ") + p.name,
                            description: Option::None
                        }
                    }).collect::<Vec<_>>()
                },
                blocks::Block::Image {
                    posx: 0,
                    posy: -1,
                    title: Option::None,
                    links: vec![blocks::Link {
                        url: String::from("http://aws-cf.imdoc.fr/prod/photos/6/9/8/4936698/16029275/img-16029275b1f.jpg?v=11"),
                        title: String::from("A'Tuin"),
                        description: Option::None
                    }]
                }
            ]
        };
        Profile {
            title: String::from("login"),
            page: page
        }
    })
}
