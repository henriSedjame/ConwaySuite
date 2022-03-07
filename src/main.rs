

#[derive(Clone, Debug)]
struct Count {
    label : Option<String>,
    count: u8,
}

impl Count {

    fn new(label: String) -> Self {
        Self {
            label: Some(label),
            count: 1
        }
    }

    fn inc(&mut self) {
        self.count += 1;
    }
}

impl ToString for Count {
    fn to_string(&self) -> String {
        format!("{} {}", self.count, self.label.clone().unwrap())
    }
}

impl Default for Count {
    fn default() -> Self {
        Self{
            label: None,
            count: 0
        }
    }
}


fn main() {
    conway_suite(5);
}

fn next_line(line: String) -> String {

    let mut current_count: Count = Count::default();
    let mut counts: Vec<Count> = vec![];

    const SPACE: &'static str = " ";

    let parts = line.split(SPACE).collect::<Vec<&str>>();

    parts.clone().into_iter().enumerate().for_each(|(n, i)|{

        match current_count.label.clone() {
            None => {
                current_count = Count::new(i.to_string());
            }
            Some(l) => {
                if l == i.to_string() {
                    current_count.inc();
                } else {
                    counts.push(current_count.clone());
                    current_count = Count::new(i.to_string());
                }
            }
        }

        if n == (parts.len() - 1) {
            counts.push(current_count.clone());
        }

    });

   counts.into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>().join(SPACE)
}

fn conway_suite(nb: u32) {

    let mut res = "1".to_string();

    (0..nb).for_each(|_|{

        println!("{}", res);

        res = next_line(res.to_string());

    })
}