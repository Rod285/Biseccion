/*
**Autor: Mejia Velazquez Jose Rodrigo
**Matricula: 2183011914
**Materia: Metodos Numericos
**Profesor: Preenja Sagar Robin
**Proyecto final: Metodo de Biseccion
**Lenguaje utilizado: Rust
*/
use std::io;

const LIM: f64 = 0.000000000001;
const INTERVALO: f64 = 0.5 - LIM;
const EPSILON: f64 = 0.0000000000000001;

fn main() {
    let mut linf:String = String::new();
    let mut lsup:String = String::new();

    println!("Introduzca límite inferior");
    io::stdin().read_line(&mut linf).expect("Error al leer la línea");
    let linf: f64 = linf.trim().parse().expect("El valor introducido no es un número");
    println!("Introduzca límite superior");
    io::stdin().read_line(&mut lsup).expect("Error al leer la línea");
    let lsup: f64 = lsup.trim().parse().expect("El valor introducido no es un número");

    let mut linfaux = linf;
    let mut lsupaux = linf + INTERVALO;
    let mut raiz: f64;
    
    while lsupaux <= lsup {
        let x:f64 = lsupaux + LIM;
        if x.abs() < EPSILON {
            lsupaux = 0.0;
        }
        
        raiz = biseccion(linfaux, lsupaux);

        if lsupaux - raiz > LIM || lsupaux - raiz == 0.0 {
            if raiz >= 0.0 {println!("raiz:  {raiz}")} else {println!("raiz: {raiz}")};
        }

        linfaux = lsupaux + LIM;
        lsupaux = linfaux + INTERVALO;
    }
}

fn biseccion(mut li: f64, mut ls: f64) -> f64 {
    let mut pm:f64;
    pm = (li+ls)/2.0;
    while (ls-li)/2.0 > LIM {
        if func(li) == 0.0 {
            pm = li;
            break;
        } else if func(ls) == 0.0 {
            pm = ls;
            break;
        } else if func(pm) * func(li) < 0.0 {
            ls = pm;
        } else {
            li = pm;
        }
        pm = (li + ls)/2.0;
    }
    pm
}

fn func(num: f64) -> f64 {num.sin()}