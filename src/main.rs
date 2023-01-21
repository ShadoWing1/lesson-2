fn main()
{
    let isim :&str= "Mehmet";
    let a: bool = true;
    let b: i32 = 15;    
    println!("Ismim: {} Yasim {}", isim, b);

    if a == true
    {
        println!("Merhaba {}", isim);
    }

    else if a != true && a != false
    {
        println!("Merhaba {}", isim);
    }

    else
    {
        println!("Who are you?");
    }
}