# SEG-Y Parser

SEG-Y Parser is a Rust-based tool designed to parse and analyze SEG-Y seismic data files. It provides a straightforward way to extract EBCDIC and binary headers, as well as individual trace information from SEG-Y files. The software is lightweight, efficient, and built to handle large seismic datasets.





## Example usage

```rs
use sgy_rs::read_segy_from_file;
use sgy_rs::errors::SegyError;

fn main() -> Result<(), SegyError> {
 
    let path = "";

 
    let segy_file = read_segy_from_file(path)?;

  
    Ok(())
}
``` 

## Contributing

We welcome contributions to this project. To contribute:
1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit your changes: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-name`.
5. Open a pull request.

## License

This project is licensed under the MIT License. 

## Contact

For questions or feedback, feel free to open an issue.

