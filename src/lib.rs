use std::{
    error::Error,
};

mod analysis;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    analysis::count_symbols::print_follow_sets(args)
}



#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_output() {
        println!("Hello world, this test is a success!");
    }

    #[test]
    fn mongodb_basic_example() {
        extern crate mongodb;
        use mongodb::{Bson, bson, doc};
        use mongodb::{Client, ThreadedClient};
        use mongodb::db::ThreadedDatabase;
        use mongodb::coll::{options::UpdateOptions};
        let client = Client::connect("localhost", 27017)
            .expect("Failed to initialize standalone client.");

        let coll = client.db("test").collection("movies");

        let doc = doc! {
            "title": "Jaws",
            "array": [ 1, 2, 3 ],
        };

        let doc_filter = doc! {
            "title": { "$eq": "Jaws" }
        };

        let doc_update = doc! {
            "$set": { "director": "James"}
        };

        let mut update_options = UpdateOptions::new();
        update_options.upsert = Some(true);
        update_options.write_concern = None;

        // Insert/update document into 'test.movies' collection
        //coll.insert_one(doc!{ "title": "Back to the Future" }, None).unwrap();
        //coll.update_one(doc!{}, doc!{ "director": "Robert Zemeckis" }, None).unwrap();
        coll.update_one(
            /*filter: */ doc_filter.clone(),
            /*update: */ doc_update.clone(),
            /*options:*/ None
        )
            .ok().expect("Failed to insert/update document.");

        // Find the document and receive a cursor
        let mut cursor = coll.find(Some(doc.clone()), None)
            .ok().expect("Failed to execute find.");

        let item = cursor.next();

        // cursor.next() returns an Option<Result<Document>>
        match item {
            Some(Ok(doc)) => match doc.get("title") {
                Some(&Bson::String(ref title)) => println!("{}", title),
                _ => panic!("Expected title to be a string!"),
            },
            Some(Err(_)) => panic!("Failed to get next from server!"),
            None => panic!("Server returned no results!"),
        }
    }

}