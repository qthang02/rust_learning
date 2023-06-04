fn main() {

    /*
     * Mutability (tính khải biến)
     * Mặc định trong Rust, các biến là không thể thay đổi giá trị (immutable)
     * Để khai báo biến có thể thay đổi giá trị, ta dùng từ khóa mut 
     */

    let mut mutable_binding = 5; // mutable variable

    println!("mutable_binding = {}", mutable_binding);

    mutable_binding = 6;

    println!("mutable_binding = {}", mutable_binding);

    let immutable_binding = 1; // immutable variable

    // immutable_binding = 2; ---> Error! 

    println!("immutable_binding = {}", immutable_binding);


    /*
    * Immutable và Const
    * Immutable:
    * - Mặc định trong Rust
    * - Không thể thay đổi giá trị
    * - Có thể shadowing
    * Const:
    * - Không thể thay đổi giá trị
    * - Không thể shadowing
    * - phải có giá trị khởi tạo
    */

    // const MAX_POINTS: u32; --> Error! Thiếu giá trị tại vị trí khởi tạo
    // MAX_POINTS = 100_000;
    const MAX_POINTS: u32 = 100_000;

    println!("MAX_POINTS = {}", MAX_POINTS);

    let immutable:u32; // immutable variable không cần giá trị khởi tạo đầu tiên
    immutable = 5;

    println!("immutable = {}", immutable);


    /*
    * Scope (phạm vi)
    * Các ràng buộc biến đều có một phạm vi dùng để giới hạn sự tồn tại của nó trong một block (khối lệnh). 
    *Một block là một tập hợp các câu lệnh được bao bởi dấu ngoặc nhọn {}.
    */

    let long_lived_binding = 1;

    // Đây là một block code, có scope nhỏ hơn so với hàm main
    {
        // Ràng buộc này chỉ tồn tại trong block này
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // Kết thúc của block

    // Lỗi! `short_lived_binding` không tồn tại trong phạm vi này
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment dòng lệnh trên

    println!("outer long: {}", long_lived_binding);


    /*
     * Shadowing (tính che phủ)
     * Đặt tên biến mới trùng với tên biến cũ
     * Điều này cho phép ta thay đổi kiểu dữ liệu của biến
     */

    
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // Ràng buộc này *shadows* shadowed_binding bên ngoài
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // Ràng buộc này *shadows* ràng buộc trước (let shadowed_binding = 1;)
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);


    /*
     * Declare first (khai báo trước)
     * Không thể sử dụng biến trước khi nó được khai báo giá trị
     */

    let a_binding;

    {
        let x = 2;

        // Khởi tạo ràng buộc biến đã khai báo phía trên
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Lỗi! Không được sử dụng ràng buộc chưa khởi tạo
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment dòng trên lại

    another_binding = 1;

    println!("another binding: {}", another_binding);

    /*
     * freezing (đóng băng)
     * Khi dữ liệu của một ràng buộc biến bị shadow bởi một ràng buộc có tính bất biến thì nó cũng sẽ bị freezes
     * Dữ liệu bị đóng băng sẽ không có khả năng sửa đổi cho đến khi ra khỏi phạm vi của ràng bụôc bất biến trên
     */

    let mut _mutable_integer = 7i32;

    {
        // Shadowing bởi một `_mutable_integer` bất biến
        let _mutable_integer = _mutable_integer;

        // Lỗi! Vì `_mutable_integer` bị đóng băng ở scope này
        // _mutable_integer = 50;
        // FIXME ^ Comment dòng trên lại

        // `_mutable_integer` đi ra khỏi scope này
    }

    // Không có lỗi! Vì `_mutable_integer` không bị đóng băng ở scope này
    _mutable_integer = 3;
}
