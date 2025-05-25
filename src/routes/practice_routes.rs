

use axum::{
    response::{IntoResponse, Html}, routing::get, Router
};



use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate<'a> {
    title: &'a str,
    user: User,
    posts: Vec<Post>,
}


#[derive(Clone)]
struct User {
    name: String,
    email: String,
    avatar_url: String,
}

#[derive(Clone)]
struct Post {
    id: u32,
    title: String,
    content: String,
    author: String,
    created_at: String,
}


pub fn practice_routes() -> Router {
    Router::new()
        .route("/", get(this_layout))
    // Add more routes here
}

async fn this_layout() -> impl IntoResponse{

  let users = get_user();
  let posts = get_posts();
  let res = LayoutTemplate { 
    title: "helslo", 
    user: users, 
    posts: posts,
    };
  match res.render() {
    Ok(html) => Html(html),
    Err(_) => Html("<h1>error occured</h1>".to_string())
  }
}

async fn hello() -> impl IntoResponse {
    let hello = HelloTemplate { name: "world" };
    match hello.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<h1>Error rendering template</h1>".to_string()),
    }
}

fn get_user() -> User {
  // make call to the repository here:
  // for examples sake just return predefined user
  let user = User {
        name: "Johnny NY".to_string(),
        email: "john@test.com".to_string(),
        avatar_url: "/static/avatars/john.jpg".to_string(),
    };

    return user;
}

fn get_posts() -> Vec<Post> {

  let posts = vec![
        Post {
            id: 1,
            title: "Birthday at the beach!!!11!".to_string(),
            content: "Spent the day at the beach BBQing with my friends and family".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-03-12".to_string(),
        },
        Post {
            id: 2,
            title: "Another Day...".to_string(),
            content: "30 hours of week just debugging... DEBUGGING... ".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-04-08".to_string(),
        },
        Post {
            id: 3,
            title: "Templates are for girls".to_string(),
            content: "Hear me out, don't you want to hydraaaaate?  Hydraating is so good for your skin.  We should all just us a Frontend Framework like Mithril.js bro".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-04-15".to_string(),
        },
        Post {
            id: 4,
            title: "Mandatory drug test at work today...".to_string(),
            content: "OMG OmG OMG OMMMG OmG og fr fr.".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-04-22".to_string(),
        },
        Post {
            id: 5,
            title: "Porch Pirates will be the de*th of me".to_string(),
            content: "Package stolen again off my porch.  God I love Hartford people".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-05-01".to_string(),
        },
        Post {
            id: 6,
            title: "Increase Chars".to_string(),
            content: "140 characters?  what is this?  an app for ants? it needs to be... at least 3 times as big".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-05-10".to_string(),
        },
        Post {
            id: 7,
            title: "Big News on the HOrizon!".to_string(),
            content: "3 years in Game Dev and about to have a demo ready! everyone stay tuned!".to_string(),
            author: "Johnny NY".to_string(),
            created_at: "2024-05-18".to_string(),
        },
    ];

    return posts;

}
