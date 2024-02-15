use passtool::*;
use serial_test::serial;

#[test]
fn passtable_test() -> Result<(), Error>{
    let message = "super secret message";
    let password = "super secret password";
    let mut pt = PassTable::new();
    let name = String::from("test");
    pt.add_password(&name, message, password)?;
    let pass = pt.get_password(&name, password)?;
    assert_eq!(pass, message);
    Ok(())
}

#[test]
fn remove_test() -> Result<(), Error>{
    let mut pt = PassTable::new();
    pt.add_password("test", "password", "1234")?;
    pt.add_password("test2", "password2", "1234")?;
    pt.remove_password("test")?;
    assert!(pt.get_password("test", "1234").is_err_and(|x| x == PassNotFound));
    assert_eq!(pt.get_password("test2", "1234").unwrap(), "password2");
    assert!(pt.remove_password("test").is_err_and(|x| x == PassNotFound));
    Ok(())
}

#[test]
fn passtable_test2() -> Result<(), Error>{
    use random_string::generate;
    let charset = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let data: Vec<(String, String, String)> = (0..10).map(|x| (x.to_string(), generate(100, charset), generate(50, charset))).collect();
    let mut pt = PassTable::new();
    for (n, m, p) in &data{
        pt.add_password(n, m, p)?;
    }

    for (n, m, p) in &data{
        let pass = pt.get_password(n, p)?;
        assert_eq!(&pass, m);
    }
    Ok(())
}

#[test]
fn incorrect_password_passtable_test() -> Result<(), Error>{
    let message = "super secret message";
    let password = "super secret password";
    let mut pt = PassTable::new();
    let name = String::from("test");
    pt.add_password(&name, message, password)?;
    let pass = pt.get_password(&name, "bebra");
    assert!(pass.is_err_and(|x| x == IncorrectPass));
    Ok(())
}
#[test]
fn not_found_passtable_test() -> Result<(), Error>{
    let message = "super secret message";
    let password = "super secret password";
    let mut pt = PassTable::new();
    let name = String::from("test");
    pt.add_password(&name, message, password)?;
    let pass = pt.get_password(&"test2".to_string(), "bebra");
    assert!(pass.is_err_and(|x| if let PassNotFound = x {true} else {false}));
    Ok(())
}

#[test]
fn alredy_exists_passtable_test() -> Result<(), Error>{
    let message = "super secret message";
    let password = "super secret password";
    let mut pt = PassTable::new();
    let name = String::from("test");
    pt.add_password(&name, message, password)?;
    let res = pt.add_password(&name, message, password);
    assert!(res.is_err_and(|x| if let PassExists = x {true} else {false}));
    Ok(())
}

#[test]
fn password_encrypt_test() -> Result<(), aes_gcm_siv::Error>{
    let password = "super secret password";
    let message = Vec::from(b"Hello world!");
    let cypher = encrypt(&message, password)?;
    let message2 = decrypt(&cypher, password)?;
    assert_eq!(&message, &message2);
    Ok(())
}

#[test]
fn incorrect_password_encrypt_test2() -> Result<(), aes_gcm_siv::Error>{
    let password = "super secret password";
    let password2 = "super not secret password";
    let message = Vec::from(b"Hello world!");
    let cypher = encrypt(&message, password)?;
    let message2 = decrypt(&cypher, password2);
    assert!(message2.is_err());
    Ok(())
}

#[test]
#[serial]
fn save_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut pt = PassTable::new();
    pt.add_password("pass1", "test1", "password1")?;
    pt.add_password("pass2", "test2", "password2")?;
    pt.add_password("pass3", "test3", "password3")?;
    pt.to_file("passwords.pt")?;
    Ok(())
}

#[test]
#[serial]
fn save_and_load_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut pt = PassTable::new();
    pt.add_password("pass1", "test1", "password1")?;
    pt.add_password("pass2", "test2", "password2")?;
    pt.add_password("pass3", "test3", "password3")?;
    pt.to_file("passwords.pt")?;

    let pt2 = PassTable::from_file("passwords.pt")?;
    assert_eq!(pt, pt2);
    assert_eq!("test3", pt2.get_password("pass3", "password3")?);
    Ok(())
}