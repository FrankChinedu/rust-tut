#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Post {
    title: String,
    author: String,
    content: String,
    published_at: Option<i32>,
}

impl AsRef<Post> for Post {
    fn as_ref(&self) -> &Post {
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Video;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VideoGuide {
    post: Post,
    videos: Vec<Video>,
}

impl AsRef<Post> for VideoGuide {
    fn as_ref(&self) -> &Post {
        &self.post
    }
}

// #[allow(dead_code)]
// fn notify(_: &Post) {}

#[allow(dead_code)]
fn notify<P>(post: P)
where
    P: AsRef<Post>,
{
    let post = post.as_ref();
    println!("Post {:?}", post);
}

#[allow(dead_code)]
fn fetch_post_with_id<T>(_: T) -> Post
where
    T: Into<String>,
{
    Post::default()
}

#[allow(dead_code)]
fn fetch_video_with_id<T>(_: T) -> VideoGuide
where
    T: Into<String>,
{
    VideoGuide::default()
}

#[allow(dead_code)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let post = fetch_post_with_id("123");
    notify(post);

    let video_guide = fetch_video_with_id("123");
    notify(video_guide);
    Ok(())
}
