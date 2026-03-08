use jsony_macros::Jsony;
#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub enum PostMedium {
    Text {
        body: String,
    },
    Image {
        url: String,
        width: u32,
        height: u32,
    },
    Video {
        url: String,
        thumbnail_url: String,
        duration_secs: u32,
    },
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub enum NotificationKind {
    Like,
    Comment,
    Follow,
    Mention,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct Timestamp {
    pub ms: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct UserId(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct PostId(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct CommentId(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct GroupChatId(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct MessageId(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Jsony)]
#[jsony(Binary, Json)]
pub struct NotificationId(pub u64);
#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct User {
    pub id: UserId,
    #[jsony(rename = "userName")]
    pub username: String,
    #[jsony(rename = "displayName")]
    pub display_name: String,
    pub bio: Option<String>,
    #[jsony(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[jsony(rename = "createdAt")]
    pub created_at: Timestamp,
    #[jsony(rename = "followerCount")]
    pub follower_count: u32,
    #[jsony(rename = "followingCount")]
    pub following_count: u32,
    pub verified: bool,
    pub email: Option<String>,
}

#[derive(Clone, Jsony)]
#[jsony(ToJson)]
pub struct UserSummary {
    pub id: UserId,
    #[jsony(rename = "userName")]
    pub username: String,
    #[jsony(rename = "displayName")]
    pub display_name: String,
    #[jsony(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub verified: bool,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct FollowRelation {
    #[jsony(rename = "followerId")]
    pub follower_id: UserId,
    #[jsony(rename = "followingId")]
    pub following_id: UserId,
    #[jsony(rename = "createdAt")]
    pub created_at: Timestamp,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct ChatMessage {
    pub id: MessageId,
    #[jsony(rename = "senderId")]
    pub sender_id: UserId,
    #[jsony(rename = "groupId")]
    pub group_id: GroupChatId,
    pub content: String,
    pub timestamp: Timestamp,
    pub edited: bool,
    #[jsony(rename = "replyTo")]
    pub reply_to: Option<MessageId>,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct GroupChat {
    pub id: GroupChatId,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<UserId>,
    #[jsony(rename = "createdAt")]
    pub created_at: Timestamp,
    #[jsony(rename = "adminId")]
    pub admin_id: UserId,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct Post {
    pub id: PostId,
    #[jsony(rename = "authorId")]
    pub author_id: UserId,
    pub medium: PostMedium,
    pub caption: String,
    #[jsony(rename = "likeCount")]
    pub like_count: u32,
    #[jsony(rename = "createdAt")]
    pub created_at: Timestamp,
    pub tags: Vec<String>,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct PostComment {
    pub id: CommentId,
    #[jsony(rename = "postId")]
    pub post_id: PostId,
    #[jsony(rename = "authorId")]
    pub author_id: UserId,
    pub text: String,
    pub timestamp: Timestamp,
    pub likes: u32,
    #[jsony(rename = "replyTo")]
    pub reply_to: Option<CommentId>,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct Notification {
    pub id: NotificationId,
    #[jsony(rename = "userId")]
    pub user_id: UserId,
    pub kind: NotificationKind,
    #[jsony(rename = "sourceId")]
    pub source_id: UserId,
    #[jsony(rename = "referenceId")]
    pub reference_id: PostId,
    pub message: String,
    pub read: bool,
    pub timestamp: Timestamp,
}

#[derive(Clone, Jsony)]
#[jsony(Binary, Json)]
pub struct Database {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
    #[jsony(rename = "postComments")]
    pub comments: Vec<PostComment>,
    #[jsony(rename = "groupChats")]
    pub group_chats: Vec<GroupChat>,
    pub messages: Vec<ChatMessage>,
    pub notifications: Vec<Notification>,
    pub followers: Vec<FollowRelation>,
}

#[derive(Clone, Jsony)]
#[jsony(FromJson)]
pub struct CreatePostRequest {
    #[jsony(rename = "authorId")]
    pub author_id: UserId,
    pub medium: PostMedium,
    pub caption: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Jsony)]
#[jsony(FromJson)]
pub struct SendMessageRequest {
    #[jsony(rename = "senderId")]
    pub sender_id: UserId,
    #[jsony(rename = "groupId")]
    pub group_id: GroupChatId,
    pub content: String,
    #[jsony(rename = "replyTo")]
    pub reply_to: Option<MessageId>,
}

#[derive(Clone, Jsony)]
#[jsony(FromJson)]
pub struct AddCommentRequest {
    #[jsony(rename = "postId")]
    pub post_id: PostId,
    #[jsony(rename = "authorId")]
    pub author_id: UserId,
    pub text: String,
    #[jsony(rename = "replyTo")]
    pub reply_to: Option<CommentId>,
}

#[derive(Clone, Jsony)]
#[jsony(FromJson)]
pub struct GetFeedRequest {
    #[jsony(rename = "userId")]
    pub user_id: UserId,
    pub limit: u32,
}

#[derive(Clone, Jsony)]
#[jsony(ToJson)]
pub struct FeedResponse {
    pub posts: Vec<Post>,
}

#[derive(Clone, Jsony)]
#[jsony(ToJson)]
pub struct ProfileResponse {
    pub user: User,
    pub posts: Vec<Post>,
    pub followers: Vec<UserSummary>,
}

#[derive(Clone, Jsony)]
#[jsony(ToJson)]
pub struct UserData {
    pub user: User,
    pub posts: Vec<Post>,
    pub followers: Vec<UserSummary>,
}
