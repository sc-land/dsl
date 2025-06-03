use sc_dsl::dsl::parser::tree::Tree;
use sc_dsl::dsl::ast::sc::SC;
use sc_dsl::dsl::ast::fly::Fly;
use sc_dsl::dsl::ast::strand::Strand;
use sc_dsl::dsl::ast::genome::Genome;
use sc_dsl::dsl::ast::anatomy::Anatomy;
use sc_dsl::dsl::ast::behavior::Behavior;

#[test]
fn test_complete_parsing_flow() {
    // Testa o fluxo completo de parsing com um programa válido
    let input = r#"
    bug TestBug
        gene x Int
        ethics test(a: Int) Int
            a.plus(1)
        end
    end
    "#.to_string();

    let tree = Tree::parse_input(input.clone()).expect("Should parse successfully");
    let sc = tree.get_sc();

    // Verifica a estrutura completa
    assert!(matches!(sc.fly.strand.genome[0], Genome::Anatomy(_)));

    if let Genome::Anatomy(anatomy) = &sc.fly.strand.genome[0] {
        if let Anatomy::Bug(bug) = anatomy {
            assert_eq!(bug.specie, "TestBug");
            assert_eq!(bug.genes.len(), 1);
            assert_eq!(bug.ethics.len(), 1);
        }
    }
}

#[test]
fn test_multiple_bugs_parsing() {
    // Testa parsing de múltiplos bugs
    let input = r#"
    bug FirstBug
        gene x Int
        ethics test Int
            x.toInt()
        end
    end

    bug SecondBug
        gene y String
        ethics hello(a: String) String
            a.toString()
        end
    end
    "#.to_string();

    let tree = Tree::parse_input(input.clone()).expect("Should parse successfully");
    let sc = tree.get_sc();

    // Verifica que temos dois bugs
    assert_eq!(sc.fly.strand.genome.len(), 2);

    // Verifica o primeiro bug
    if let Genome::Anatomy(anatomy) = &sc.fly.strand.genome[0] {
        if let Anatomy::Bug(bug) = anatomy {
            assert_eq!(bug.specie, "FirstBug");
            assert_eq!(bug.genes.len(), 1);
            assert_eq!(bug.ethics.len(), 1);
        }
    }

    // Verifica o segundo bug
    if let Genome::Anatomy(anatomy) = &sc.fly.strand.genome[1] {
        if let Anatomy::Bug(bug) = anatomy {
            assert_eq!(bug.specie, "SecondBug");
            assert_eq!(bug.genes.len(), 1);
            assert_eq!(bug.ethics.len(), 1);
        }
    }
}

#[test]
fn test_complex_ethics_parsing() {
    // Testa parsing de ethics complexos
    let input = r#"
    bug ComplexBug
        gene x Int
        ethics test(a: Int, b: String) Int
            if a.greaterThan(0)
                a
            else
                0.toInt()
            end
        end
    end
    "#.to_string();

    let tree = Tree::parse_input(input.clone()).expect("Should parse successfully");
    let sc = tree.get_sc();

    if let Genome::Anatomy(anatomy) = &sc.fly.strand.genome[0] {
        if let Anatomy::Bug(bug) = anatomy {
            assert_eq!(bug.ethics.len(), 1);
            let ethics = &bug.ethics[0];
            assert!(ethics.signature.is_some());
            assert!(ethics.feedback.is_some());
            assert!(ethics.body.is_some());
        }
    }
}

#[test]
fn test_nested_structures() {
    // Testa estruturas aninhadas (if dentro de ethics)
    let input = r#"
    bug NestedBug
        gene x Int
        ethics test(a: Int) Int
            if a.greaterThan(0)
                if a.lessThan(10)
                    a
                else
                    10.toInt()
                end
            else
                0.toInt()
            end
        end
    end
    "#.to_string();

    let tree = Tree::parse_input(input.clone()).expect("Should parse successfully");
    let sc = tree.get_sc();

    if let Genome::Anatomy(anatomy) = &sc.fly.strand.genome[0] {
        if let Anatomy::Bug(bug) = anatomy {
            assert_eq!(bug.ethics.len(), 1);
            let ethics = &bug.ethics[0];
            assert!(ethics.body.is_some());
        }
    }
}
