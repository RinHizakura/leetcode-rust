use std::collections::{HashMap, HashSet};

struct Post {
    user_id: i32,
    tweet_id: i32,
}

impl Post {
    fn new(user_id: i32, tweet_id: i32) -> Self {
        Post { user_id, tweet_id }
    }
}
struct User {
    follow: HashSet<i32>,
}

impl User {
    fn new() -> Self {
        User {
            follow: HashSet::new(),
        }
    }
}

struct Twitter {
    post: Vec<Post>,
    user: Vec<User>,
    map: HashMap<i32, usize>,
    slot_idx: usize,
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            post: Vec::new(),
            user: Vec::new(),
            map: HashMap::new(),
            slot_idx: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.post.push(Post::new(user_id, tweet_id))
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut cnt = 10;
        let mut news = Vec::new();

        let len = self.post.len();
        for idx in (0..len).rev() {
            let post = &self.post[idx];

            let mut pick = false;
            if post.user_id == user_id {
                pick = true;
            } else if let Some(slot_idx) = self.map.get(&user_id) {
                if self.user[*slot_idx].follow.contains(&post.user_id) {
                    pick = true;
                }
            }

            if pick == true {
                news.push(post.tweet_id);
                cnt -= 1;
            }

            if cnt == 0 {
                break;
            }
        }

        news
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let slot_idx;
        if let Some(i) = self.map.get(&follower_id) {
            slot_idx = *i;
        } else {
            /* Assign a new slot_id for this follwer */
            slot_idx = self.slot_idx;
            self.map.insert(follower_id, slot_idx);
            self.user.push(User::new());
            self.slot_idx += 1;
        }

        /* Here we assume that one can't follow somebody twice without
         * unfollowing first. */
        self.user[slot_idx].follow.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(slot_idx) = self.map.get(&follower_id) {
            self.user[*slot_idx].follow.remove(&followee_id);
        }
    }
}

fn main() {
    let mut obj = Twitter::new();
    obj.post_tweet(1, 1);
    let ret: Vec<i32> = obj.get_news_feed(1);
    obj.follow(1, 2);
    obj.unfollow(1, 2);

    println!("{:?}", ret);
}
