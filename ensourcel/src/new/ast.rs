struct Magic{
    byte_size : u8,
    page_size : u8,
    qbyte_size : u8,
    foreword : Scroll
}

struct Scroll{
    name : String
}