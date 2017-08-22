//RustからCに渡す構造体
#[repr(C)]
pub struct Telexistence{
    a: i32,
    destroy: extern "C" fn(), //Cから呼ぶための関数ポインタ
    callback: extern "C" fn(cb: &CB),
}

//結論から言うとimplで定義したメソッドはCから呼べない
#[no_mangle]
impl Telexistence{
    pub extern fn reference(&self) {
       println!("taking self by reference!");
    }
}

//Telesixtence objectをcreate method内で作ると、
//create methodを抜けた時に解放されてしまい
//C側で触るとsegmentation errorになるのでstatic objectにする
#[macro_use]
extern crate lazy_static;
    lazy_static! {
        pub static ref TELEXISTENCE: Telexistence = {
            let tex = Telexistence{a: 100, destroy: destroy, callback: callback};
            tex
        };
}

//CからRustに渡す構造体
#[repr(C)]
pub struct CB{
    a: i32,
    event: extern "C" fn(),
}

//CからCBを貰ってcallbackを発火させる
#[no_mangle]
pub extern "C" fn callback(cb: &CB){
    println!("callback");
    println!("callback {}", (*cb).a);
    (cb.event)();
}

//Cからの単純な関数呼び出し
#[no_mangle]
pub extern fn destroy(){
    println!("destroy called");
}

//CにRustの構造体を渡す関数
#[no_mangle]
pub extern fn create() -> &'static Telexistence{
    &TELEXISTENCE
}

