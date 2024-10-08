use std::fmt;

#[derive(Debug)]
struct FileSystemEntityProps<'a> {
    name: &'a str,
    id: usize,
}

#[derive(Debug)]
struct File<'a> {
    common_props: FileSystemEntityProps<'a>
}

struct Folder<'a> {
    common_props: FileSystemEntityProps<'a>,
    children: Vec<&'a mut dyn FileSystemEntity>,
}

struct SymbolicLink<'a> {
    common_props: FileSystemEntityProps<'a>,
    link: &'a dyn FileSystemEntity,
}

trait FileSystemEntity {
    fn get(&self, name: &str) -> Option<&dyn FileSystemEntity>;
    fn get_mut(&mut self, name: &str) -> Option<&mut dyn FileSystemEntity>;
    fn delete(&mut self);
    fn delete_id(&mut self, id: usize) -> Result<usize, &str>;
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
    fn id(&self) -> usize;
} 

impl FileSystemEntity for File<'_> {
    fn get(&self, name: &str) -> Option<&dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            None
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            None
        }
    }

    fn delete(&mut self) {
        println!("deleted file {}", self.common_props.name);
    }

    fn delete_id(&mut self, _: usize) -> Result<usize, &str> {
        Err("Unsupported!")
    }

    fn to_string(&self) -> String {
        String::from(self.common_props.name)
    }

    fn name(&self) -> &str {
        self.common_props.name
    }

    fn id(&self) -> usize {
        self.common_props.id
    }
}

impl FileSystemEntity for Folder<'_> {
    fn get(&self, name: &str) -> Option<&dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            for item in &self.children {
                if item.name() == name {
                    return Some(&**item);
                }
            }
            None
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            for item in &mut self.children {
                if item.name() == name {
                    return Some(*item);
                }
            }
            None
        }
    }

    fn delete(&mut self) {
        while !self.children.is_empty() {
            let x: &mut dyn FileSystemEntity = self.children.pop().unwrap();
            x.delete();
        }
        println!("deleted folder {}", self.common_props.name);
    }

    fn delete_id(&mut self, id: usize) -> Result<usize, &str>
    {
        let mut item_to_remove : Option<usize> = None; 
        for (pos, item) in self.children.iter_mut().enumerate() {
            if item.id() == id {
                item.delete();
                item_to_remove = Some(pos);
                break;
            }
        }

        if let Some(x) = item_to_remove {
            self.children.remove(x);
            return Ok(id);
        }

        Err("Not found")
    }

    fn to_string(&self) -> String {
        let mut s = format!("{{ {:?}:", self.common_props.name);
        for item in &self.children {
            s.push_str(&format!(" {}, ", item.to_string()));
        }
        s.push_str(&format!(" }}"));
        s
    }

    fn name(&self) -> &str {
        self.common_props.name
    }

    fn id(&self) -> usize {
        self.common_props.id
    }
}

impl FileSystemEntity for SymbolicLink<'_> {
    fn get(&self, name: &str) -> Option<&dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            None
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            None
        }
    }

    fn delete(&mut self) {
        println!("deleted symbolic link {} to the file {}", 
                 self.common_props.name, self.link.name());
    }

    fn delete_id(&mut self, _: usize) -> Result<usize, &str> {
        Err("Unsupported!")
    }

    fn to_string(&self) -> String {
        format!("{:?} -> {:?}", 
                self.common_props.name, 
                self.link.name()
               )
    }

    fn name(&self) -> &str {
        &self.common_props.name
    }

    fn id(&self) -> usize {
        self.common_props.id
    }
}

impl fmt::Debug for Folder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a> Folder<'a> {
    fn add<T>(&mut self, f: &'a mut T)
        where T: FileSystemEntity 
    {
        self.children.push(f);
    }
}

fn main() {
    let mut root =  Folder {
        common_props: FileSystemEntityProps {
            name: "/", 
            id: 1,
        },
        children: Vec::new() 
    };

    let mut f = File {
        common_props: FileSystemEntityProps {
            name: "file1",
            id: 2,
        }
    };
    root.add(&mut f);

    let mut fol2 = Folder {
        common_props: FileSystemEntityProps {
            name: "folder2/",
            id: 3,
        },
        children: Vec::new(),
    };
    let mut f2 = File {
        common_props: FileSystemEntityProps {
            name: "file2",
            id: 4,
        }
    };
    fol2.add(&mut f2);
    root.add(&mut fol2);

    let mut f3 = File {
        common_props: FileSystemEntityProps {
            name: "file3",
            id: 5,
        }
    };
    let mut fol3 = Folder {
        common_props: FileSystemEntityProps {
            name: "folder3/",
            id: 6,
        },
        children: vec![&mut f3],
    };
    root.add(&mut fol3);
    println!("root: {root:#?}");

    if let Some(x) = root.get("file1") {
        println!("Found file1: {}", x.to_string());
    } else {
        println!("Unable to find file1!");
    }

    // Test case #1
    // root.delete();
    // if let Some(x) = root.get("file1") {
    //     println!("Found file1: {}", x.to_string());
    // } else {
    //     println!("Unable to find file1!");
    // }
    // println!("root: {root:#?}");

    // Test case #2
    // root.delete_id(6).unwrap();
    // if let Some(x) = root.get("folder3/" {
    //     println!("Found folder3/: {}", x.to_string());
    // } else {
    //     println!("Unable to find folder3!");
    // }
    // println!("root: {root:#?}");

    // Test case #3
    // root.delete_id(2).unwrap();
    // if let Some(x) = root.get("file1".to_string()) {
    //     println!("Found file1: {}", x);
    // } else {
    //     println!("Unable to find file1!");
    // }
    // println!("root: {root:#?}");

    // Test case #4
    // if let Some(x) = root.get_mut("folder3/") {
    //     x.delete_id(5).unwrap();
    //     if let Some(y) = x.get("file3") {
    //         println!("Found file3: {}", y.to_string());
    //     } else {
    //         println!("Unable to find file3!");
    //     }
    // }
    // println!("root: {root:#?}");

    // // Test case #5 (not completed yet)
    // let mut s1 = SymbolicLink {
    //     common_props: FileSystemEntityProps {
    //         name: "link1",
    //         id: 7,
    //     },
    //     link: root.get("file1").unwrap(),
    // };
    // root.add(&mut s1);
    // println!("root: {root:#?}");
} 

