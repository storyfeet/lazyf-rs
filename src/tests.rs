
use flag;
use lzlist;
use brace;
use cfg;

use get::SGetter;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn args(){
    let v = ["-a","60"];
    let r = flag::ss_get("-a",&mut v.into_iter().map(|&a|String::from(a)));
    assert_eq!(r,Some("60".to_string()));

    let a = flag::Fg{}.get_s_def("-poop","nopoopforyou");
    assert_eq!(a,"nopoopforyou");

    let p = flag::FgTest::new(vec!["-b".to_string(),"3".to_string()]);
    assert_eq!(p.get_t_def("-b",0),3);

}

#[test]
fn reader(){
    let f = lzlist::LzList::load("test_data/lztest.lz").unwrap();

    assert_eq!( f.len(),2); 
    assert_eq!( f.get_s("config.ext0"),Some("4".to_string()));
    assert_eq!( f.get_t_def("config.ext0",0),4);
    assert_eq!( f.get_s("c2.lesson"),Some("3".to_string()));      

    for (i,v) in f.iter().enumerate(){
        if i == 0 {
            assert_eq!(v.name , "config")
        }
    }
    assert_eq!(f.get_s("c3.poo"),None);

}

#[test]
fn config(){
    let cg = cfg::Cfg::load("test_data/lztest.lz");
    assert_eq!(cg.get_t_def(("-fg","c2.lesson"),0),3);
}

fn strrep1(_:&str)->String{
    "plop".to_string()
}

fn rev(s:&str)->String{
    s.chars().rev().collect()
}

#[test]
fn string_edit() {
    assert_eq!(brace::replace("cd{a}b",&|_s|{
        "poop".to_string()
    }),"cdpoopb");
    assert_eq!(brace::replace("pp{a}b",&strrep1),"ppplopb");

    assert_eq!(rev("hello"),"olleh");

    assert_eq!(brace::replace("ab{cd{ef}g{hi}j}",&rev),"abjhigefdc");

    assert_eq!(brace::replace("ab{cd{ef}g{hij",&rev),"abhijgefdc");

    assert_eq!(brace::replace("as}b{ef}",&rev),"as}bfe");

    //fix test so not locked to my account
    assert_eq!(brace::env_replace("h{HOME}"),"h/home/matthew");
}

