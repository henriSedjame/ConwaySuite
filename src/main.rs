
/// La suite de Conway est une suite mathématique inventée en 1986 par le mathématicien John Horton Conway,
/// initialement sous le nom de « suite audioactive »1. Elle est également connue sous le nom anglais de Look and Say (« regarde et dis »).
/// Dans cette suite, un terme se détermine en annonçant les chiffres formant le terme précédent.


/// Count est la représentation d'un terme.
///
/// Par exemple `1 1` donnerait un `Count { label : Some("1"), count: 2}`
/// parce que le libellé `1` apparait 2 fois
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


/// Cette méthode permet de lire une ligne, et donc de construire la prochaine
/// Le principe est de stocker les counts dans une liste et de les afficher à la fin
fn next_line(line: String) -> String {

    const SPACE: &'static str = " ";

    // La liste des counts est initialiée
    let mut counts: Vec<Count> = vec![];

    // Le Count courant est initialié
    let mut current_count: Count = Count::default();

    // On itère sur les caractères de la ligne à lire
    let parts = line.split(SPACE).collect::<Vec<&str>>();

    parts.clone().into_iter().enumerate().for_each(|(n, i)|{


        match current_count.label.clone() {
            // Si le `label` du current_count est absent (ce qui signifie qu'il a encore sa valeur par défaut)
            None => {

                // On change la valeur du current_count avec comme `label` le caractère courant
                current_count = Count::new(i.to_string());
            }

            // Si le current_count a deja un `label`
            Some(l) => {

                // Si le label du current_count est égale au caractère courant
                if l == i.to_string() {

                    // On incrémente le count du label courant
                    current_count.inc();
                }
                // Si le caractère courant est différent du `label` du current_count
                else {
                    // On ajoute le current_count à la liste de counts
                    counts.push(current_count.clone());

                    // avant de reinitilaiser le current_count avec un nouveau label
                    current_count = Count::new(i.to_string());
                }
            }
        }

        // Si on est à la dernière itération
        if n == (parts.len() - 1) {

            // On ajoute le current_count à la liste de counts
            counts.push(current_count.clone());
        }

    });

    // On retourne enfin la jointure de tous les counts formattés
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

#[cfg(test)]
mod conway_suite_test {
    use crate::next_line;

    #[test]
    fn should_pass() {
        assert_eq!(String::from("1 1"), next_line(String::from("1")));
        assert_eq!(String::from("2 1"), next_line(String::from("1 1")));
        assert_eq!(String::from("1 2 1 1"), next_line(String::from("2 1")));
        assert_eq!(String::from("1 1 1 2 2 1"), next_line(String::from("1 2 1 1")));
        assert_eq!(String::from("3 1 2 2 1 1"), next_line(String::from("1 1 1 2 2 1")));
        assert_eq!(String::from("1 3 1 1 2 2 2 1"), next_line(String::from("3 1 2 2 1 1")));
    }

}