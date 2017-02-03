# rust-perfcnt-reader
A usage example of the rust-perfcnt library. The application receives in input by the user either the process pid to monitor or directly the path to the process to start and monitor. 
Basic counters are reported: . Additional ones will come soon.

## Requirements
Of course, you will need Rust installed. If you haven't already, get it here: [rust-lang.org](https://www.rust-lang.org). Also you need [Cargo](https://crates.io) to easily compile. The rustc compiler version required is the 1.15.0-nightly.


## Usage

1. Clone the [source] with `git`:

   ```sh
   $ git clone https://github.com/dzobbe/rust-perfcnt-reader.git
   $ cd rust-perfcnt-reader
   ```
2. Build:

     ```sh
    $ sudo cargo build
    ```

3. Run the application by passing through command line either the process pid or the application path to monitor:

    ```sh
    $ ./target/debug/pcm-reader --pid=3245
    ```
  or
  
    ```sh
    $ ./target/debug/pcm-reader --app="/path/to/something/memcached" --args2app="-l 127.0.0.1 -p 1234"
    ```
    
## Example
    
