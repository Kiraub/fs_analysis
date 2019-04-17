use std::{
    error::Error,
    fs::{
        File,
    },
    io::{
        //stdin,
        BufReader,
        prelude::*,
        //stdout,
        //Write,
    },
    collections::{
        HashMap,
    },
};

const TESTFILE : &str = ".\\output\\data.txt";
const IGNORESTRING : &str = " -\n";

pub fn print_follow_sets(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let filepath;
    let ignore;
    let mut args = args.iter();
    print!("{}",args.next().unwrap());
    if let Some(pathstr) = args.next() {
        print!(" {}", pathstr);
        filepath = pathstr.clone();
    } else {
        filepath = String::from(TESTFILE);
    }
    if let Some(ignorestr) = args.next() {
        print!(" {}", ignorestr);
        ignore = ignorestr.clone();
    } else {
        ignore = String::from(IGNORESTRING);
    }
    println!("\nOutput:");
    let filehandle = File::open(filepath)?;
    let mut buf_reader = BufReader::new(filehandle);
    let mut buffer = String::new();
    let mut last_char : Option<char> = None;
    let mut pre_last_char : Option<char> = None;
    // HM<c1, (u1, HM<c2, (u2, HM<c3, u3> )> )>
    let mut letter_counter : HashMap<char, (usize, HashMap<char, (usize, HashMap<char, usize>)>)> = HashMap::new();
    while if let Ok(length) = buf_reader.read_line(&mut buffer) {
        let mut citer = buffer.chars();
        loop {
            if let Some(c) = citer.next() {
                if !ignore.contains(c) {
                    {
                        let ct = letter_counter.entry(c).or_insert((0, HashMap::new()));
                        ct.0 += 1;
                    }
                    if let Some(lc) = last_char {
                        {
                            // (u1, HM<c2, (u2, HM<c3, u3> )> )
                            let lct = letter_counter.entry(lc).or_insert((0, HashMap::new()));
                            // (u2, HM<c3, u3> )
                            let flct = lct.1.entry(c).or_insert((0, HashMap::new()));
                            // u2
                            flct.0 += 1;
                        }
                        if let Some(plc) = pre_last_char {
                            // (u1, HM<c2, (u2, HM<c3, u3> )> )
                            let plct = letter_counter.entry(plc).or_insert((0, HashMap::new()));
                            // (u2, HM<c3, u3> )
                            let fplct = plct.1.entry(lc).or_insert((0, HashMap::new()));
                            // u3
                            let ffplct = fplct.1.entry(c).or_insert(0);
                            *ffplct += 1;
                        }
                        pre_last_char = Some(lc);
                    }
                    last_char = Some(c);
                }
            } else {
                break;
            }
        }
        buffer = String::new();
        if length>0{
            true
        } else {
            false
        }
    } else {
        false
    }{}
    for (k,(v, hm)) in letter_counter.iter() {
        buffer = String::new();
        buffer.push_str(&format!("C<{}>CC<{}>F:", k, v));
        for (fk, (fv, fhm)) in hm.iter() {
            buffer.push_str(&format!("FC<{}>FCC<{}>FF:", fk, fv));
            for (ffk, ffv) in fhm.iter() {
                buffer.push_str(&format!("FFC<{}>FFCC<{}>;", ffk, ffv));
            }
        }
        println!("{}", buffer);
    }
    Ok(())
}
