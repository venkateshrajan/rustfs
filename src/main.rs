use std::fmt;

#[derive(Debug)]
struct FileSystemEntityProps {
    name: String,
}

#[derive(Debug)]
struct File {
    common_props: FileSystemEntityProps
}

struct Folder<'a> {
    common_props: FileSystemEntityProps,
    children: Vec<&'a mut dyn FileSystemEntity>,
}

impl fmt::Debug for Folder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

trait FileSystemEntity {
    fn get(&self, name: String) -> Option<&dyn FileSystemEntity>;
    fn delete(&mut self);
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
} 

impl FileSystemEntity for File {
    fn get(&self, name: String) -> Option<&dyn FileSystemEntity> {
        if self.common_props.name == name {
            Some(self)
        } else {
            None
        }
    }

    fn delete(&mut self) {
        println!("deleted file {}", self.common_props.name);
    }

    fn to_string(&self) -> String{
        String::from(format!("{:?}", self.common_props.name))
    }

    fn name(&self) -> &str {
        &self.common_props.name
    }
}

impl FileSystemEntity for Folder<'_> {
    fn get(&self, name: String) -> Option<&dyn FileSystemEntity> {
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

    fn delete(&mut self) {
        while !self.children.is_empty() {
            let x: &mut dyn FileSystemEntity = self.children.pop().unwrap();
            x.delete();
        }
        println!("deleted folder {}", self.common_props.name);
    }

    fn to_string(&self) -> String{
        let mut s = format!("{{ {}:", self.common_props.name);
        for item in &self.children {
            s.push_str(&format!(" {}, ", item.to_string()));
        }
        s.push_str(&format!(" }}"));
        s
    }

    fn name(&self) -> &str {
        &self.common_props.name
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
            name: String::from("/"), 
        },
        children: Vec::new() 
    };

    let mut f = File {
        common_props: FileSystemEntityProps {
            name: String::from("file1"),
        }
    };
    root.add(&mut f);

    let mut fol2 = Folder {
        common_props: FileSystemEntityProps {
            name: String::from("folder2/"),
        },
        children: Vec::new(),
    };
    let mut f2 = File {
        common_props: FileSystemEntityProps {
            name: String::from("file2"),
        }
    };
    fol2.add(&mut f2);
    root.add(&mut fol2);

    let mut f3 = File {
        common_props: FileSystemEntityProps {
            name: String::from("file3"),
        }
    };
    let mut fol3 = Folder {
        common_props: FileSystemEntityProps {
            name: String::from("folder3/"),
        },
        children: vec![&mut f3],
    };
    root.add(&mut fol3);
    println!("root: {root:#?}");

    if let Some(x) = root.get("file1".to_string()) {
        println!("Found file1: {}", x.to_string());
    } else {
        println!("Unable to find file1!");
    }

    root.delete();
    if let Some(x) = root.get("file1".to_string()) {
        println!("Found file1: {}", x.to_string());
    } else {
        println!("Unable to find file1!");
    }
} 

