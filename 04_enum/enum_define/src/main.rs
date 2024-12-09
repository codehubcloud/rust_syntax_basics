#[derive(Debug)]
enum IpAddrKind
{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr
{
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKindWithDataType
{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message
{
    Qiut,
    Move
    {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message
{
    fn send_msg(self)
    {
        println!("{:?}", self);
    }
}

fn enum_usage()
{
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn enum_with_data_type_usage()
{
    let home = IpAddrKindWithDataType::V4(127, 0, 0, 1);
    let loopback = IpAddrKindWithDataType::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn message_usage()
{
    let msg1 = Message::Qiut;
    let msg2 = Message::Move {
        x: 50,
        y: 100,
    };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(0, 255, 255);
    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);
    msg1.send_msg();
}

fn main()
{
    enum_usage();
    enum_with_data_type_usage();
    message_usage();
}
