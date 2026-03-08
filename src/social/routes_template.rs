use crate::models::*;
use crate::compat;
use crate::db;

pub fn dispatch(db_path: &str, route: &str, input: &str) -> String {
    match route {
        "seed" => seed(db_path),
        "get_feed" => get_feed(db_path, input),
        "create_post" => create_post(db_path, input),
        "send_message" => send_message(db_path, input),
        "add_comment" => add_comment(db_path, input),
        "get_profile" => get_profile(db_path, input),
        "list_chats" => list_chats(db_path, input),
        _ => panic!("Unknown route: {}", route),
    }
}

fn seed(db_path: &str) -> String {
    let mut users = Vec::new();
    for i in 0..10u64 {
        users.push(User {
            id: UserId(i + 1),
            username: format!("user_{}", i + 1),
            display_name: format!("User {}", i + 1),
            bio: if i % 3 == 0 { Some(format!("Bio for user {}", i + 1)) } else { None },
            avatar_url: if i % 2 == 0 { Some(format!("https://example.com/avatar/{}.jpg", i + 1)) } else { None },
            created_at: Timestamp { ms: 1700000000000 + (i as i64) * 86400000 },
            follower_count: (i as u32) * 100 + 50,
            following_count: (i as u32) * 30 + 10,
            verified: i < 3,
            email: Some(format!("user{}@example.com", i + 1)),
        });
    }

    let mut posts = Vec::new();
    for i in 0..20u64 {
        let medium = match i % 3 {
            0 => PostMedium::Text { body: format!("Post body number {}", i + 1) },
            1 => PostMedium::Image {
                url: format!("https://example.com/img/{}.jpg", i + 1),
                width: 1920,
                height: 1080,
            },
            _ => PostMedium::Video {
                url: format!("https://example.com/vid/{}.mp4", i + 1),
                thumbnail_url: format!("https://example.com/thumb/{}.jpg", i + 1),
                duration_secs: 120 + (i as u32) * 10,
            },
        };
        posts.push(Post {
            id: PostId(i + 1),
            author_id: UserId((i % 10) + 1),
            medium,
            caption: format!("Caption for post {}", i + 1),
            like_count: (i as u32) * 15,
            created_at: Timestamp { ms: 1700100000000 + (i as i64) * 3600000 },
            tags: vec![format!("tag{}", i % 5), format!("topic{}", i % 3)],
        });
    }

    let mut comments = Vec::new();
    for i in 0..30u64 {
        comments.push(PostComment {
            id: CommentId(i + 1),
            post_id: PostId((i % 20) + 1),
            author_id: UserId((i % 10) + 1),
            text: format!("Comment text {}", i + 1),
            timestamp: Timestamp { ms: 1700200000000 + (i as i64) * 1800000 },
            likes: (i as u32) * 3,
            reply_to: if i % 4 == 0 && i > 0 { Some(CommentId(i)) } else { None },
        });
    }

    let mut group_chats = Vec::new();
    for i in 0..5u64 {
        let members: Vec<UserId> = (1..=(i + 2).min(10)).map(|x| UserId(x)).collect();
        group_chats.push(GroupChat {
            id: GroupChatId(i + 1),
            name: format!("Group {}", i + 1),
            description: if i % 2 == 0 { Some(format!("Description for group {}", i + 1)) } else { None },
            members,
            created_at: Timestamp { ms: 1700000000000 + (i as i64) * 172800000 },
            admin_id: UserId(1),
        });
    }

    let mut messages = Vec::new();
    for i in 0..50u64 {
        messages.push(ChatMessage {
            id: MessageId(i + 1),
            sender_id: UserId((i % 10) + 1),
            group_id: GroupChatId((i % 5) + 1),
            content: format!("Message content {}", i + 1),
            timestamp: Timestamp { ms: 1700300000000 + (i as i64) * 60000 },
            edited: i % 7 == 0,
            reply_to: if i % 5 == 0 && i > 0 { Some(MessageId(i)) } else { None },
        });
    }

    let mut notifications = Vec::new();
    for i in 0..20u64 {
        notifications.push(Notification {
            id: NotificationId(i + 1),
            user_id: UserId((i % 10) + 1),
            kind: match i % 4 {
                0 => NotificationKind::Like,
                1 => NotificationKind::Comment,
                2 => NotificationKind::Follow,
                _ => NotificationKind::Mention,
            },
            source_id: UserId((i % 10) + 1),
            reference_id: PostId((i % 20) + 1),
            message: format!("Notification message {}", i + 1),
            read: i % 2 == 0,
            timestamp: Timestamp { ms: 1700400000000 + (i as i64) * 300000 },
        });
    }

    let mut followers = Vec::new();
    for i in 0..15u64 {
        followers.push(FollowRelation {
            follower_id: UserId((i % 10) + 1),
            following_id: UserId(((i + 3) % 10) + 1),
            created_at: Timestamp { ms: 1700050000000 + (i as i64) * 43200000 },
        });
    }

    let database = Database {
        users,
        posts,
        comments,
        group_chats,
        messages,
        notifications,
        followers,
    };
    db::save_db(db_path, &database);
    String::from("{\"ok\":true}")
}

fn get_feed(db_path: &str, input: &str) -> String {
    let req: GetFeedRequest = compat::from_json(input);
    let database = db::load_db(db_path);
    let limit = req.limit as usize;
    let feed_posts: Vec<Post> = database.posts.into_iter().take(limit).collect();
    let response = FeedResponse { posts: feed_posts };
    compat::to_json(&response)
}

fn create_post(db_path: &str, input: &str) -> String {
    let req: CreatePostRequest = compat::from_json(input);
    let mut database = db::load_db(db_path);
    let next_id = PostId(database.posts.iter().map(|p| p.id.0).max().unwrap_or(0) + 1);
    let post = Post {
        id: next_id,
        author_id: req.author_id,
        medium: req.medium,
        caption: req.caption,
        like_count: 0,
        created_at: Timestamp { ms: 1700500000000 },
        tags: req.tags,
    };
    database.posts.push(post);
    db::save_db(db_path, &database);
    compat::to_json(&database)
}

fn send_message(db_path: &str, input: &str) -> String {
    let req: SendMessageRequest = compat::from_json(input);
    let mut database = db::load_db(db_path);
    let next_id = MessageId(database.messages.iter().map(|m| m.id.0).max().unwrap_or(0) + 1);
    let msg = ChatMessage {
        id: next_id,
        sender_id: req.sender_id,
        group_id: req.group_id,
        content: req.content,
        timestamp: Timestamp { ms: 1700500000000 },
        edited: false,
        reply_to: req.reply_to,
    };
    database.messages.push(msg);
    db::save_db(db_path, &database);
    compat::to_json(&database)
}

fn add_comment(db_path: &str, input: &str) -> String {
    let req: AddCommentRequest = compat::from_json(input);
    let mut database = db::load_db(db_path);
    let next_id = CommentId(database.comments.iter().map(|c| c.id.0).max().unwrap_or(0) + 1);
    let comment = PostComment {
        id: next_id,
        post_id: req.post_id,
        author_id: req.author_id,
        text: req.text,
        timestamp: Timestamp { ms: 1700500000000 },
        likes: 0,
        reply_to: req.reply_to,
    };
    database.comments.push(comment);
    db::save_db(db_path, &database);
    compat::to_json(&database)
}

fn get_profile(db_path: &str, input: &str) -> String {
    let req: GetFeedRequest = compat::from_json(input);
    let database = db::load_db(db_path);
    let user = database.users.iter().find(|u| u.id == req.user_id).unwrap().clone();
    let user_posts: Vec<Post> = database.posts.into_iter().filter(|p| p.author_id == user.id).collect();
    let followers: Vec<UserSummary> = database.followers.iter()
        .filter(|f| f.following_id == user.id)
        .filter_map(|f| database.users.iter().find(|u| u.id == f.follower_id))
        .map(|u| UserSummary {
            id: u.id,
            username: u.username.clone(),
            display_name: u.display_name.clone(),
            avatar_url: u.avatar_url.clone(),
            verified: u.verified,
        })
        .collect();
    let response = ProfileResponse { user, posts: user_posts, followers };
    compat::to_json(&response)
}

fn list_chats(db_path: &str, input: &str) -> String {
    let req: GetFeedRequest = compat::from_json(input);
    let database = db::load_db(db_path);
    let user_chats: Vec<GroupChat> = database.group_chats.into_iter()
        .filter(|gc| gc.members.contains(&req.user_id))
        .collect();
    compat::to_json(&user_chats)
}
