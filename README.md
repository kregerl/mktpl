# Make Template

Make template is a small utility program used for creating and saving template for files that are often reused.

## Usage

```
USAGE:
    mktpl [OPTIONS] [ARGS]

ARGS:
    <template_name>    
    <file_path>        [default: .]

OPTIONS:
    -c            Copy template to specified <file_path> after creation
    -h, --help    Print help information
    -l, --list    List possible templates
    -y            Assume yes to prompts
```

## Install

To install to program you'll need to have cargo installed.  
See `https://doc.rust-lang.org/cargo/getting-started/installation.html`  
Once cargo is installed, clone this repo and install using the following command:   
`cargo install --path mktpl`  
Where `mktpl` is the path to the cloned repo. 
