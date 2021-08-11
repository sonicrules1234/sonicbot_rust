use acid_store::repo::Commit;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use acid_store::repo::{OpenOptions, value::ValueRepo, OpenMode};
use acid_store::store::{DirectoryConfig};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SonicObject {
    //jsonstr: String,
    pub value: Value,
}
pub struct SonicPersistObject {
    pub tree: ValueRepo<String>,
}

impl SonicObject {
    pub fn new(value: impl Serialize) -> Self {
        let jsonstring = serde_json::to_string(&value).unwrap();
        let v: Value = serde_json::from_str(jsonstring.as_str()).unwrap();
        ////println!("snew value is '{:?}'", v);
        Self {
            //jsonstr: jsonstring,
            value: v,
        }
    }
    pub fn get(&self, key: &str) -> SonicObject {
        ////println!("{}", self.value);
        //println!("{:?}", self.value);
        SonicObject::new(self.value.get(key.to_string()).unwrap().to_owned())
    }
    pub fn contains(&mut self, key: &str) -> bool {
        let svalue = self.value.as_object_mut().unwrap();
        svalue.contains_key(key)
    }
    pub fn insert(&mut self, key: &str, value: impl Serialize) -> () {
        let svalue = self.value.as_object_mut().unwrap();
        //println!("svalue is now '{:?}'", svalue);
        ////println!("svalue = '{:?}'", svalue);
        let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        svalue.insert(key.to_string(), serde_json::from_str(&sobject).unwrap());
        //svalue.remove("value");        
        ////println!("svalue = '{:?}'", svalue);
        let jsonstring = serde_json::to_string(&svalue).unwrap();
        let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        self.value = v;
    }
    pub fn push(&mut self, value: impl Serialize) -> () {
        let mut svalue = self.value.as_array_mut().unwrap().to_vec();
        ////println!("svalue = '{:?}'", svalue);
        let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        svalue.push(serde_json::from_str(&sobject).unwrap());
        //svalue.remove("value");        
        ////println!("svalue = '{:?}'", svalue);
        let jsonstring = serde_json::to_string(&svalue).unwrap();
        let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        self.value = v;
    }
    pub fn remove(&mut self, key: &str) -> () {
        let svalue = self.value.as_object_mut().unwrap();
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.insert(key.to_string(), serde_json::from_str(&sobject).unwrap());
        svalue.remove(key);        
        ////println!("svalue = '{:?}'", svalue);
        let jsonstring = serde_json::to_string(&svalue).unwrap();
        let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        self.value = v;
    }
    pub fn getindex(&mut self, index: usize) -> SonicObject {
        SonicObject::new(self.value[index].to_owned())
    }
    pub fn getindexvalue(&mut self, index: usize) -> Value {
        self.value[index].to_owned()
    }
    pub fn removeindex(&mut self, index: usize) -> () {
        let mut svalue = self.value.as_array_mut().unwrap().to_vec();
        ////println!("svalue = '{:?}'", svalue);
        //let sobject = serde_json::to_string(&value).unwrap();
        ////println!("sobject = '{:?}'", sobject);
        //svalue.insert(key.to_string(), serde_json::from_str(&sobject).unwrap());
        svalue.remove(index);        
        ////println!("svalue = '{:?}'", svalue);
        let jsonstring = serde_json::to_string(&svalue).unwrap();
        let v: Value = serde_json::from_str(&jsonstring).unwrap();        
        //self.jsonstr = jsonstring;
        self.value = v;
    }
    pub fn getvalue(&mut self, key: &str) -> Value {
        self.value.get(key.to_string()).unwrap().to_owned()
    }
}
impl SonicPersistObject {
    pub fn new(filepath: PathBuf) -> Self {
        let tree = OpenOptions::new().mode(OpenMode::Create).open(&DirectoryConfig{ path: filepath }).unwrap();//sled::open(&filepath).unwrap();
        Self {
            tree: tree,
        }
    }
    pub fn contains(&mut self, key: &str) -> bool {
        self.tree.contains(key)
    }
    pub fn get(&self, key: &str) -> SonicObject {
        let jsonstring: String = self.tree.get(&key.to_string()).unwrap();
        //let p: Value = serde_json::from_str(String::from_utf8(self.tree.get(key).unwrap().unwrap().as_ref().to_vec()).unwrap().as_str()).unwrap();
        let p: Value = serde_json::from_str(jsonstring.as_str()).unwrap();
        ////println!("p is '{:?}'", p);
        SonicObject::new(p)
    }
    pub fn insert(&mut self, key: &str, value: impl Serialize) -> () {
        //let mut sobj = SonicObject::new(value);
        self.tree.insert(key.to_string(), &serde_json::to_string(&value).unwrap()).unwrap();
        self.tree.commit().unwrap();
    }
    pub fn flush(&mut self) -> () {
        self.tree.commit().unwrap();
    }
}
pub fn getemptyvalue() -> Value {
    let data = r#"{}"#;
    let v: Value = serde_json::from_str(data).unwrap();
    v
}



#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path};
    use crate::SonicPersistObject;
    
    use serde_json::json;
    
    #[test]
    fn it_works() {
        if Path::new("test.db").exists() {
            fs::remove_dir_all("test.db").unwrap();
        }
        let mut pobj = SonicPersistObject::new(Path::new("test.db").to_path_buf());
        //println!("1");
        pobj.insert("test", json!({}));
        let mut a = pobj.get("test");
        //println!("2");
        a.insert("a", json!({}));
        //println!("2.5");
        let mut b = a.get("a");
        //println!("b is: '{:?}'", b);
        //println!("3");
        b.insert("test1", "good");
        //println!("b is now '{:?}'", b);
        //a.remove("a");
        a.insert("a", b.value);
        a.insert("b", json!([]));
        let mut c = a.get("b");
        c.push("blah");
        //println!("c is now '{:?}'", c);
        //println!("a is now '{:?}'", a);
        //a.remove("b");
        a.insert("b", c.value);
        //println!("4");
        pobj.insert("test", a.value);
        //println!("5");
        //assert_eq!(pobj.get("test").as_array().unwrap().to_vec(), vec!(1, 2, 3));
        assert_eq!(pobj.get("test").get("b").getindexvalue(0), "blah".to_string());
    }
}
