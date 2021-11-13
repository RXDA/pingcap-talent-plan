use std::collections::HashMap;

#[derive(Default,Debug)]
pub struct KV{
    map: HashMap<String,String>
}


impl KV{
    pub fn set(&mut self,k:String,v:String){
        self.map.insert(k, v);
    }

    pub fn get(&self, k:String)->Option<String>{
        self.map.get(&k).cloned()
    }

    pub fn rm(&mut self, k:String){
        self.map.remove(&k);
    }
}