use std::collections::HashSet;

use ::beam_file::chunk::*;
use ::beam_file::StandardBeamFile;
use ::eyre::Report as AnyError;
use ::structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short, long = "chunk")]
    chunks: Vec<String>,

    #[structopt(name = "input-file")]
    input_files: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct PrettyBytes<'a>(&'a [u8]);

impl<'a> std::fmt::Display for PrettyBytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::pretty_hex::pretty_hex_write(f, &self.0)
    }
}

fn main() {
    run(StructOpt::from_args()).expect("Failure")
}

fn run(args: Args) -> Result<(), AnyError> {
    let chunks = args.chunks.iter().map(String::as_bytes).collect();
    for input_file in args.input_files {
        let () = dump_single_file(&input_file, &chunks)?;
    }

    Ok(())
}

fn dump_single_file(input_file: &str, chunks: &HashSet<&[u8]>) -> Result<(), AnyError> {
    println!("FILE: {}", input_file);
    let beam_file = StandardBeamFile::from_file(input_file)?;
    for chunk in beam_file.chunks {
        let chunk_id = &chunk.id()[..];
        if chunks.is_empty() || chunks.contains(chunk_id) {
            let () = dump_chunk(chunk)?;
        }
    }

    Ok(())
}

fn dump_chunk(chunk: StandardChunk) -> Result<(), AnyError> {
    let id = std::str::from_utf8(chunk.id())?;
    println!(" CHUNK:{}", id);
    match chunk {
        StandardChunk::Abst(abst_chunk) => dump_abst_chunk(abst_chunk)?,
        StandardChunk::Attr(attr_chunk) => dump_attr_chunk(attr_chunk)?,
        StandardChunk::Atom(atom_chunk) => dump_atom_chunk(atom_chunk)?,
        StandardChunk::CInf(cinf_chunk) => dump_cinf_chunk(cinf_chunk)?,
        StandardChunk::Code(code_chunk) => dump_code_chunk(code_chunk)?,
        StandardChunk::Dbgi(dbgi_chunk) => dump_dbgi_chunk(dbgi_chunk)?,
        StandardChunk::Docs(docs_chunk) => dump_docs_chunk(docs_chunk)?,
        StandardChunk::ExpT(expt_chunk) => dump_expt_chunk(expt_chunk)?,
        StandardChunk::FunT(funt_chunk) => dump_funt_chunk(funt_chunk)?,
        StandardChunk::ImpT(impt_chunk) => dump_impt_chunk(impt_chunk)?,
        StandardChunk::LitT(litt_chunk) => dump_litt_chunk(litt_chunk)?,
        StandardChunk::LocT(loct_chunk) => dump_loct_chunk(loct_chunk)?,
        StandardChunk::StrT(strt_chunk) => dump_strt_chunk(strt_chunk)?,

        StandardChunk::Unknown(raw_chunk) => dump_raw_chunk(raw_chunk)?,
    }
    Ok(())
}

fn dump_docs_chunk(docs_chunk: DocsChunk) -> Result<(), AnyError> {
    println!("  docs: {:#?}", docs_chunk.term);
    Ok(())
}

fn dump_funt_chunk(funt_chunk: FunTChunk) -> Result<(), AnyError> {
    println!("  functions: {:#?}", funt_chunk.functions);
    Ok(())
}

fn dump_litt_chunk(litt_chunk: LitTChunk) -> Result<(), AnyError> {
    println!("  literlas: {:#?}", litt_chunk.literals);
    Ok(())
}

fn dump_strt_chunk(strt_chunk: StrTChunk) -> Result<(), AnyError> {
    println!("  strings: {:#?}", strt_chunk.strings);
    Ok(())
}

fn dump_abst_chunk(abst_chunk: AbstChunk) -> Result<(), AnyError> {
    println!("  ast: {:#?}", abst_chunk.term);
    Ok(())
}

fn dump_atom_chunk(atom_chunk: AtomChunk) -> Result<(), AnyError> {
    println!("  atoms: {:#?}", atom_chunk.atoms);
    Ok(())
}

fn dump_expt_chunk(expt_chunk: ExpTChunk) -> Result<(), AnyError> {
    println!("  exports: {:#?}", expt_chunk.exports);
    Ok(())
}

fn dump_impt_chunk(impt_chunk: ImpTChunk) -> Result<(), AnyError> {
    println!("  imports: {:#?}", impt_chunk.imports);
    Ok(())
}

fn dump_loct_chunk(loct_chunk: LocTChunk) -> Result<(), AnyError> {
    println!("  locals: {:#?}", loct_chunk.locals);
    Ok(())
}

fn dump_cinf_chunk(cinf_chunk: CInfChunk) -> Result<(), AnyError> {
    println!("  info: {:#?}", cinf_chunk.term);
    Ok(())
}

fn dump_attr_chunk(attr_chunk: AttrChunk) -> Result<(), AnyError> {
    println!("  attributes: {:#?}", attr_chunk.term);
    Ok(())
}

fn dump_dbgi_chunk(dbgi_chunk: DbgiChunk) -> Result<(), AnyError> {
    println!("  debug-info: {:#?}", dbgi_chunk.term);
    Ok(())
}

fn dump_raw_chunk(raw_chunk: RawChunk) -> Result<(), AnyError> {
    println!("{}", PrettyBytes(&raw_chunk.data));
    Ok(())
}

fn dump_code_chunk(code_chunk: CodeChunk) -> Result<(), AnyError> {
    println!("  info-size: {}", code_chunk.info_size);
    println!("  version: {}", code_chunk.version);
    println!("  opcode_max: {}", code_chunk.opcode_max);
    println!("  label-count: {}", code_chunk.label_count);
    println!("  function-count: {}", code_chunk.function_count);

    println!("{}", PrettyBytes(&code_chunk.bytecode));

    Ok(())
}
