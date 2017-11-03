
//fn fibo (x: i32) -> i32 {

#[no_mangle]
pub extern fn fibo (x: i32) -> i32 {

	if x <= 2 {
		return 1;
	}
	else {
		return fibo (x - 1) + fibo (x - 2);
	}

}
