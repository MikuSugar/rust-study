fn main() {
    println!("Hello, world!");

    show(7);

    let x=5;
    let y={
        let x=3;
        x+1
    };
    println!("x:{},y:{}",x,y);

    println!("{}+{}={}",x,y,add(x,y));

    let mut  str=String::from("Hello");
    str.push_str(",World!");
    println!("{}",str);

    println!("{},{},max:{}",x,y,max(x,y));

    let mut cnt=0;
    let res = loop {
        cnt+=1;
        if cnt==5000 {break cnt}
    };
    println!("res:{}",res);

    cnt=4;
    while cnt>=0 {
        println!("cnt value: {}",cnt);
        cnt-=1;
    }
    
    let arr=[1,2,3,4,5];

    for num in arr.iter()  {
        println!("num:{}",num)
    }

    println!("\"{}\" len: {}",str,str_len(&str));

    add_str(&mut str);

    println!("str:{}",str);

    let slice = &arr[..2];

    for elem in slice.iter() {
        println!("{}",elem)
    }

    let user=User{
        name:String::from("miku"),
        age:16,
        active:true
    };

    println!("{}",user.name);

    let t=Mytuple(14,true);
    println!("t.0:{} ,t.1:{}",t.0,t.1);

    println!("{:?}",user);

    let rect=Rect{
        width:15,
        high:6
    };
    println!("{:?} area:{}",rect,rect.area());
}

fn show(x:i32){
    println!("{}",x)
}

fn add(x:i32,y:i32)->i32{
    x+y
}

fn max(x:i32,y:i32)->i32{
    if x>y {x}
    else {y}
}

//unsize 无符号整行
fn str_len(str:&String) -> usize {
    str.len()
}
fn add_str(str:&mut String){
    str.push_str(" Hi,Rust! ")
}

#[derive(Debug)]
struct User{
    name:String,
    age:usize,
    active:bool
}
struct Mytuple(i32,bool);

#[derive(Debug)]
struct Rect{
    width:usize,
    high:usize
}
impl Rect{
    fn area(&self)->usize{
        self.high*self.width
    }
}

