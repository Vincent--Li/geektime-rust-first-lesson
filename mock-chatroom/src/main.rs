use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Gender::Unspecified => write!(f, "Gender Unspecified"),
            Gender::Female => write!(f, "Gender Female"),
            Gender::Male => write!(f, "Gender Male"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId {}", self.0)
    }
}

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

impl Display for TopicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TopicId {}", self.0)
    }
}

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 如果Event包含的枚举类型是复杂结构, 需要引入生命周期
#[derive(Debug, Clone)]
enum Event<'a> {
    Join((&'a User, &'a Topic)),
    Leave((&'a User, &'a Topic)),
    Message((&'a User, &'a Topic, &'a str)),
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: ({},{},{})", self.id, self.name, self.gender)
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Topic: ({}, {}, {})", self.id, self.name, self.owner)
    }
}

// 带生命周期的结构体 impl trait
impl<'a> Display for Event<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match 使用 self(不可变引用) 或 *self(可变引用)
        match *self {
            Event::Join((user, topic)) => write!(f, "({} Join {})", user, topic),
            Event::Leave((user, topic)) => write!(f, "({} Leave {})", user, topic),
            Event::Message((user, topic, message)) => {
                write!(f, "({} in {} send a message: {})", user, topic, message)
            }
        }
    }
}

// impl的时候也要显式的生命生命周期
impl<'a> Event<'a> {
    fn show_random(event_name: String, action: String) {
        println!("{} {}", event_name, action)
    }
}

fn process_event(event: &Event) {
    // 匹配模式可以对 struct, enum做处理
    match *event {
        Event::Join(e) => println!("User {} join", e.0),
        Event::Leave(e) => println!("User {} leave", e.0),
        _ => println!("===========================\nMessage handled individually\n==========================="),
    }

    // 可以只关注自己想要的部分
    if let Event::Message(e) = event {
        println!("Topic: {}, User {} say {}", e.1, e.0, e.2)
    }
}

fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "Rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join((&alice, &topic));
    let event2 = Event::Join((&bob, &topic));
    let event3 = Event::Message((&alice, &topic, "Welcome bob into the rust world".as_ref()));
    let event4 = Event::Leave((&alice, &topic));
    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
    process_event(&event4);
}
