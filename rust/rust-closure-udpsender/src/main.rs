use std::thread;

extern crate udpthread;

fn main(){
    println!("hogehoge");
    let x = udpthread::closure::closure(10000);
    x("hoge");
    x("moge");
    x("aaa");
    x("bbb");
    x("ccc");
    x("ddd");

    thread::sleep_ms(1000);
}
