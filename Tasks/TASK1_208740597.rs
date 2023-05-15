fn main(){
    let string1 = "Hello";
    let string2 = " World!"
    concatenated_string = concatenate_strings(&string1, &string2);
    println!("The concatenated string is {}", concatenated_string);

}

fn concatenate_strings(s1: String, s2: String) -> String{
    let result = s1.push_str(s2);
    return result;
}
