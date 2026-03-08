use std::cell::Cell;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{bail, Context};
use proc_macro2::{Ident, TokenTree};

use crate::bench::{Benchy, BuildProfile, Perf, Scenario};
use crate::library::Libary;
use crate::schema::{
    Enum, EnumVariant, Field, ItemKind, Struct, Type, Types, IMPL_BINARY, IMPL_FROM_JSON,
    IMPL_JSON, IMPL_TO_JSON,
};
use crate::token;

pub struct SocialMediaTask<'a> {
    structs: Vec<&'a Struct<'a>>,
    enums: Vec<&'a Enum<'a>>,
}

fn field<'a>(name: &str, ty: Type<'a>, rename: Option<&'a str>) -> Field<'a> {
    Field {
        name: Ident::owned(name),
        inner_type: Cell::new(ty),
        rename,
    }
}

fn rn<'a>(bump: &'a bumpalo::Bump, s: &str) -> Option<&'a str> {
    Some(&*bump.alloc_str(s))
}

impl<'a> SocialMediaTask<'a> {
    pub fn new(types: &'a mut Types<'a>) -> SocialMediaTask<'a> {
        let mut structs = Vec::new();
        let mut enums = Vec::new();

        let both = IMPL_JSON | IMPL_BINARY;

        let timestamp = types.create_struct(
            ItemKind::Struct,
            Ident::owned("Timestamp"),
            vec![field("ms", Type::I64, None)],
            0,
            both,
        );
        let timestamp_ty = Type::Struct(timestamp);
        structs.push(timestamp);

        let user_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("UserId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let user_id_ty = Type::Struct(user_id);
        structs.push(user_id);

        let post_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("PostId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let post_id_ty = Type::Struct(post_id);
        structs.push(post_id);

        let comment_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("CommentId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let comment_id_ty = Type::Struct(comment_id);
        structs.push(comment_id);

        let group_chat_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("GroupChatId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let group_chat_id_ty = Type::Struct(group_chat_id);
        structs.push(group_chat_id);

        let message_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("MessageId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let message_id_ty = Type::Struct(message_id);
        structs.push(message_id);

        let notification_id = types.create_struct(
            ItemKind::Tuple,
            Ident::owned("NotificationId"),
            vec![field("0", Type::U64, None)],
            0,
            both,
        );
        let notification_id_ty = Type::Struct(notification_id);
        structs.push(notification_id);

        let text_fields = types.alloc_fields(vec![field("body", Type::String, None)]);
        let image_fields = types.alloc_fields(vec![
            field("url", Type::String, None),
            field("width", Type::U32, None),
            field("height", Type::U32, None),
        ]);
        let video_fields = types.alloc_fields(vec![
            field("url", Type::String, None),
            field("thumbnail_url", Type::String, None),
            field("duration_secs", Type::U32, None),
        ]);
        let post_medium = types.create_enum(
            Ident::owned("PostMedium"),
            vec![
                EnumVariant {
                    name: Ident::owned("Text"),
                    fields: text_fields,
                },
                EnumVariant {
                    name: Ident::owned("Image"),
                    fields: image_fields,
                },
                EnumVariant {
                    name: Ident::owned("Video"),
                    fields: video_fields,
                },
            ],
            both,
        );
        let post_medium_ty = Type::Enum(post_medium);
        enums.push(post_medium);

        let notification_kind = types.create_enum(
            Ident::owned("NotificationKind"),
            vec![
                EnumVariant {
                    name: Ident::owned("Like"),
                    fields: types.alloc_fields(vec![]),
                },
                EnumVariant {
                    name: Ident::owned("Comment"),
                    fields: types.alloc_fields(vec![]),
                },
                EnumVariant {
                    name: Ident::owned("Follow"),
                    fields: types.alloc_fields(vec![]),
                },
                EnumVariant {
                    name: Ident::owned("Mention"),
                    fields: types.alloc_fields(vec![]),
                },
            ],
            both,
        );
        let notification_kind_ty = Type::Enum(notification_kind);
        enums.push(notification_kind);

        let user = types.create_struct(
            ItemKind::Struct,
            Ident::owned("User"),
            vec![
                field("id", user_id_ty, None),
                field("username", Type::String, rn(types.bump, "userName")),
                field("display_name", Type::String, rn(types.bump, "displayName")),
                field("bio", Type::Option(&Type::String), None),
                field(
                    "avatar_url",
                    Type::Option(&Type::String),
                    rn(types.bump, "avatarUrl"),
                ),
                field("created_at", timestamp_ty, rn(types.bump, "createdAt")),
                field("follower_count", Type::U32, rn(types.bump, "followerCount")),
                field(
                    "following_count",
                    Type::U32,
                    rn(types.bump, "followingCount"),
                ),
                field("verified", Type::Bool, None),
                field("email", Type::Option(&Type::String), None),
            ],
            0,
            both,
        );
        structs.push(user);

        let user_summary = types.create_struct(
            ItemKind::Struct,
            Ident::owned("UserSummary"),
            vec![
                field("id", user_id_ty, None),
                field("username", Type::String, rn(types.bump, "userName")),
                field("display_name", Type::String, rn(types.bump, "displayName")),
                field(
                    "avatar_url",
                    Type::Option(&Type::String),
                    rn(types.bump, "avatarUrl"),
                ),
                field("verified", Type::Bool, None),
            ],
            0,
            IMPL_TO_JSON,
        );
        structs.push(user_summary);

        let follow_relation = types.create_struct(
            ItemKind::Struct,
            Ident::owned("FollowRelation"),
            vec![
                field("follower_id", user_id_ty, rn(types.bump, "followerId")),
                field("following_id", user_id_ty, rn(types.bump, "followingId")),
                field("created_at", timestamp_ty, rn(types.bump, "createdAt")),
            ],
            0,
            both,
        );
        structs.push(follow_relation);

        let chat_message = types.create_struct(
            ItemKind::Struct,
            Ident::owned("ChatMessage"),
            vec![
                field("id", message_id_ty, None),
                field("sender_id", user_id_ty, rn(types.bump, "senderId")),
                field("group_id", group_chat_id_ty, rn(types.bump, "groupId")),
                field("content", Type::String, None),
                field("timestamp", timestamp_ty, None),
                field("edited", Type::Bool, None),
                field(
                    "reply_to",
                    Type::Option(types.bump.alloc(message_id_ty)),
                    rn(types.bump, "replyTo"),
                ),
            ],
            0,
            both,
        );
        structs.push(chat_message);

        let group_chat = types.create_struct(
            ItemKind::Struct,
            Ident::owned("GroupChat"),
            vec![
                field("id", group_chat_id_ty, None),
                field("name", Type::String, None),
                field("description", Type::Option(&Type::String), None),
                field("members", Type::Vec(types.bump.alloc(user_id_ty)), None),
                field("created_at", timestamp_ty, rn(types.bump, "createdAt")),
                field("admin_id", user_id_ty, rn(types.bump, "adminId")),
            ],
            0,
            both,
        );
        structs.push(group_chat);

        let post = types.create_struct(
            ItemKind::Struct,
            Ident::owned("Post"),
            vec![
                field("id", post_id_ty, None),
                field("author_id", user_id_ty, rn(types.bump, "authorId")),
                field("medium", post_medium_ty, None),
                field("caption", Type::String, None),
                field("like_count", Type::U32, rn(types.bump, "likeCount")),
                field("created_at", timestamp_ty, rn(types.bump, "createdAt")),
                field("tags", Type::Vec(&Type::String), None),
            ],
            0,
            both,
        );
        structs.push(post);

        let post_comment = types.create_struct(
            ItemKind::Struct,
            Ident::owned("PostComment"),
            vec![
                field("id", comment_id_ty, None),
                field("post_id", post_id_ty, rn(types.bump, "postId")),
                field("author_id", user_id_ty, rn(types.bump, "authorId")),
                field("text", Type::String, None),
                field("timestamp", timestamp_ty, None),
                field("likes", Type::U32, None),
                field(
                    "reply_to",
                    Type::Option(types.bump.alloc(comment_id_ty)),
                    rn(types.bump, "replyTo"),
                ),
            ],
            0,
            both,
        );
        structs.push(post_comment);

        let notification = types.create_struct(
            ItemKind::Struct,
            Ident::owned("Notification"),
            vec![
                field("id", notification_id_ty, None),
                field("user_id", user_id_ty, rn(types.bump, "userId")),
                field("kind", notification_kind_ty, None),
                field("source_id", user_id_ty, rn(types.bump, "sourceId")),
                field("reference_id", post_id_ty, rn(types.bump, "referenceId")),
                field("message", Type::String, None),
                field("read", Type::Bool, None),
                field("timestamp", timestamp_ty, None),
            ],
            0,
            both,
        );
        structs.push(notification);

        let database = types.create_struct(
            ItemKind::Struct,
            Ident::owned("Database"),
            vec![
                field(
                    "users",
                    Type::Vec(types.bump.alloc(Type::Struct(user))),
                    None,
                ),
                field(
                    "posts",
                    Type::Vec(types.bump.alloc(Type::Struct(post))),
                    None,
                ),
                field(
                    "comments",
                    Type::Vec(types.bump.alloc(Type::Struct(post_comment))),
                    rn(types.bump, "postComments"),
                ),
                field(
                    "group_chats",
                    Type::Vec(types.bump.alloc(Type::Struct(group_chat))),
                    rn(types.bump, "groupChats"),
                ),
                field(
                    "messages",
                    Type::Vec(types.bump.alloc(Type::Struct(chat_message))),
                    None,
                ),
                field(
                    "notifications",
                    Type::Vec(types.bump.alloc(Type::Struct(notification))),
                    None,
                ),
                field(
                    "followers",
                    Type::Vec(types.bump.alloc(Type::Struct(follow_relation))),
                    None,
                ),
            ],
            0,
            both,
        );
        structs.push(database);

        let create_post_request = types.create_struct(
            ItemKind::Struct,
            Ident::owned("CreatePostRequest"),
            vec![
                field("author_id", user_id_ty, rn(types.bump, "authorId")),
                field("medium", post_medium_ty, None),
                field("caption", Type::String, None),
                field("tags", Type::Vec(&Type::String), None),
            ],
            0,
            IMPL_FROM_JSON,
        );
        structs.push(create_post_request);

        let send_message_request = types.create_struct(
            ItemKind::Struct,
            Ident::owned("SendMessageRequest"),
            vec![
                field("sender_id", user_id_ty, rn(types.bump, "senderId")),
                field("group_id", group_chat_id_ty, rn(types.bump, "groupId")),
                field("content", Type::String, None),
                field(
                    "reply_to",
                    Type::Option(types.bump.alloc(message_id_ty)),
                    rn(types.bump, "replyTo"),
                ),
            ],
            0,
            IMPL_FROM_JSON,
        );
        structs.push(send_message_request);

        let add_comment_request = types.create_struct(
            ItemKind::Struct,
            Ident::owned("AddCommentRequest"),
            vec![
                field("post_id", post_id_ty, rn(types.bump, "postId")),
                field("author_id", user_id_ty, rn(types.bump, "authorId")),
                field("text", Type::String, None),
                field(
                    "reply_to",
                    Type::Option(types.bump.alloc(comment_id_ty)),
                    rn(types.bump, "replyTo"),
                ),
            ],
            0,
            IMPL_FROM_JSON,
        );
        structs.push(add_comment_request);

        let get_feed_request = types.create_struct(
            ItemKind::Struct,
            Ident::owned("GetFeedRequest"),
            vec![
                field("user_id", user_id_ty, rn(types.bump, "userId")),
                field("limit", Type::U32, None),
            ],
            0,
            IMPL_FROM_JSON,
        );
        structs.push(get_feed_request);

        let feed_response = types.create_struct(
            ItemKind::Struct,
            Ident::owned("FeedResponse"),
            vec![field(
                "posts",
                Type::Vec(types.bump.alloc(Type::Struct(post))),
                None,
            )],
            0,
            IMPL_TO_JSON,
        );
        structs.push(feed_response);

        let profile_response = types.create_struct(
            ItemKind::Struct,
            Ident::owned("ProfileResponse"),
            vec![
                field("user", Type::Struct(user), None),
                field(
                    "posts",
                    Type::Vec(types.bump.alloc(Type::Struct(post))),
                    None,
                ),
                field(
                    "followers",
                    Type::Vec(types.bump.alloc(Type::Struct(user_summary))),
                    None,
                ),
            ],
            0,
            IMPL_TO_JSON,
        );
        structs.push(profile_response);

        let user_data = types.create_struct(
            ItemKind::Struct,
            Ident::owned("UserData"),
            vec![
                field("user", Type::Struct(user), None),
                field(
                    "posts",
                    Type::Vec(types.bump.alloc(Type::Struct(post))),
                    None,
                ),
                field(
                    "followers",
                    Type::Vec(types.bump.alloc(Type::Struct(user_summary))),
                    None,
                ),
            ],
            0,
            IMPL_TO_JSON,
        );
        structs.push(user_data);

        SocialMediaTask { structs, enums }
    }

    pub fn codegen_models(&self, lib: &Libary) -> Vec<u8> {
        let mut out: Vec<TokenTree> = Vec::new();
        lib.gen_module_prelude(&mut out);
        let json_mask = IMPL_TO_JSON | IMPL_FROM_JSON;
        for e in &self.enums {
            splat!((&mut out); #[[derive(Clone)]]);
            e.generate_def_with_flags(&mut out, lib, true, e.flags & json_mask);
        }
        for s in &self.structs {
            if s.kind == ItemKind::Tuple {
                splat!((&mut out); #[[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]]);
            } else {
                splat!((&mut out); #[[derive(Clone)]]);
            }
            s.generate_def_with_flags(&mut out, lib, true, s.flags & json_mask);
        }
        token::to_rust(out.into_iter().collect())
    }

    pub fn codegen_models_multi(&self, lib: &Libary) -> Vec<u8> {
        let mut out: Vec<TokenTree> = Vec::new();
        lib.gen_module_prelude_multi(&mut out);
        for e in &self.enums {
            splat!((&mut out); #[[derive(Clone)]]);
            e.generate_def_with_flags(&mut out, lib, true, e.flags);
        }
        for s in &self.structs {
            if s.kind == ItemKind::Tuple {
                splat!((&mut out); #[[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]]);
            } else {
                splat!((&mut out); #[[derive(Clone)]]);
            }
            s.generate_def_with_flags(&mut out, lib, true, s.flags);
        }
        token::to_rust(out.into_iter().collect())
    }

    pub fn bench_multi(
        &self,
        name: &str,
        lib: Libary,
        scenarios: &[Scenario],
        samples: u32,
    ) -> anyhow::Result<(Vec<(String, String)>, Vec<(Scenario, Perf)>)> {
        let mut b = Benchy::open(name, &lib.dependencies_multi())?;
        b.modification_target = "src/models.rs".to_string();

        b.write_bytes("models.rs", &self.codegen_models_multi(&lib))?;
        b.write_bytes("compat.rs", &lib.compat_module_multi_bytes())?;
        b.write_bytes("db.rs", include_bytes!("social/db_binary_template.rs"))?;
        b.write_bytes("routes.rs", include_bytes!("social/routes_template.rs"))?;
        b.write_bytes("main.rs", include_bytes!("social/main_template.rs"))?;

        let mut result = Vec::new();
        for scenario in scenarios {
            let perf = if let Scenario::RuntimeBenchmark { profile } = scenario {
                self.seed_and_bench_runtime_multi(&mut b, *profile)?
            } else {
                b.bench(samples, scenario.clone())
            };
            let perf = perf?;
            println!("{:?}\n   {}", scenario, perf);
            result.push((scenario.clone(), perf));
        }
        let versions = b.read_versions();
        Ok((versions, result))
    }

    fn seed_and_bench_runtime_multi(
        &self,
        b: &mut Benchy,
        profile: BuildProfile,
    ) -> Result<Result<Perf, anyhow::Error>, anyhow::Error> {
        self.seed_and_bench(b, profile, "social.db.bin")
    }

    pub fn bench(
        &self,
        name: &str,
        lib: Libary,
        scenarios: &[Scenario],
        samples: u32,
    ) -> anyhow::Result<(Vec<(String, String)>, Vec<(Scenario, Perf)>)> {
        let mut b = Benchy::open(name, &lib.dependencies())?;
        b.modification_target = "src/models.rs".to_string();

        b.write_bytes("models.rs", &self.codegen_models(&lib))?;
        b.write_bytes("compat.rs", &lib.compat_module_bytes())?;
        b.write_bytes("db.rs", include_bytes!("social/db_template.rs"))?;
        b.write_bytes("routes.rs", include_bytes!("social/routes_template.rs"))?;
        b.write_bytes("main.rs", include_bytes!("social/main_template.rs"))?;

        let mut result = Vec::new();
        for scenario in scenarios {
            let perf = if let Scenario::RuntimeBenchmark { profile } = scenario {
                self.seed_and_bench_runtime(&mut b, *profile)?
            } else {
                b.bench(samples, scenario.clone())
            };
            let perf = perf?;
            println!("{:?}\n   {}", scenario, perf);
            result.push((scenario.clone(), perf));
        }
        let versions = b.read_versions();
        Ok((versions, result))
    }

    fn seed_and_bench_runtime(
        &self,
        b: &mut Benchy,
        profile: BuildProfile,
    ) -> Result<Result<Perf, anyhow::Error>, anyhow::Error> {
        self.seed_and_bench(b, profile, "social.db.json")
    }

    fn seed_and_bench(
        &self,
        b: &mut Benchy,
        profile: BuildProfile,
        db_filename: &str,
    ) -> Result<Result<Perf, anyhow::Error>, anyhow::Error> {
        let (exe, exe_size) = b.build_and_strip(profile)?;

        if b.name.starts_with("baseline") {
            return Ok(Ok(Perf {
                instructions: Default::default(),
                cycles: Default::default(),
                task_clock: Default::default(),
                duration: Default::default(),
                build_size: Some(exe_size),
            }));
        }

        let db_path = b.crate_directory.join(db_filename);
        let db_path_str = db_path.to_string_lossy();

        let mut seed_proc = std::process::Command::new(&exe)
            .arg(&*db_path_str)
            .arg("seed")
            .arg("1")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        {
            let mut stdin = seed_proc.stdin.take().unwrap();
            stdin.write_all(b"")?;
        }
        let seed_status = seed_proc.wait()?;
        if !seed_status.success() {
            return Ok(Err(anyhow::anyhow!("Seed failed")));
        }

        let request_json = r#"{"userId":1,"limit":10}"#;
        let command = format!("{db_path_str} get_feed 10000");
        Ok(Ok(b.perf_stat_runtime(&exe, exe_size, &command, request_json.as_bytes())?))
    }

    fn build_social_crate(&self, lib: &Libary) -> anyhow::Result<Benchy> {
        let name = format!("{}_social", lib.crate_prefix());
        let b = Benchy::open(&name, &lib.dependencies())?;
        b.write_bytes("models.rs", &self.codegen_models(lib))?;
        b.write_bytes("compat.rs", &lib.compat_module_bytes())?;
        b.write_bytes("db.rs", include_bytes!("social/db_template.rs"))?;
        b.write_bytes("routes.rs", include_bytes!("social/routes_template.rs"))?;
        b.write_bytes("main.rs", include_bytes!("social/main_template.rs"))?;

        let status = Command::new("cargo")
            .arg("build")
            .env("RUSTFLAGS", "")
            .env(
                "CARGO_TARGET_DIR",
                b.crate_directory.join("./target").as_os_str(),
            )
            .stderr(Stdio::inherit())
            .stdout(Stdio::null())
            .current_dir(&b.crate_directory)
            .status()?;
        if !status.success() {
            bail!("{} failed to build", name);
        }
        Ok(b)
    }

    fn run_route(exe: &Path, db_path: &Path, route: &str, input: &str) -> anyhow::Result<String> {
        let mut child = Command::new(exe)
            .arg(db_path.to_string_lossy().as_ref())
            .arg(route)
            .arg("1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("spawning social binary")?;
        {
            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(input.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("route '{}' failed: {}", route, stderr);
        }
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn verify(&self, libraries: &[Libary]) -> anyhow::Result<()> {
        use verify_types::*;

        let test_routes: &[(&str, &str, RouteOutput)] = &[
            ("seed", "", RouteOutput::Seed),
            ("get_feed", r#"{"userId":1,"limit":10}"#, RouteOutput::Feed),
            (
                "get_profile",
                r#"{"userId":1,"limit":10}"#,
                RouteOutput::Profile,
            ),
            (
                "list_chats",
                r#"{"userId":1,"limit":10}"#,
                RouteOutput::ListChats,
            ),
            (
                "create_post",
                r#"{"authorId":1,"medium":{"Text":{"body":"test post"}},"caption":"test","tags":["a","b"]}"#,
                RouteOutput::Db,
            ),
            (
                "send_message",
                r#"{"senderId":1,"groupId":1,"content":"hello","replyTo":null}"#,
                RouteOutput::Db,
            ),
            (
                "add_comment",
                r#"{"postId":1,"authorId":1,"text":"nice","replyTo":null}"#,
                RouteOutput::Db,
            ),
        ];

        let jsony_lib = Libary::Jsony { path: None };
        println!("  building jsony (reference)...");
        let jsony_b = self.build_social_crate(&jsony_lib)?;
        let jsony_exe = jsony_b
            .crate_directory
            .join(format!("./target/debug/{}", jsony_b.name));
        let jsony_db = jsony_b.crate_directory.join("verify.db.json");
        let _ = std::fs::remove_file(&jsony_db);

        let mut reference_outputs: Vec<String> = Vec::new();
        for (route, input, _) in test_routes {
            let out = Self::run_route(&jsony_exe, &jsony_db, route, input)
                .with_context(|| format!("jsony route '{}'", route))?;
            reference_outputs.push(out);
        }

        for (i, (route, _, kind)) in test_routes.iter().enumerate() {
            let json = &reference_outputs[i];
            if let Err(e) = kind.parse_with_jsony(json) {
                bail!(
                    "jsony reference output for '{}' failed to parse: {}\n  json: {}",
                    route,
                    e,
                    json
                );
            }
        }
        println!("  jsony reference: all routes produce valid, parseable output");

        let mut all_pass = true;

        for lib in libraries {
            if matches!(lib, Libary::Jsony { .. }) {
                continue;
            }
            if !lib.supports_social() {
                println!("  skipping {} (unsupported)", lib.name());
                continue;
            }

            println!("  building {}...", lib.name());
            let b = match self.build_social_crate(lib) {
                Ok(b) => b,
                Err(e) => {
                    println!("  SKIP {} (build failed: {})", lib.name(), e);
                    all_pass = false;
                    continue;
                }
            };

            let exe = b.crate_directory.join(format!("./target/debug/{}", b.name));
            let db_path = b.crate_directory.join("verify.db.json");
            let _ = std::fs::remove_file(&db_path);

            for (i, (route, input, kind)) in test_routes.iter().enumerate() {
                let actual = match Self::run_route(&exe, &db_path, route, input) {
                    Ok(out) => out,
                    Err(e) => {
                        println!("  FAIL  {:<12}  route={}  error: {}", lib.name(), route, e);
                        all_pass = false;
                        continue;
                    }
                };

                let ref_json = &reference_outputs[i];
                match kind.compare_jsony(ref_json, &actual) {
                    Ok(()) => {
                        println!("  OK    {:<12}  route={}", lib.name(), route);
                    }
                    Err(e) => {
                        all_pass = false;
                        println!("  FAIL  {:<12}  route={}  {}", lib.name(), route, e);
                    }
                }
            }
        }

        if all_pass {
            println!("\n  verification passed");
        } else {
            bail!("verification failed");
        }

        Ok(())
    }
}

mod verify_types {
    use jsony::Jsony;

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VTimestamp {
        pub ms: i64,
    }

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VUserId(pub u64);

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VPostId(pub u64);

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VCommentId(pub u64);

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VGroupChatId(pub u64);

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VMessageId(pub u64);

    #[derive(Jsony, Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[jsony(Json)]
    pub struct VNotificationId(pub u64);

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub enum VPostMedium {
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

    #[derive(Jsony, Debug, PartialEq, Clone)]
    #[jsony(Json)]
    pub enum VNotificationKind {
        Like,
        Comment,
        Follow,
        Mention,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VUser {
        pub id: VUserId,
        #[jsony(rename = "userName")]
        pub username: String,
        #[jsony(rename = "displayName")]
        pub display_name: String,
        pub bio: Option<String>,
        #[jsony(rename = "avatarUrl")]
        pub avatar_url: Option<String>,
        #[jsony(rename = "createdAt")]
        pub created_at: VTimestamp,
        #[jsony(rename = "followerCount")]
        pub follower_count: u32,
        #[jsony(rename = "followingCount")]
        pub following_count: u32,
        pub verified: bool,
        pub email: Option<String>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VUserSummary {
        pub id: VUserId,
        #[jsony(rename = "userName")]
        pub username: String,
        #[jsony(rename = "displayName")]
        pub display_name: String,
        #[jsony(rename = "avatarUrl")]
        pub avatar_url: Option<String>,
        pub verified: bool,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VFollowRelation {
        #[jsony(rename = "followerId")]
        pub follower_id: VUserId,
        #[jsony(rename = "followingId")]
        pub following_id: VUserId,
        #[jsony(rename = "createdAt")]
        pub created_at: VTimestamp,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VPost {
        pub id: VPostId,
        #[jsony(rename = "authorId")]
        pub author_id: VUserId,
        pub medium: VPostMedium,
        pub caption: String,
        #[jsony(rename = "likeCount")]
        pub like_count: u32,
        #[jsony(rename = "createdAt")]
        pub created_at: VTimestamp,
        pub tags: Vec<String>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VPostComment {
        pub id: VCommentId,
        #[jsony(rename = "postId")]
        pub post_id: VPostId,
        #[jsony(rename = "authorId")]
        pub author_id: VUserId,
        pub text: String,
        pub timestamp: VTimestamp,
        pub likes: u32,
        #[jsony(rename = "replyTo")]
        pub reply_to: Option<VCommentId>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VGroupChat {
        pub id: VGroupChatId,
        pub name: String,
        pub description: Option<String>,
        pub members: Vec<VUserId>,
        #[jsony(rename = "createdAt")]
        pub created_at: VTimestamp,
        #[jsony(rename = "adminId")]
        pub admin_id: VUserId,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VChatMessage {
        pub id: VMessageId,
        #[jsony(rename = "senderId")]
        pub sender_id: VUserId,
        #[jsony(rename = "groupId")]
        pub group_id: VGroupChatId,
        pub content: String,
        pub timestamp: VTimestamp,
        pub edited: bool,
        #[jsony(rename = "replyTo")]
        pub reply_to: Option<VMessageId>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VNotification {
        pub id: VNotificationId,
        #[jsony(rename = "userId")]
        pub user_id: VUserId,
        pub kind: VNotificationKind,
        #[jsony(rename = "sourceId")]
        pub source_id: VUserId,
        #[jsony(rename = "referenceId")]
        pub reference_id: VPostId,
        pub message: String,
        pub read: bool,
        pub timestamp: VTimestamp,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VDatabase {
        pub users: Vec<VUser>,
        pub posts: Vec<VPost>,
        #[jsony(rename = "postComments")]
        pub comments: Vec<VPostComment>,
        #[jsony(rename = "groupChats")]
        pub group_chats: Vec<VGroupChat>,
        pub messages: Vec<VChatMessage>,
        pub notifications: Vec<VNotification>,
        pub followers: Vec<VFollowRelation>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VFeedResponse {
        pub posts: Vec<VPost>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VProfileResponse {
        pub user: VUser,
        pub posts: Vec<VPost>,
        pub followers: Vec<VUserSummary>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VUserData {
        pub user: VUser,
        pub posts: Vec<VPost>,
        pub followers: Vec<VUserSummary>,
    }

    #[derive(Jsony, Debug, PartialEq)]
    #[jsony(Json)]
    pub struct VSeedResponse {
        pub ok: bool,
    }

    pub enum RouteOutput {
        Seed,
        Feed,
        Profile,
        ListChats,
        Db,
    }

    impl RouteOutput {
        pub fn parse_with_jsony(&self, json: &str) -> Result<(), String> {
            match self {
                RouteOutput::Seed => {
                    jsony::from_json::<VSeedResponse>(json).map_err(|e| format!("{e:?}"))?;
                }
                RouteOutput::Feed => {
                    jsony::from_json::<VFeedResponse>(json).map_err(|e| format!("{e:?}"))?;
                }
                RouteOutput::Profile => {
                    jsony::from_json::<VProfileResponse>(json).map_err(|e| format!("{e:?}"))?;
                }
                RouteOutput::ListChats => {
                    jsony::from_json::<Vec<VGroupChat>>(json).map_err(|e| format!("{e:?}"))?;
                }
                RouteOutput::Db => {
                    jsony::from_json::<VDatabase>(json).map_err(|e| format!("{e:?}"))?;
                }
            }
            Ok(())
        }

        pub fn compare_jsony(&self, expected_json: &str, actual_json: &str) -> Result<(), String> {
            match self {
                RouteOutput::Seed => {
                    let expected: VSeedResponse = jsony::from_json(expected_json)
                        .map_err(|e| format!("parse expected: {e:?}"))?;
                    let actual: VSeedResponse = jsony::from_json(actual_json)
                        .map_err(|e| format!("parse actual: {e:?}"))?;
                    if expected != actual {
                        return Err(format!("expected {:?}, got {:?}", expected, actual));
                    }
                }
                RouteOutput::Feed => {
                    let expected: VFeedResponse = jsony::from_json(expected_json)
                        .map_err(|e| format!("parse expected: {e:?}"))?;
                    let actual: VFeedResponse = jsony::from_json(actual_json)
                        .map_err(|e| format!("parse actual: {e:?}"))?;
                    if expected != actual {
                        return Err(format!(
                            "feed mismatch: {} expected posts, {} actual posts",
                            expected.posts.len(),
                            actual.posts.len()
                        ));
                    }
                }
                RouteOutput::Profile => {
                    let expected: VProfileResponse = jsony::from_json(expected_json)
                        .map_err(|e| format!("parse expected: {e:?}"))?;
                    let actual: VProfileResponse = jsony::from_json(actual_json)
                        .map_err(|e| format!("parse actual: {e:?}"))?;
                    if expected != actual {
                        return Err(format!(
                            "profile mismatch: user {:?} vs {:?}",
                            expected.user.username, actual.user.username
                        ));
                    }
                }
                RouteOutput::ListChats => {
                    let expected: Vec<VGroupChat> = jsony::from_json(expected_json)
                        .map_err(|e| format!("parse expected: {e:?}"))?;
                    let actual: Vec<VGroupChat> = jsony::from_json(actual_json)
                        .map_err(|e| format!("parse actual: {e:?}"))?;
                    if expected != actual {
                        return Err(format!(
                            "list_chats mismatch: {} expected, {} actual",
                            expected.len(),
                            actual.len()
                        ));
                    }
                }
                RouteOutput::Db => {
                    let expected: VDatabase = jsony::from_json(expected_json)
                        .map_err(|e| format!("parse expected: {e:?}"))?;
                    let actual: VDatabase = jsony::from_json(actual_json)
                        .map_err(|e| format!("parse actual: {e:?}"))?;
                    if expected != actual {
                        if expected.users != actual.users {
                            return Err("db users mismatch".into());
                        }
                        if expected.posts != actual.posts {
                            return Err("db posts mismatch".into());
                        }
                        if expected.comments != actual.comments {
                            return Err("db comments mismatch".into());
                        }
                        if expected.group_chats != actual.group_chats {
                            return Err("db group_chats mismatch".into());
                        }
                        if expected.messages != actual.messages {
                            return Err("db messages mismatch".into());
                        }
                        if expected.notifications != actual.notifications {
                            return Err("db notifications mismatch".into());
                        }
                        if expected.followers != actual.followers {
                            return Err("db followers mismatch".into());
                        }
                        return Err("db mismatch (unknown field)".into());
                    }
                }
            }
            Ok(())
        }
    }
}
