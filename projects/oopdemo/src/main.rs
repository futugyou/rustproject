#![allow(unused)]
use oopdemo::Button;
use oopdemo::Post;
use oopdemo::Screen;
use oopdemo::SelectBox;

fn main() {
    //screen_run();
    state_pattern();
}

fn state_pattern() {
    let mut post = Post::new();
    post.add_text("i say hi ,you say yes!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("i say hi ,you say yes!", post.content());
}

fn screen_run() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 32,
                height: 19,
                options: vec![String::from("")],
            }),
            Box::new(Button {
                width: 32,
                height: 19,
                lable: String::from(""),
            }),
        ],
    };
    screen.run();
}
