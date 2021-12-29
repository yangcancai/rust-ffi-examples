mod player;
#[test]
fn it_work(){
    use player::a;
    let p = player::new();
     player::make(p);
    assert_eq!(player::get_id(p), 1);
    assert_eq!(player::get_name(p), "hello");
    assert_eq!(a(1),2)
}