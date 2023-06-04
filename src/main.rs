fn main() {
    /*
     * literals là gì ?
     * literals là các giá trị cố định được đặt trực tiếp vào code
     * Số nguyên 1, số thực 1.2, ký tự 'a', xâu "abc", boolean true and kiểu đơn vị () 
     * đều có thể được diễn đạt bằng literals 
     */

    // Số nguyên
    let x = 1;

    println!("x = {}", x);

    // Sử dụng dấu gạch dưới để việc biểu diễn số được sáng sủa và dễ đọc hơn!
    let y = 1_000_000; // 1_000_000 = 1000000

    println!("y = {}", y);

    // Số thực
    let f = 1.0;

    println!("f = {}", f);

    // Ký tự
    let c = 'a';

    println!("c = {}", c);

    // Xâu
    let s = "abc";

    println!("s = {}", s);

    // Boolean
    let b = true;

    println!("b = {}", b);

    // Kiểu đơn vị
    let u = ();

    println!("u = {:?}", u);

    // let a:i32;
    // print!("a = {}", a) // lỗi vì a chưa được khởi tạo giá trị mặc định

    /*
    * tuple là gì ?
    * tuple là một cấu trúc dữ liệu cho phép lưu trữ nhiều giá trị có các kiểu dữ liệu khác nhau
    */

    // Khai báo tuple với 3 phần tử có kiểu dữ liệu lần lượt là i32, f64, char
    let tup: (i32, f64, char) = (500, 6.4, 'a');

    // Truy cập phần tử thứ nhất của tuple
    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    // Truy cập phần tử thứ hai của tuple
    println!("tup.1 = {}", tup.1);

    // truy cập thông qua 1 tên biến
    let _char = tup.2;

    println!("_char = {}", _char);


    /*
    * array là gì ?
    * array là một cấu trúc dữ liệu cho phép lưu trữ nhiều giá trị có cùng kiểu dữ liệu
    * array có độ dài cố định và không thể thay đổi
    */

    // Khai báo array với 5 phần tử có kiểu dữ liệu là i32
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Truy cập phần tử thứ nhất của array
    let x = arr[0];

    println!("x = {}", x);

    // Truy cập phần tử thứ hai của array
    println!("arr.1 = {}", arr[1]);

    // in ra toàn bộ array
    println!("arr = {:?}", arr);
}
