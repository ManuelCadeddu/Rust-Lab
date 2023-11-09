// Import clap traits
use clap::{
            Parser,         // Per convertire gli argomenti in strutture dati
            /*
                Args,           // Per gestire la logica di gestione degli argomenti
                FromArgMatches, // Per convertire gli argomenti in istanze di strutture dati
                CommandFactory, // Crea un Command rilevante per un container definito dall'utente
                Subcommand,     // Per definire sottocomandi:
                                // comando = azione principale,
                                // sottocomando = funzionalità secondaria,
                                // argomento = parametro obbligatorio,
                                // opzione = parametro facoltativo preceduto da trattino
                ValueEnum       // Per definire un'enumerazione di valori validi per un'opzione
             */
};

#[derive(Debug, Parser)]    // Automatic generation of the implementation of the specified traits
// Metadata (no output)
#[command(
    author = "Simone, Femaf & Manuel_della_mensa",
    version = "1.0",
    about = "Leggi stringa da linea di comando",
    long_about = None
)]
pub struct SlugiArgs {
    /// Input string to parse
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,
}


/*
use clap::Parser;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}*/